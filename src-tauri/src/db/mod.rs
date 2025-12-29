pub mod models;

use rusqlite::{Connection, Result};

/// Initialize the database with required tables
pub fn init_db(db_path: &str) -> Result<()> {
    let conn = Connection::open(db_path)?;
    
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS papers (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            title_ja TEXT,
            abstract TEXT,
            summary_ja TEXT,
            url_pdf TEXT,
            url_paper TEXT,
            published TEXT,
            fetched_at TEXT DEFAULT (datetime('now'))
        );

        CREATE TABLE IF NOT EXISTS paper_tasks (
            paper_id TEXT NOT NULL,
            task_slug TEXT NOT NULL,
            category TEXT,
            PRIMARY KEY (paper_id, task_slug),
            FOREIGN KEY (paper_id) REFERENCES papers(id) ON DELETE CASCADE
        );

        CREATE INDEX IF NOT EXISTS idx_papers_published ON papers(published DESC);
        CREATE INDEX IF NOT EXISTS idx_paper_tasks_category ON paper_tasks(category);
        CREATE INDEX IF NOT EXISTS idx_paper_tasks_slug ON paper_tasks(task_slug);
        "
    )?;
    
    // Migration: Add title_ja column if it doesn't exist (for existing databases)
    let _ = conn.execute("ALTER TABLE papers ADD COLUMN title_ja TEXT", []);
    
    Ok(())
}

/// Get a database connection
pub fn get_connection(db_path: &str) -> Result<Connection> {
    Connection::open(db_path)
}

/// Get papers from database, optionally filtered by category
pub fn get_papers_from_db(
    db_path: &str,
    category: Option<&str>,
    limit: Option<i64>,
) -> Result<Vec<models::Paper>> {
    let conn = get_connection(db_path)?;
    
    let limit_val = limit.unwrap_or(50);
    
    let papers = if let Some(cat) = category {
        let mut stmt = conn.prepare(
            "SELECT DISTINCT p.id, p.title, p.title_ja, p.abstract, p.summary_ja, p.url_pdf, p.url_paper, p.published, p.fetched_at
             FROM papers p
             JOIN paper_tasks pt ON p.id = pt.paper_id
             WHERE pt.category = ?1
             ORDER BY p.published DESC
             LIMIT ?2"
        )?;
        
        let paper_iter = stmt.query_map([cat, &limit_val.to_string()], |row| {
            Ok(models::Paper {
                id: row.get(0)?,
                title: row.get(1)?,
                title_ja: row.get(2)?,
                r#abstract: row.get(3)?,
                summary_ja: row.get(4)?,
                url_pdf: row.get(5)?,
                url_paper: row.get(6)?,
                published: row.get(7)?,
                fetched_at: row.get(8)?,
                tasks: vec![],
            })
        })?;
        
        paper_iter.collect::<Result<Vec<_>>>()?
    } else {
        let mut stmt = conn.prepare(
            "SELECT id, title, title_ja, abstract, summary_ja, url_pdf, url_paper, published, fetched_at
             FROM papers
             ORDER BY published DESC
             LIMIT ?1"
        )?;
        
        let paper_iter = stmt.query_map([limit_val], |row| {
            Ok(models::Paper {
                id: row.get(0)?,
                title: row.get(1)?,
                title_ja: row.get(2)?,
                r#abstract: row.get(3)?,
                summary_ja: row.get(4)?,
                url_pdf: row.get(5)?,
                url_paper: row.get(6)?,
                published: row.get(7)?,
                fetched_at: row.get(8)?,
                tasks: vec![],
            })
        })?;
        
        paper_iter.collect::<Result<Vec<_>>>()?
    };
    
    // Load tasks for each paper
    let mut papers_with_tasks = papers;
    for paper in &mut papers_with_tasks {
        paper.tasks = get_tasks_for_paper(&conn, &paper.id)?;
    }
    
    Ok(papers_with_tasks)
}

fn get_tasks_for_paper(conn: &Connection, paper_id: &str) -> Result<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT task_slug FROM paper_tasks WHERE paper_id = ?1"
    )?;
    
    let tasks = stmt.query_map([paper_id], |row| row.get(0))?
        .collect::<Result<Vec<String>>>()?;
    
    Ok(tasks)
}

/// Insert or update a paper
pub fn upsert_paper(conn: &Connection, paper: &models::Paper) -> Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO papers (id, title, title_ja, abstract, summary_ja, url_pdf, url_paper, published, fetched_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, datetime('now'))",
        (
            &paper.id,
            &paper.title,
            &paper.title_ja,
            &paper.r#abstract,
            &paper.summary_ja,
            &paper.url_pdf,
            &paper.url_paper,
            &paper.published,
        ),
    )?;
    Ok(())
}

/// Insert paper-task relationship
pub fn insert_paper_task(conn: &Connection, paper_id: &str, task_slug: &str, category: &str) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO paper_tasks (paper_id, task_slug, category) VALUES (?1, ?2, ?3)",
        (paper_id, task_slug, category),
    )?;
    Ok(())
}

/// Update paper summary
pub fn update_paper_summary(db_path: &str, paper_id: &str, summary: &str) -> Result<()> {
    let conn = get_connection(db_path)?;
    conn.execute(
        "UPDATE papers SET summary_ja = ?1 WHERE id = ?2",
        (summary, paper_id),
    )?;
    Ok(())
}

/// Update paper Japanese title
pub fn update_paper_title_ja(conn: &Connection, paper_id: &str, title_ja: &str) -> Result<()> {
    conn.execute(
        "UPDATE papers SET title_ja = ?1 WHERE id = ?2",
        (title_ja, paper_id),
    )?;
    Ok(())
}

/// Get a single paper by ID
pub fn get_paper_by_id(db_path: &str, paper_id: &str) -> Result<Option<models::Paper>> {
    let conn = get_connection(db_path)?;
    
    let mut stmt = conn.prepare(
        "SELECT id, title, title_ja, abstract, summary_ja, url_pdf, url_paper, published, fetched_at
         FROM papers WHERE id = ?1"
    )?;
    
    let mut paper_iter = stmt.query_map([paper_id], |row| {
        Ok(models::Paper {
            id: row.get(0)?,
            title: row.get(1)?,
            title_ja: row.get(2)?,
            r#abstract: row.get(3)?,
            summary_ja: row.get(4)?,
            url_pdf: row.get(5)?,
            url_paper: row.get(6)?,
            published: row.get(7)?,
            fetched_at: row.get(8)?,
            tasks: vec![],
        })
    })?;
    
    if let Some(paper_result) = paper_iter.next() {
        let mut paper = paper_result?;
        paper.tasks = get_tasks_for_paper(&conn, &paper.id)?;
        Ok(Some(paper))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::atomic::{AtomicU64, Ordering};
    
    static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);
    
    fn create_test_db(test_name: &str) -> String {
        let counter = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
        let db_path = format!("/tmp/test_paper_news_{}_{}.db", test_name, counter);
        // Remove if exists
        let _ = fs::remove_file(&db_path);
        init_db(&db_path).unwrap();
        db_path
    }
    
    fn cleanup_test_db(db_path: &str) {
        let _ = fs::remove_file(db_path);
    }
    
    #[test]
    fn test_init_db() {
        let db_path = create_test_db("init");
        // If we got here, init_db succeeded
        cleanup_test_db(&db_path);
    }
    
    #[test]
    fn test_upsert_and_get_paper() {
        let db_path = create_test_db("upsert");
        let conn = get_connection(&db_path).unwrap();
        
        let paper = models::Paper {
            id: "test-123".to_string(),
            title: "Test Paper".to_string(),
            title_ja: Some("テスト論文".to_string()),
            r#abstract: Some("This is a test abstract".to_string()),
            summary_ja: None,
            url_pdf: Some("https://example.com/paper.pdf".to_string()),
            url_paper: Some("https://example.com/paper".to_string()),
            published: Some("2024-01-01".to_string()),
            fetched_at: None,
            tasks: vec![],
        };
        
        upsert_paper(&conn, &paper).unwrap();
        
        let retrieved = get_paper_by_id(&db_path, "test-123").unwrap();
        assert!(retrieved.is_some());
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.id, "test-123");
        assert_eq!(retrieved.title, "Test Paper");
        assert_eq!(retrieved.title_ja, Some("テスト論文".to_string()));
        
        cleanup_test_db(&db_path);
    }
    
    #[test]
    fn test_paper_tasks() {
        let db_path = create_test_db("tasks");
        let conn = get_connection(&db_path).unwrap();
        
        let paper = models::Paper {
            id: "test-456".to_string(),
            title: "ML Paper".to_string(),
            title_ja: None,
            r#abstract: None,
            summary_ja: None,
            url_pdf: None,
            url_paper: None,
            published: None,
            fetched_at: None,
            tasks: vec![],
        };
        
        upsert_paper(&conn, &paper).unwrap();
        insert_paper_task(&conn, "test-456", "machine-learning", "AI全般").unwrap();
        insert_paper_task(&conn, "test-456", "deep-learning", "AI全般").unwrap();
        
        let papers = get_papers_from_db(&db_path, Some("AI全般"), Some(10)).unwrap();
        assert_eq!(papers.len(), 1);
        assert_eq!(papers[0].tasks.len(), 2);
        
        cleanup_test_db(&db_path);
    }
}

