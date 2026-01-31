//! Go modules 依存関係パーサー

use super::{Dependency, ScanDependencies, ScanError};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// go.sum をパース
pub fn parse_go_sum(path: &Path) -> Result<ScanDependencies, ScanError> {
    let content = fs::read_to_string(path)?;
    let mut dependencies = Vec::new();
    let mut seen: HashSet<(String, String)> = HashSet::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // Skip empty lines
        if trimmed.is_empty() {
            continue;
        }

        // go.sum format:
        // module/path v1.2.3 h1:hash...
        // module/path v1.2.3/go.mod h1:hash...

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() >= 2 {
            let module_path = parts[0];
            let version_raw = parts[1];

            // Parse version (remove /go.mod suffix if present)
            let version = version_raw
                .strip_suffix("/go.mod")
                .unwrap_or(version_raw);

            // Clean version string (remove +incompatible, etc.)
            let version = clean_go_version(version);

            // Deduplicate
            let key = (module_path.to_string(), version.clone());
            if !seen.contains(&key) {
                seen.insert(key);
                dependencies.push(Dependency {
                    name: module_path.to_string(),
                    version,
                    ecosystem: "Go".to_string(),
                });
            }
        }
    }

    Ok(ScanDependencies {
        ecosystem: "Go".to_string(),
        source_file: path.to_string_lossy().to_string(),
        dependencies,
    })
}

/// Go.mod をパース（補助的に使用）
#[allow(dead_code)]
pub fn parse_go_mod(path: &Path) -> Result<ScanDependencies, ScanError> {
    let content = fs::read_to_string(path)?;
    let mut dependencies = Vec::new();

    let mut in_require_block = false;

    for line in content.lines() {
        let trimmed = line.trim();

        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }

        // Start of require block
        if trimmed.starts_with("require (") || trimmed == "require (" {
            in_require_block = true;
            continue;
        }

        // End of require block
        if trimmed == ")" && in_require_block {
            in_require_block = false;
            continue;
        }

        // Single-line require
        if trimmed.starts_with("require ") && !trimmed.contains('(') {
            let rest = trimmed.strip_prefix("require ").unwrap().trim();
            if let Some(dep) = parse_go_require_line(rest) {
                dependencies.push(dep);
            }
            continue;
        }

        // Inside require block
        if in_require_block {
            if let Some(dep) = parse_go_require_line(trimmed) {
                dependencies.push(dep);
            }
        }
    }

    Ok(ScanDependencies {
        ecosystem: "Go".to_string(),
        source_file: path.to_string_lossy().to_string(),
        dependencies,
    })
}

/// require 行をパース
/// "github.com/pkg/errors v0.9.1" -> Dependency
/// "github.com/pkg/errors v0.9.1 // indirect" -> Dependency
fn parse_go_require_line(line: &str) -> Option<Dependency> {
    // Remove comments
    let line = line.split("//").next().unwrap_or(line).trim();

    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 2 {
        let module_path = parts[0];
        let version = clean_go_version(parts[1]);

        return Some(Dependency {
            name: module_path.to_string(),
            version,
            ecosystem: "Go".to_string(),
        });
    }

    None
}

/// Go バージョン文字列をクリーンアップ
fn clean_go_version(version: &str) -> String {
    // Remove +incompatible suffix
    let version = version.strip_suffix("+incompatible").unwrap_or(version);
    
    // Remove pseudo-version timestamps but keep the base
    // v0.0.0-20210101120000-abcdef123456 -> v0.0.0-20210101120000-abcdef123456
    // (keep as-is for vulnerability matching)
    
    version.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_go_sum() {
        let content = r#"
github.com/pkg/errors v0.9.1 h1:FEBLx1zS214owpjy7qsBeixbURkuhQAwrK5UwLGTwt4=
github.com/pkg/errors v0.9.1/go.mod h1:bwawxfHBFNV+L2hUp1rHADufV3IMtnDRdf1r5NINEl0=
golang.org/x/sys v0.0.0-20210615035016-665e8c7367d1 h1:SrN+KX8Art/Sf4HNj6Zcz06G7VEz+7w9tdXTPOZ7+l4=
"#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();

        let result = parse_go_sum(file.path()).unwrap();
        assert_eq!(result.ecosystem, "Go");
        assert_eq!(result.dependencies.len(), 2); // Deduplicated
        assert_eq!(result.dependencies[0].name, "github.com/pkg/errors");
        assert_eq!(result.dependencies[0].version, "v0.9.1");
    }

    #[test]
    fn test_clean_go_version() {
        assert_eq!(clean_go_version("v1.2.3"), "v1.2.3");
        assert_eq!(clean_go_version("v1.2.3+incompatible"), "v1.2.3");
        assert_eq!(
            clean_go_version("v0.0.0-20210101120000-abcdef123456"),
            "v0.0.0-20210101120000-abcdef123456"
        );
    }
}
