//! RFC Database Queries

use rusqlite::{Connection, Result, params};
use super::models::{Rfc, RfcFilter, RfcBookmark, RfcHistory, RfcListResponse};

/// Insert or update an RFC
pub fn upsert_rfc(conn: &Connection, rfc: &Rfc) -> Result<()> {
    let authors_json = serde_json::to_string(&rfc.authors).unwrap_or_default();
    let keywords_json = serde_json::to_string(&rfc.keywords).unwrap_or_default();
    
    conn.execute(
        "INSERT INTO rfcs (id, number, title, abstract, status, published_date, authors, keywords, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, datetime('now'))
         ON CONFLICT(id) DO UPDATE SET
            title = excluded.title,
            abstract = excluded.abstract,
            status = excluded.status,
            published_date = excluded.published_date,
            authors = excluded.authors,
            keywords = excluded.keywords,
            updated_at = datetime('now')",
        params![
            &rfc.id,
            rfc.number,
            &rfc.title,
            &rfc.r#abstract,
            &rfc.status,
            &rfc.published_date,
            &authors_json,
            &keywords_json,
        ],
    )?;
    Ok(())
}

/// Insert RFC category relationship
pub fn insert_rfc_category(conn: &Connection, rfc_id: &str, category: &str) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO rfc_categories (rfc_id, category) VALUES (?1, ?2)",
        params![rfc_id, category],
    )?;
    Ok(())
}

/// Get RFCs with filtering and pagination
pub fn get_rfcs(
    conn: &Connection,
    filter: Option<&RfcFilter>,
    page: i32,
    limit: i32,
) -> Result<RfcListResponse> {
    let offset = (page - 1) * limit;
    
    // Build WHERE clause
    let mut conditions: Vec<String> = Vec::new();
    let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
    
    if let Some(f) = filter {
        if let Some(ref search) = f.search {
            conditions.push("(title LIKE ?1 OR abstract LIKE ?1)".to_string());
            params_vec.push(Box::new(format!("%{}%", search)));
        }
        
        if let Some(num) = f.rfc_number {
            let idx = params_vec.len() + 1;
            conditions.push(format!("number = ?{}", idx));
            params_vec.push(Box::new(num));
        }
        
        if let Some(ref statuses) = f.status {
            if !statuses.is_empty() {
                let placeholders: Vec<String> = statuses.iter().enumerate()
                    .map(|(i, _)| format!("?{}", params_vec.len() + i + 1))
                    .collect();
                conditions.push(format!("status IN ({})", placeholders.join(", ")));
                for s in statuses {
                    params_vec.push(Box::new(s.clone()));
                }
            }
        }
        
        if let Some(year_from) = f.year_from {
            let idx = params_vec.len() + 1;
            conditions.push(format!("CAST(substr(published_date, 1, 4) AS INTEGER) >= ?{}", idx));
            params_vec.push(Box::new(year_from));
        }
        
        if let Some(year_to) = f.year_to {
            let idx = params_vec.len() + 1;
            conditions.push(format!("CAST(substr(published_date, 1, 4) AS INTEGER) <= ?{}", idx));
            params_vec.push(Box::new(year_to));
        }
    }
    
    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };
    
    // Get total count
    let count_sql = format!("SELECT COUNT(*) FROM rfcs {}", where_clause);
    let total: i64 = conn.query_row(&count_sql, rusqlite::params_from_iter(params_vec.iter().map(|p| p.as_ref())), |row| row.get(0))?;
    
    // Get RFCs
    let limit_idx = params_vec.len() + 1;
    let offset_idx = params_vec.len() + 2;
    params_vec.push(Box::new(limit));
    params_vec.push(Box::new(offset));
    
    let sql = format!(
        "SELECT id, number, title, abstract, status, published_date, authors, keywords,
                summary_easy, summary_normal, summary_technical, implementation_guide,
                title_ja, abstract_ja
         FROM rfcs {}
         ORDER BY number DESC
         LIMIT ?{} OFFSET ?{}",
        where_clause, limit_idx, offset_idx
    );
    
    let mut stmt = conn.prepare(&sql)?;
    let params_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();
    
    let rfc_iter = stmt.query_map(rusqlite::params_from_iter(params_refs), |row| {
        let authors_json: String = row.get(6)?;
        let keywords_json: String = row.get(7)?;
        let authors: Vec<String> = serde_json::from_str(&authors_json).unwrap_or_default();
        let keywords: Vec<String> = serde_json::from_str(&keywords_json).unwrap_or_default();
        let id: String = row.get(0)?;
        
        Ok(Rfc {
            id: id.clone(),
            number: row.get(1)?,
            title: row.get(2)?,
            r#abstract: row.get(3)?,
            status: row.get::<_, Option<String>>(4)?.unwrap_or_default(),
            published_date: row.get(5)?,
            authors,
            keywords,
            categories: vec![], // Will be filled later
            summary_easy: row.get(8)?,
            summary_normal: row.get(9)?,
            summary_technical: row.get(10)?,
            implementation_guide: row.get(11)?,
            title_ja: row.get(12)?,
            abstract_ja: row.get(13)?,
            is_bookmarked: false, // Will be filled later
        })
    })?;
    
    let mut rfcs: Vec<Rfc> = rfc_iter.collect::<Result<Vec<_>>>()?;
    
    // Load categories and bookmark status for each RFC
    for rfc in &mut rfcs {
        rfc.categories = get_categories_for_rfc(conn, &rfc.id)?;
        rfc.is_bookmarked = is_rfc_bookmarked(conn, &rfc.id)?;
    }
    
    // Filter by categories if specified
    if let Some(f) = filter {
        if let Some(ref cats) = f.categories {
            if !cats.is_empty() {
                rfcs.retain(|rfc| rfc.categories.iter().any(|c| cats.contains(c)));
            }
        }
    }
    
    Ok(RfcListResponse {
        rfcs,
        total,
        page,
        limit,
    })
}

/// Get a single RFC by ID
pub fn get_rfc_by_id(conn: &Connection, rfc_id: &str) -> Result<Option<Rfc>> {
    let mut stmt = conn.prepare(
        "SELECT id, number, title, abstract, status, published_date, authors, keywords,
                summary_easy, summary_normal, summary_technical, implementation_guide,
                title_ja, abstract_ja
         FROM rfcs WHERE id = ?1"
    )?;
    
    let mut rfc_iter = stmt.query_map([rfc_id], |row| {
        let authors_json: String = row.get(6)?;
        let keywords_json: String = row.get(7)?;
        let authors: Vec<String> = serde_json::from_str(&authors_json).unwrap_or_default();
        let keywords: Vec<String> = serde_json::from_str(&keywords_json).unwrap_or_default();
        
        Ok(Rfc {
            id: row.get(0)?,
            number: row.get(1)?,
            title: row.get(2)?,
            r#abstract: row.get(3)?,
            status: row.get::<_, Option<String>>(4)?.unwrap_or_default(),
            published_date: row.get(5)?,
            authors,
            keywords,
            categories: vec![],
            summary_easy: row.get(8)?,
            summary_normal: row.get(9)?,
            summary_technical: row.get(10)?,
            implementation_guide: row.get(11)?,
            title_ja: row.get(12)?,
            abstract_ja: row.get(13)?,
            is_bookmarked: false,
        })
    })?;
    
    if let Some(rfc_result) = rfc_iter.next() {
        let mut rfc = rfc_result?;
        rfc.categories = get_categories_for_rfc(conn, &rfc.id)?;
        rfc.is_bookmarked = is_rfc_bookmarked(conn, &rfc.id)?;
        Ok(Some(rfc))
    } else {
        Ok(None)
    }
}

/// Get categories for an RFC
fn get_categories_for_rfc(conn: &Connection, rfc_id: &str) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT category FROM rfc_categories WHERE rfc_id = ?1")?;
    let cats = stmt.query_map([rfc_id], |row| row.get(0))?
        .collect::<Result<Vec<String>>>()?;
    Ok(cats)
}

/// Check if RFC is bookmarked
fn is_rfc_bookmarked(conn: &Connection, rfc_id: &str) -> Result<bool> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM rfc_bookmarks WHERE rfc_id = ?1",
        [rfc_id],
        |row| row.get(0),
    )?;
    Ok(count > 0)
}

/// Update RFC summary (easy/normal/technical)
pub fn update_rfc_summary(
    conn: &Connection,
    rfc_id: &str,
    level: &str,
    summary: &str,
) -> Result<()> {
    let column = match level {
        "easy" => "summary_easy",
        "normal" => "summary_normal",
        "technical" => "summary_technical",
        _ => return Ok(()),
    };
    
    conn.execute(
        &format!("UPDATE rfcs SET {} = ?1, updated_at = datetime('now') WHERE id = ?2", column),
        params![summary, rfc_id],
    )?;
    Ok(())
}

/// Update RFC implementation guide
pub fn update_rfc_implementation_guide(conn: &Connection, rfc_id: &str, guide: &str) -> Result<()> {
    conn.execute(
        "UPDATE rfcs SET implementation_guide = ?1, updated_at = datetime('now') WHERE id = ?2",
        params![guide, rfc_id],
    )?;
    Ok(())
}

/// Update RFC Japanese abstract
pub fn update_rfc_abstract_ja(conn: &Connection, rfc_id: &str, abstract_ja: &str) -> Result<()> {
    conn.execute(
        "UPDATE rfcs SET abstract_ja = ?1, updated_at = datetime('now') WHERE id = ?2",
        params![abstract_ja, rfc_id],
    )?;
    Ok(())
}

/// Update RFC Japanese title
pub fn update_rfc_title_ja(conn: &Connection, rfc_id: &str, title_ja: &str) -> Result<()> {
    conn.execute(
        "UPDATE rfcs SET title_ja = ?1, updated_at = datetime('now') WHERE id = ?2",
        params![title_ja, rfc_id],
    )?;
    Ok(())
}


// ============================================================================
// Bookmark Operations
// ============================================================================

/// Add RFC bookmark
pub fn add_rfc_bookmark(conn: &Connection, rfc_id: &str, memo: Option<&str>) -> Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO rfc_bookmarks (rfc_id, memo, created_at) VALUES (?1, ?2, datetime('now'))",
        params![rfc_id, memo],
    )?;
    Ok(())
}

/// Remove RFC bookmark
pub fn remove_rfc_bookmark(conn: &Connection, rfc_id: &str) -> Result<()> {
    conn.execute("DELETE FROM rfc_bookmarks WHERE rfc_id = ?1", [rfc_id])?;
    Ok(())
}

/// Get all RFC bookmarks
pub fn get_rfc_bookmarks(conn: &Connection) -> Result<Vec<RfcBookmark>> {
    let mut stmt = conn.prepare(
        "SELECT rfc_id, memo, created_at FROM rfc_bookmarks ORDER BY created_at DESC"
    )?;
    
    let bookmarks = stmt.query_map([], |row| {
        Ok(RfcBookmark {
            rfc_id: row.get(0)?,
            memo: row.get(1)?,
            created_at: row.get(2)?,
        })
    })?.collect::<Result<Vec<_>>>()?;
    
    Ok(bookmarks)
}

// ============================================================================
// History Operations
// ============================================================================

/// Add RFC to history
pub fn add_rfc_history(conn: &Connection, rfc_id: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO rfc_history (rfc_id, viewed_at) VALUES (?1, datetime('now'))",
        [rfc_id],
    )?;
    Ok(())
}

/// Get RFC history
pub fn get_rfc_history(conn: &Connection, limit: Option<i32>) -> Result<Vec<RfcHistory>> {
    let limit_val = limit.unwrap_or(50);
    
    let mut stmt = conn.prepare(
        "SELECT DISTINCT rfc_id, MAX(viewed_at) as viewed_at
         FROM rfc_history
         GROUP BY rfc_id
         ORDER BY viewed_at DESC
         LIMIT ?1"
    )?;
    
    let history = stmt.query_map([limit_val], |row| {
        Ok(RfcHistory {
            rfc_id: row.get(0)?,
            viewed_at: row.get(1)?,
        })
    })?.collect::<Result<Vec<_>>>()?;
    
    Ok(history)
}

// ============================================================================
// Category Operations
// ============================================================================

/// Get all used categories
pub fn get_all_rfc_categories(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT DISTINCT category FROM rfc_categories ORDER BY category"
    )?;
    
    let categories = stmt.query_map([], |row| row.get(0))?
        .collect::<Result<Vec<String>>>()?;
    
    Ok(categories)
}

/// Get RFC count
pub fn get_rfc_count(conn: &Connection) -> Result<i64> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM rfcs",
        [],
        |row| row.get(0),
    )?;
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{init_db, get_connection};
    use std::fs;
    use std::sync::atomic::{AtomicU64, Ordering};
    
    static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);
    
    fn create_test_db(test_name: &str) -> (String, Connection) {
        let counter = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
        let db_path = format!("/tmp/test_rfc_{}_{}.db", test_name, counter);
        let _ = fs::remove_file(&db_path);
        init_db(&db_path).unwrap();
        let conn = get_connection(&db_path).unwrap();
        (db_path, conn)
    }
    
    fn cleanup_test_db(db_path: &str) {
        let _ = fs::remove_file(db_path);
    }
    
    #[test]
    fn test_upsert_and_get_rfc() {
        let (db_path, conn) = create_test_db("upsert");
        
        let rfc = Rfc {
            id: "RFC9114".to_string(),
            number: 9114,
            title: "HTTP/3".to_string(),
            r#abstract: Some("HTTP/3 is the third version...".to_string()),
            status: "PROPOSED STANDARD".to_string(),
            published_date: Some("2022-06".to_string()),
            authors: vec!["M. Bishop".to_string()],
            keywords: vec!["HTTP".to_string(), "QUIC".to_string()],
            categories: vec![],
            summary_easy: None,
            summary_normal: None,
            summary_technical: None,
            implementation_guide: None,
            title_ja: None,
            abstract_ja: None,
            is_bookmarked: false,
        };
        
        upsert_rfc(&conn, &rfc).unwrap();
        insert_rfc_category(&conn, "RFC9114", "http").unwrap();
        insert_rfc_category(&conn, "RFC9114", "transport").unwrap();
        
        let retrieved = get_rfc_by_id(&conn, "RFC9114").unwrap();
        assert!(retrieved.is_some());
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.number, 9114);
        assert_eq!(retrieved.title, "HTTP/3");
        assert_eq!(retrieved.categories.len(), 2);
        
        cleanup_test_db(&db_path);
    }
    
    #[test]
    fn test_bookmark_operations() {
        let (db_path, conn) = create_test_db("bookmark");
        
        let rfc = Rfc {
            id: "RFC9000".to_string(),
            number: 9000,
            title: "QUIC".to_string(),
            r#abstract: None,
            status: "PROPOSED STANDARD".to_string(),
            published_date: None,
            authors: vec![],
            keywords: vec![],
            categories: vec![],
            summary_easy: None,
            summary_normal: None,
            summary_technical: None,
            implementation_guide: None,
            title_ja: None,
            abstract_ja: None,
            is_bookmarked: false,
        };
        
        upsert_rfc(&conn, &rfc).unwrap();
        
        // Add bookmark
        add_rfc_bookmark(&conn, "RFC9000", Some("重要なプロトコル")).unwrap();
        assert!(is_rfc_bookmarked(&conn, "RFC9000").unwrap());
        
        // Get bookmarks
        let bookmarks = get_rfc_bookmarks(&conn).unwrap();
        assert_eq!(bookmarks.len(), 1);
        assert_eq!(bookmarks[0].memo, Some("重要なプロトコル".to_string()));
        
        // Remove bookmark
        remove_rfc_bookmark(&conn, "RFC9000").unwrap();
        assert!(!is_rfc_bookmarked(&conn, "RFC9000").unwrap());
        
        cleanup_test_db(&db_path);
    }
}

