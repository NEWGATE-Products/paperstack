//! pip (Python) 依存関係パーサー

use super::{Dependency, ScanDependencies, ScanError};
use std::fs;
use std::path::Path;

/// requirements.txt をパース
///
/// ⚠️ 注意: requirements.txt は生成方法によって含まれる依存関係が異なります。
///
/// - `pip freeze > requirements.txt`: 間接依存（transitive）も含む（推奨）
/// - 手動作成: 直接依存のみになることが多い
/// - `pip-compile` (pip-tools): 間接依存も含む
///
/// 包括的な脆弱性スキャンには、`pip freeze` や `poetry.lock`、
/// `Pipfile.lock` などロックファイル形式の使用を推奨します。
pub fn parse_requirements(path: &Path) -> Result<ScanDependencies, ScanError> {
    let content = fs::read_to_string(path)?;
    let mut dependencies = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // Skip comments and empty lines
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Skip options like -r, -e, --index-url, etc.
        if trimmed.starts_with('-') {
            continue;
        }

        // Parse package specification
        if let Some(dep) = parse_requirement_line(trimmed) {
            dependencies.push(dep);
        }
    }

    Ok(ScanDependencies {
        ecosystem: "PyPI".to_string(),
        source_file: path.to_string_lossy().to_string(),
        dependencies,
    })
}

/// 単一の requirements 行をパース
fn parse_requirement_line(line: &str) -> Option<Dependency> {
    // Remove environment markers (;)
    let line = line.split(';').next().unwrap_or(line).trim();
    
    // Remove extras like [security,socks]
    let line = if let Some(bracket_pos) = line.find('[') {
        if let Some(end_bracket) = line.find(']') {
            format!("{}{}", &line[..bracket_pos], &line[end_bracket + 1..])
        } else {
            line.to_string()
        }
    } else {
        line.to_string()
    };
    let line = line.trim();

    // Parse different version specifiers
    // ==, >=, <=, ~=, !=, >, <
    let version_ops = ["==", ">=", "<=", "~=", "!=", ">", "<"];
    
    for op in version_ops {
        if let Some(pos) = line.find(op) {
            let name = line[..pos].trim().to_lowercase();
            let version_part = &line[pos + op.len()..];
            
            // Handle version ranges like ">=1.0,<2.0"
            let version = version_part
                .split(',')
                .next()
                .unwrap_or(version_part)
                .trim();

            // For ==, use exact version; for others, note it's a constraint
            if !name.is_empty() && !version.is_empty() {
                return Some(Dependency {
                    name,
                    version: if op == "==" {
                        version.to_string()
                    } else {
                        format!("{}{}", op, version)
                    },
                    ecosystem: "PyPI".to_string(),
                });
            }
        }
    }

    // No version specifier - package name only
    let name = line.trim().to_lowercase();
    if !name.is_empty() && !name.contains(' ') {
        return Some(Dependency {
            name,
            version: "*".to_string(), // Any version
            ecosystem: "PyPI".to_string(),
        });
    }

    None
}

/// poetry.lock をパース
pub fn parse_poetry_lock(path: &Path) -> Result<ScanDependencies, ScanError> {
    let content = fs::read_to_string(path)?;
    let mut dependencies = Vec::new();

    // poetry.lock is TOML format
    // [[package]]
    // name = "package-name"
    // version = "1.0.0"

    let mut current_name: Option<String> = None;
    let mut current_version: Option<String> = None;
    let mut in_package_section = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed == "[[package]]" {
            // Save previous package
            if let (Some(name), Some(version)) = (current_name.take(), current_version.take()) {
                dependencies.push(Dependency {
                    name,
                    version,
                    ecosystem: "PyPI".to_string(),
                });
            }
            in_package_section = true;
            continue;
        }

        // New section
        if trimmed.starts_with('[') && !trimmed.starts_with("[[package]]") {
            if let (Some(name), Some(version)) = (current_name.take(), current_version.take()) {
                dependencies.push(Dependency {
                    name,
                    version,
                    ecosystem: "PyPI".to_string(),
                });
            }
            in_package_section = false;
            continue;
        }

        if in_package_section {
            if let Some(name) = parse_toml_string(trimmed, "name") {
                current_name = Some(name.to_lowercase());
            } else if let Some(version) = parse_toml_string(trimmed, "version") {
                current_version = Some(version);
            }
        }
    }

    // Last package
    if let (Some(name), Some(version)) = (current_name, current_version) {
        dependencies.push(Dependency {
            name,
            version,
            ecosystem: "PyPI".to_string(),
        });
    }

    Ok(ScanDependencies {
        ecosystem: "PyPI".to_string(),
        source_file: path.to_string_lossy().to_string(),
        dependencies,
    })
}

/// Pipfile.lock をパース（JSON形式）
pub fn parse_pipfile_lock(path: &Path) -> Result<ScanDependencies, ScanError> {
    let content = fs::read_to_string(path)?;
    let mut dependencies = Vec::new();

    // Pipfile.lock is JSON
    // Simple parsing without full JSON crate support
    // Structure:
    // {
    //   "default": {
    //     "package-name": { "version": "==1.0.0" },
    //     ...
    //   },
    //   "develop": { ... }
    // }

    let json: serde_json::Value =
        serde_json::from_str(&content).map_err(|e| ScanError::Parse(e.to_string()))?;

    // Process "default" dependencies
    if let Some(default) = json.get("default").and_then(|v| v.as_object()) {
        for (name, info) in default {
            if let Some(version) = info.get("version").and_then(|v| v.as_str()) {
                let version = version.trim_start_matches("==");
                dependencies.push(Dependency {
                    name: name.to_lowercase(),
                    version: version.to_string(),
                    ecosystem: "PyPI".to_string(),
                });
            }
        }
    }

    // Process "develop" dependencies
    if let Some(develop) = json.get("develop").and_then(|v| v.as_object()) {
        for (name, info) in develop {
            if let Some(version) = info.get("version").and_then(|v| v.as_str()) {
                let version = version.trim_start_matches("==");
                dependencies.push(Dependency {
                    name: name.to_lowercase(),
                    version: version.to_string(),
                    ecosystem: "PyPI".to_string(),
                });
            }
        }
    }

    Ok(ScanDependencies {
        ecosystem: "PyPI".to_string(),
        source_file: path.to_string_lossy().to_string(),
        dependencies,
    })
}

/// TOML string 値をパース
fn parse_toml_string(line: &str, key: &str) -> Option<String> {
    let pattern = format!("{} = ", key);
    if line.starts_with(&pattern) {
        let value_part = &line[pattern.len()..];
        let value = value_part
            .trim()
            .trim_start_matches('"')
            .trim_end_matches('"');
        return Some(value.to_string());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_requirement_line() {
        assert_eq!(
            parse_requirement_line("requests==2.28.0"),
            Some(Dependency {
                name: "requests".to_string(),
                version: "2.28.0".to_string(),
                ecosystem: "PyPI".to_string(),
            })
        );

        assert_eq!(
            parse_requirement_line("django>=3.0,<4.0"),
            Some(Dependency {
                name: "django".to_string(),
                version: ">=3.0".to_string(),
                ecosystem: "PyPI".to_string(),
            })
        );

        assert_eq!(
            parse_requirement_line("flask"),
            Some(Dependency {
                name: "flask".to_string(),
                version: "*".to_string(),
                ecosystem: "PyPI".to_string(),
            })
        );
    }

    #[test]
    fn test_parse_requirements() {
        let content = r#"
# This is a comment
requests==2.28.0
django>=3.0
flask
-r other-requirements.txt
"#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();

        let result = parse_requirements(file.path()).unwrap();
        assert_eq!(result.ecosystem, "PyPI");
        assert_eq!(result.dependencies.len(), 3);
    }
}
