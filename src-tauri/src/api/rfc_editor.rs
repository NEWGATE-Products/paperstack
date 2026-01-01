//! RFC Editor API Client
//! 
//! Fetches RFC metadata from RFC Editor Index XML

use quick_xml::events::Event;
use quick_xml::Reader;
use thiserror::Error;

const RFC_INDEX_URL: &str = "https://www.rfc-editor.org/rfc-index.xml";
const RFC_TEXT_BASE_URL: &str = "https://www.rfc-editor.org/rfc";

#[derive(Error, Debug)]
pub enum RfcEditorError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("XML parse error: {0}")]
    XmlError(#[from] quick_xml::Error),
    #[error("Invalid RFC data: {0}")]
    InvalidData(String),
}

/// Raw RFC entry from XML
#[derive(Debug, Clone, Default)]
pub struct RfcEntry {
    pub doc_id: String,           // "RFC9114"
    pub title: String,
    pub authors: Vec<String>,
    pub date_month: Option<String>,
    pub date_year: Option<String>,
    pub r#abstract: Option<String>,
    pub status: Option<String>,
    pub keywords: Vec<String>,
}

impl RfcEntry {
    /// Extract RFC number from doc_id (e.g., "RFC9114" -> 9114)
    pub fn number(&self) -> Option<i32> {
        self.doc_id
            .strip_prefix("RFC")
            .and_then(|n| n.parse().ok())
    }
    
    /// Get published date in YYYY-MM format
    pub fn published_date(&self) -> Option<String> {
        match (&self.date_year, &self.date_month) {
            (Some(year), Some(month)) => {
                let month_num = month_to_number(month);
                Some(format!("{}-{:02}", year, month_num))
            }
            (Some(year), None) => Some(format!("{}-01", year)),
            _ => None,
        }
    }
}

fn month_to_number(month: &str) -> u32 {
    match month.to_lowercase().as_str() {
        "january" => 1,
        "february" => 2,
        "march" => 3,
        "april" => 4,
        "may" => 5,
        "june" => 6,
        "july" => 7,
        "august" => 8,
        "september" => 9,
        "october" => 10,
        "november" => 11,
        "december" => 12,
        _ => 1,
    }
}

pub struct RfcEditorClient {
    client: reqwest::Client,
}

impl RfcEditorClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
    
    /// Fetch RFC Index XML and parse all RFC entries
    pub async fn fetch_rfc_index(&self) -> Result<Vec<RfcEntry>, RfcEditorError> {
        println!("Fetching RFC index from {}...", RFC_INDEX_URL);
        
        let response = self.client
            .get(RFC_INDEX_URL)
            .send()
            .await?;
        
        let xml_text = response.text().await?;
        println!("Downloaded {} bytes of XML", xml_text.len());
        
        let entries = parse_rfc_index(&xml_text)?;
        println!("Parsed {} RFC entries", entries.len());
        
        Ok(entries)
    }
    
    /// Fetch RFC full text
    pub async fn fetch_rfc_text(&self, rfc_number: i32) -> Result<String, RfcEditorError> {
        let url = format!("{}/rfc{}.txt", RFC_TEXT_BASE_URL, rfc_number);
        
        let response = self.client
            .get(&url)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(RfcEditorError::InvalidData(format!(
                "RFC {} not found (status: {})", rfc_number, response.status()
            )));
        }
        
        let text = response.text().await?;
        Ok(text)
    }
}

impl Default for RfcEditorClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse RFC Index XML
fn parse_rfc_index(xml: &str) -> Result<Vec<RfcEntry>, RfcEditorError> {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);
    
    let mut entries: Vec<RfcEntry> = Vec::new();
    let mut current_entry: Option<RfcEntry> = None;
    let mut current_element = String::new();
    let mut in_abstract = false;
    let mut abstract_text = String::new();
    let mut in_author = false;
    let mut current_author_name = String::new();
    let mut in_date = false;
    let mut in_keywords = false;
    
    let mut buf = Vec::new();
    
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                current_element = name.clone();
                
                match name.as_str() {
                    "rfc-entry" => {
                        current_entry = Some(RfcEntry::default());
                    }
                    "abstract" => {
                        in_abstract = true;
                        abstract_text.clear();
                    }
                    "author" => {
                        in_author = true;
                        current_author_name.clear();
                    }
                    "date" => {
                        in_date = true;
                    }
                    "keywords" => {
                        in_keywords = true;
                    }
                    _ => {}
                }
            }
            Ok(Event::End(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                
                match name.as_str() {
                    "rfc-entry" => {
                        if let Some(entry) = current_entry.take() {
                            // Only include actual RFCs (not STDs, BCPs, etc.)
                            if entry.doc_id.starts_with("RFC") && entry.number().is_some() {
                                entries.push(entry);
                            }
                        }
                    }
                    "abstract" => {
                        if let Some(ref mut entry) = current_entry {
                            let cleaned = abstract_text.trim().to_string();
                            if !cleaned.is_empty() {
                                entry.r#abstract = Some(cleaned);
                            }
                        }
                        in_abstract = false;
                    }
                    "author" => {
                        if let Some(ref mut entry) = current_entry {
                            let name = current_author_name.trim().to_string();
                            if !name.is_empty() {
                                entry.authors.push(name);
                            }
                        }
                        in_author = false;
                    }
                    "date" => {
                        in_date = false;
                    }
                    "keywords" => {
                        in_keywords = false;
                    }
                    _ => {}
                }
                current_element.clear();
            }
            Ok(Event::Text(e)) => {
                let text = e.unescape().unwrap_or_default().to_string();
                
                if in_abstract && current_element == "p" {
                    if !abstract_text.is_empty() {
                        abstract_text.push(' ');
                    }
                    abstract_text.push_str(&text);
                } else if in_author && current_element == "name" {
                    current_author_name.push_str(&text);
                } else if let Some(ref mut entry) = current_entry {
                    match current_element.as_str() {
                        "doc-id" => entry.doc_id = text,
                        // Only capture title if NOT inside <author> element
                        // (author/title contains role like "Editor", not the RFC title)
                        "title" if !in_author => entry.title = text,
                        "month" if in_date => entry.date_month = Some(text),
                        "year" if in_date => entry.date_year = Some(text),
                        "current-status" => entry.status = Some(text),
                        "kw" if in_keywords => {
                            let kw = text.trim().to_string();
                            if !kw.is_empty() {
                                entry.keywords.push(kw);
                            }
                        }
                        _ => {}
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                eprintln!("XML parse error at position {}: {:?}", reader.buffer_position(), e);
                // Continue parsing despite errors
            }
            _ => {}
        }
        buf.clear();
    }
    
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_rfc_entry() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<rfc-index>
  <rfc-entry>
    <doc-id>RFC9114</doc-id>
    <title>HTTP/3</title>
    <author>
      <name>M. Bishop</name>
    </author>
    <date>
      <month>June</month>
      <year>2022</year>
    </date>
    <abstract>
      <p>HTTP/3 is the third version of the Hypertext Transfer Protocol.</p>
    </abstract>
    <current-status>PROPOSED STANDARD</current-status>
    <keywords>
      <kw>HTTP</kw>
      <kw>QUIC</kw>
    </keywords>
  </rfc-entry>
</rfc-index>"#;
        
        let entries = parse_rfc_index(xml).unwrap();
        assert_eq!(entries.len(), 1);
        
        let entry = &entries[0];
        assert_eq!(entry.doc_id, "RFC9114");
        assert_eq!(entry.title, "HTTP/3");
        assert_eq!(entry.number(), Some(9114));
        assert_eq!(entry.published_date(), Some("2022-06".to_string()));
        assert_eq!(entry.authors, vec!["M. Bishop"]);
        assert!(entry.r#abstract.as_ref().unwrap().contains("HTTP/3"));
        assert_eq!(entry.status, Some("PROPOSED STANDARD".to_string()));
        assert_eq!(entry.keywords, vec!["HTTP", "QUIC"]);
    }
    
    #[test]
    fn test_month_to_number() {
        assert_eq!(month_to_number("January"), 1);
        assert_eq!(month_to_number("june"), 6);
        assert_eq!(month_to_number("DECEMBER"), 12);
    }
    
    #[test]
    fn test_author_title_not_overwrite_rfc_title() {
        // Test that <title> inside <author> (role like "Editor") 
        // does not overwrite the RFC title
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<rfc-index>
  <rfc-entry>
    <doc-id>RFC9000</doc-id>
    <title>QUIC: A UDP-Based Multiplexed and Secure Transport</title>
    <author>
      <name>J. Iyengar</name>
      <title>Editor</title>
    </author>
    <author>
      <name>M. Thomson</name>
      <title>Editor</title>
    </author>
    <date>
      <month>May</month>
      <year>2021</year>
    </date>
    <current-status>PROPOSED STANDARD</current-status>
  </rfc-entry>
</rfc-index>"#;
        
        let entries = parse_rfc_index(xml).unwrap();
        assert_eq!(entries.len(), 1);
        
        let entry = &entries[0];
        assert_eq!(entry.doc_id, "RFC9000");
        // Title should NOT be "Editor"
        assert_eq!(entry.title, "QUIC: A UDP-Based Multiplexed and Secure Transport");
        assert_eq!(entry.authors, vec!["J. Iyengar", "M. Thomson"]);
    }
}

