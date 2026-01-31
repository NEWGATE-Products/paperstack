//! CocoaPods 依存関係パーサー

use super::{Dependency, ScanDependencies, ScanError};
use std::fs;
use std::path::Path;

/// Podfile.lock をパース (YAML-like形式)
pub fn parse_podfile_lock(path: &Path) -> Result<ScanDependencies, ScanError> {
    let content = fs::read_to_string(path)?;
    let mut dependencies = Vec::new();

    let mut in_pods_section = false;

    for line in content.lines() {
        // Detect PODS section
        if line == "PODS:" {
            in_pods_section = true;
            continue;
        }

        // New section starts (no leading spaces)
        if !line.starts_with(' ') && !line.is_empty() && line != "PODS:" {
            in_pods_section = false;
            continue;
        }

        // Parse pod entries in PODS section
        // Format (2 spaces): "  - PodName (version):" or "  - PodName (version)"
        if in_pods_section && line.starts_with("  - ") && !line.starts_with("    ") {
            let trimmed = line.trim_start_matches("  - ");
            if let Some(dep) = parse_pod_entry(trimmed) {
                dependencies.push(dep);
            }
        }
    }

    Ok(ScanDependencies {
        ecosystem: "CocoaPods".to_string(),
        source_file: path.to_string_lossy().to_string(),
        dependencies,
    })
}

/// Pod エントリをパース
/// "Alamofire (5.6.4)" -> Dependency
/// "Alamofire (5.6.4):" -> Dependency (with sub-dependencies)
fn parse_pod_entry(entry: &str) -> Option<Dependency> {
    // Find version in parentheses
    let paren_start = entry.find('(')?;
    let paren_end = entry.find(')')?;

    let name = entry[..paren_start].trim();
    let version = &entry[paren_start + 1..paren_end];

    if !name.is_empty() && !version.is_empty() {
        return Some(Dependency {
            name: name.to_string(),
            version: version.to_string(),
            ecosystem: "CocoaPods".to_string(),
        });
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_podfile_lock() {
        let content = r#"PODS:
  - Alamofire (5.6.4)
  - Firebase/Analytics (10.5.0):
    - Firebase/Core
    - FirebaseAnalytics (~> 10.5.0)
  - Firebase/Core (10.5.0):
    - Firebase/CoreOnly
    - FirebaseAnalytics (~> 10.5.0)
  - SDWebImage (5.15.0):
    - SDWebImage/Core (= 5.15.0)

DEPENDENCIES:
  - Alamofire
  - Firebase/Analytics
  - SDWebImage

SPEC REPOS:
  trunk:
    - Alamofire
    - Firebase
    - SDWebImage

SPEC CHECKSUMS:
  Alamofire: hash123
  Firebase: hash456

PODFILE CHECKSUM: abcdef

COCOAPODS: 1.12.0
"#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();

        let result = parse_podfile_lock(file.path()).unwrap();
        assert_eq!(result.ecosystem, "CocoaPods");
        assert_eq!(result.dependencies.len(), 4);

        let alamofire = result.dependencies.iter().find(|d| d.name == "Alamofire").unwrap();
        assert_eq!(alamofire.version, "5.6.4");

        let firebase = result.dependencies.iter().find(|d| d.name == "Firebase/Analytics").unwrap();
        assert_eq!(firebase.version, "10.5.0");
    }

    #[test]
    fn test_parse_pod_entry() {
        let dep = parse_pod_entry("Alamofire (5.6.4)").unwrap();
        assert_eq!(dep.name, "Alamofire");
        assert_eq!(dep.version, "5.6.4");

        let dep2 = parse_pod_entry("Firebase/Analytics (10.5.0):").unwrap();
        assert_eq!(dep2.name, "Firebase/Analytics");
        assert_eq!(dep2.version, "10.5.0");
    }
}
