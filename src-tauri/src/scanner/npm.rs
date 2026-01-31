//! npm / pnpm / yarn 依存関係パーサー

use super::{Dependency, ScanDependencies, ScanError};
use serde::Deserialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// package-lock.json をパース
pub fn parse_package_lock(path: &Path) -> Result<ScanDependencies, ScanError> {
    let content = fs::read_to_string(path)?;
    let lock_file: PackageLockJson =
        serde_json::from_str(&content).map_err(|e| ScanError::Parse(e.to_string()))?;

    let mut dependencies = Vec::new();

    // v2/v3 format (packages)
    if let Some(packages) = lock_file.packages {
        for (name, info) in packages {
            // Skip root package (empty name) and linked packages
            if name.is_empty() || info.link.unwrap_or(false) {
                continue;
            }

            // パッケージ名を抽出（node_modules/... から）
            let package_name = extract_package_name(&name);
            
            if let Some(version) = info.version {
                dependencies.push(Dependency {
                    name: package_name,
                    version,
                    ecosystem: "npm".to_string(),
                });
            }
        }
    }
    // v1 format (dependencies)
    else if let Some(deps) = lock_file.dependencies {
        collect_dependencies_v1(&deps, &mut dependencies);
    }

    Ok(ScanDependencies {
        ecosystem: "npm".to_string(),
        source_file: path.to_string_lossy().to_string(),
        dependencies,
    })
}

/// パッケージ名を node_modules パスから抽出
fn extract_package_name(path: &str) -> String {
    // "node_modules/@scope/package" -> "@scope/package"
    // "node_modules/package" -> "package"
    if let Some(stripped) = path.strip_prefix("node_modules/") {
        // ネストされた node_modules を処理
        if let Some(pos) = stripped.rfind("node_modules/") {
            return stripped[pos + 13..].to_string();
        }
        return stripped.to_string();
    }
    path.to_string()
}

/// v1形式の依存関係を再帰的に収集
fn collect_dependencies_v1(
    deps: &HashMap<String, PackageLockDependency>,
    result: &mut Vec<Dependency>,
) {
    for (name, info) in deps {
        if let Some(ref version) = info.version {
            result.push(Dependency {
                name: name.clone(),
                version: version.clone(),
                ecosystem: "npm".to_string(),
            });
        }

        // ネストされた依存関係も収集
        if let Some(ref nested) = info.dependencies {
            collect_dependencies_v1(nested, result);
        }
    }
}

/// pnpm-lock.yaml をパース
pub fn parse_pnpm_lock(path: &Path) -> Result<ScanDependencies, ScanError> {
    let content = fs::read_to_string(path)?;
    let mut dependencies = Vec::new();

    // Simple YAML parsing without external crate
    // pnpm-lock.yaml format varies by version, we'll handle common formats
    
    let mut in_packages_section = false;
    let mut current_package: Option<String> = None;
    let mut current_version: Option<String> = None;

    for line in content.lines() {
        let trimmed = line.trim();

        // Check for packages section (v6+) or lockfileVersion
        if trimmed.starts_with("packages:") {
            in_packages_section = true;
            continue;
        }

        // New section starts
        if !trimmed.is_empty() && !line.starts_with(' ') && !line.starts_with('\t') {
            if trimmed != "packages:" {
                in_packages_section = false;
            }
        }

        if in_packages_section {
            // Package entry like "  /@scope/package@1.0.0:" or "  /package@1.0.0:"
            if line.starts_with("  /") || line.starts_with("  '/'") {
                // Save previous package if exists
                if let (Some(name), Some(version)) = (current_package.take(), current_version.take()) {
                    dependencies.push(Dependency {
                        name,
                        version,
                        ecosystem: "npm".to_string(),
                    });
                }

                // Parse new package
                let clean_line = trimmed.trim_start_matches('/').trim_start_matches("'/").trim_end_matches(':').trim_end_matches("':");
                if let Some((name, version)) = parse_pnpm_package_spec(clean_line) {
                    current_package = Some(name);
                    current_version = Some(version);
                }
            }
            // Version field within package (older format)
            else if trimmed.starts_with("version:") {
                if current_package.is_some() && current_version.is_none() {
                    let version = trimmed
                        .strip_prefix("version:")
                        .map(|s| s.trim().trim_matches('\'').trim_matches('"'))
                        .unwrap_or("");
                    if !version.is_empty() {
                        current_version = Some(version.to_string());
                    }
                }
            }
        }

        // Handle dependencies section (older pnpm format)
        if trimmed.starts_with("dependencies:") || trimmed.starts_with("devDependencies:") {
            continue;
        }
    }

    // Don't forget the last package
    if let (Some(name), Some(version)) = (current_package, current_version) {
        dependencies.push(Dependency {
            name,
            version,
            ecosystem: "npm".to_string(),
        });
    }

    // Deduplicate
    dependencies.sort_by(|a, b| (&a.name, &a.version).cmp(&(&b.name, &b.version)));
    dependencies.dedup_by(|a, b| a.name == b.name && a.version == b.version);

    Ok(ScanDependencies {
        ecosystem: "npm".to_string(),
        source_file: path.to_string_lossy().to_string(),
        dependencies,
    })
}

/// pnpm パッケージスペックをパース
/// "@scope/package@1.0.0" -> ("@scope/package", "1.0.0")
/// "package@1.0.0" -> ("package", "1.0.0")
fn parse_pnpm_package_spec(spec: &str) -> Option<(String, String)> {
    // Handle scoped packages
    if spec.starts_with('@') {
        // Find the second @ which separates name from version
        if let Some(at_pos) = spec[1..].find('@') {
            let name = &spec[..at_pos + 1];
            let version = &spec[at_pos + 2..];
            // Remove any suffix like (peer=...) or resolution info
            let version = version.split('(').next().unwrap_or(version).trim();
            return Some((name.to_string(), version.to_string()));
        }
    } else {
        // Non-scoped package
        if let Some(at_pos) = spec.find('@') {
            let name = &spec[..at_pos];
            let version = &spec[at_pos + 1..];
            let version = version.split('(').next().unwrap_or(version).trim();
            return Some((name.to_string(), version.to_string()));
        }
    }
    None
}

/// yarn.lock をパース (v1 クラシック形式と v2/berry 形式の両方に対応)
pub fn parse_yarn_lock(path: &Path) -> Result<ScanDependencies, ScanError> {
    let content = fs::read_to_string(path)?;
    
    // Detect format: v2/berry uses "__metadata:" at the top
    let is_berry = content.contains("__metadata:");
    
    let dependencies = if is_berry {
        parse_yarn_berry(&content)
    } else {
        parse_yarn_classic(&content)
    };

    Ok(ScanDependencies {
        ecosystem: "npm".to_string(),
        source_file: path.to_string_lossy().to_string(),
        dependencies,
    })
}

/// yarn v1 (classic) 形式をパース
/// Format:
/// ```
/// lodash@^4.17.21:
///   version "4.17.21"
///   resolved "..."
/// ```
fn parse_yarn_classic(content: &str) -> Vec<Dependency> {
    let mut dependencies = Vec::new();
    let mut seen: HashSet<(String, String)> = HashSet::new();
    
    let mut current_names: Vec<String> = Vec::new();
    let mut current_version: Option<String> = None;

    for line in content.lines() {
        // Skip comments
        if line.starts_with('#') {
            continue;
        }

        // Package declaration line (not indented, ends with ":")
        // Can have multiple packages: "lodash@^4.17.0, lodash@^4.17.21:"
        if !line.starts_with(' ') && !line.is_empty() && line.ends_with(':') {
            // Save previous package if exists
            if let Some(version) = current_version.take() {
                for name in current_names.drain(..) {
                    let key = (name.clone(), version.clone());
                    if !seen.contains(&key) {
                        seen.insert(key);
                        dependencies.push(Dependency {
                            name,
                            version: version.clone(),
                            ecosystem: "npm".to_string(),
                        });
                    }
                }
            }
            current_names.clear();

            // Parse package names from line
            let specs = line.trim_end_matches(':');
            for spec in specs.split(", ") {
                if let Some(name) = extract_yarn_package_name(spec.trim()) {
                    if !current_names.contains(&name) {
                        current_names.push(name);
                    }
                }
            }
        }
        // Version line (indented)
        else if line.starts_with("  version ") {
            let version_part = line.trim().strip_prefix("version ").unwrap_or("");
            let version = version_part.trim_matches('"').trim_matches('\'');
            if !version.is_empty() {
                current_version = Some(version.to_string());
            }
        }
    }

    // Don't forget the last package
    if let Some(version) = current_version {
        for name in current_names {
            let key = (name.clone(), version.clone());
            if !seen.contains(&key) {
                seen.insert(key);
                dependencies.push(Dependency {
                    name,
                    version: version.clone(),
                    ecosystem: "npm".to_string(),
                });
            }
        }
    }

    dependencies
}

/// yarn v2/berry 形式をパース
/// Format:
/// ```yaml
/// "lodash@npm:^4.17.21":
///   version: 4.17.21
///   resolution: "lodash@npm:4.17.21"
/// ```
fn parse_yarn_berry(content: &str) -> Vec<Dependency> {
    let mut dependencies = Vec::new();
    let mut seen: HashSet<(String, String)> = HashSet::new();
    
    let mut current_names: Vec<String> = Vec::new();
    let mut current_version: Option<String> = None;

    for line in content.lines() {
        // Skip metadata and comments
        if line.starts_with("__metadata:") || line.starts_with('#') {
            continue;
        }

        // Package declaration line (quoted, ends with ":")
        // "lodash@npm:^4.17.21, lodash@npm:^4.17.0":
        if !line.starts_with(' ') && !line.is_empty() && line.ends_with(':') {
            // Save previous package
            if let Some(version) = current_version.take() {
                for name in current_names.drain(..) {
                    let key = (name.clone(), version.clone());
                    if !seen.contains(&key) {
                        seen.insert(key);
                        dependencies.push(Dependency {
                            name,
                            version: version.clone(),
                            ecosystem: "npm".to_string(),
                        });
                    }
                }
            }
            current_names.clear();

            // Parse package names
            let specs = line.trim_matches('"').trim_end_matches(':').trim_matches('"');
            for spec in specs.split(", ") {
                if let Some(name) = extract_yarn_berry_package_name(spec.trim().trim_matches('"')) {
                    if !current_names.contains(&name) {
                        current_names.push(name);
                    }
                }
            }
        }
        // Version line
        else if line.trim().starts_with("version:") {
            let version_part = line.trim().strip_prefix("version:").unwrap_or("").trim();
            let version = version_part.trim_matches('"').trim_matches('\'');
            if !version.is_empty() {
                current_version = Some(version.to_string());
            }
        }
    }

    // Don't forget the last package
    if let Some(version) = current_version {
        for name in current_names {
            let key = (name.clone(), version.clone());
            if !seen.contains(&key) {
                seen.insert(key);
                dependencies.push(Dependency {
                    name,
                    version: version.clone(),
                    ecosystem: "npm".to_string(),
                });
            }
        }
    }

    dependencies
}

/// yarn classic パッケージ名を抽出
/// "lodash@^4.17.21" -> "lodash"
/// "@types/node@^18.0.0" -> "@types/node"
fn extract_yarn_package_name(spec: &str) -> Option<String> {
    let spec = spec.trim_matches('"').trim_matches('\'');
    
    // Handle scoped packages
    if spec.starts_with('@') {
        // Find the second @ which separates name from version
        if let Some(at_pos) = spec[1..].find('@') {
            return Some(spec[..at_pos + 1].to_string());
        }
    } else {
        // Non-scoped package
        if let Some(at_pos) = spec.find('@') {
            return Some(spec[..at_pos].to_string());
        }
    }
    None
}

/// yarn berry パッケージ名を抽出
/// "lodash@npm:^4.17.21" -> "lodash"
/// "@types/node@npm:^18.0.0" -> "@types/node"
fn extract_yarn_berry_package_name(spec: &str) -> Option<String> {
    let spec = spec.trim_matches('"').trim_matches('\'');
    
    // Berry format: "package@npm:version" or "@scope/package@npm:version"
    // First, try to find @npm: or @workspace: etc.
    
    if spec.starts_with('@') {
        // Scoped package: @scope/name@npm:version
        // Find @npm: or similar after the scope
        if let Some(protocol_pos) = spec[1..].find("@npm:") {
            return Some(spec[..protocol_pos + 1].to_string());
        }
        if let Some(protocol_pos) = spec[1..].find("@workspace:") {
            return Some(spec[..protocol_pos + 1].to_string());
        }
        // Fallback: find second @
        if let Some(at_pos) = spec[1..].find('@') {
            return Some(spec[..at_pos + 1].to_string());
        }
    } else {
        // Non-scoped: name@npm:version
        if let Some(at_pos) = spec.find('@') {
            return Some(spec[..at_pos].to_string());
        }
    }
    None
}

// --- JSON Types ---

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PackageLockJson {
    #[serde(default)]
    #[allow(dead_code)]
    lockfile_version: Option<i32>,
    #[serde(default)]
    packages: Option<HashMap<String, PackageLockPackage>>,
    #[serde(default)]
    dependencies: Option<HashMap<String, PackageLockDependency>>,
}

#[derive(Debug, Deserialize)]
struct PackageLockPackage {
    version: Option<String>,
    #[serde(default)]
    link: Option<bool>,
    #[serde(default)]
    #[allow(dead_code)]
    dev: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct PackageLockDependency {
    version: Option<String>,
    #[serde(default)]
    dependencies: Option<HashMap<String, PackageLockDependency>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_extract_package_name() {
        assert_eq!(extract_package_name("node_modules/lodash"), "lodash");
        assert_eq!(
            extract_package_name("node_modules/@types/node"),
            "@types/node"
        );
        assert_eq!(
            extract_package_name("node_modules/foo/node_modules/bar"),
            "bar"
        );
    }

    #[test]
    fn test_parse_pnpm_package_spec() {
        assert_eq!(
            parse_pnpm_package_spec("lodash@4.17.21"),
            Some(("lodash".to_string(), "4.17.21".to_string()))
        );
        assert_eq!(
            parse_pnpm_package_spec("@types/node@18.0.0"),
            Some(("@types/node".to_string(), "18.0.0".to_string()))
        );
    }

    #[test]
    fn test_extract_yarn_package_name() {
        assert_eq!(
            extract_yarn_package_name("lodash@^4.17.21"),
            Some("lodash".to_string())
        );
        assert_eq!(
            extract_yarn_package_name("@types/node@^18.0.0"),
            Some("@types/node".to_string())
        );
    }

    #[test]
    fn test_extract_yarn_berry_package_name() {
        assert_eq!(
            extract_yarn_berry_package_name("lodash@npm:^4.17.21"),
            Some("lodash".to_string())
        );
        assert_eq!(
            extract_yarn_berry_package_name("@types/node@npm:^18.0.0"),
            Some("@types/node".to_string())
        );
    }

    #[test]
    fn test_parse_yarn_classic() {
        let content = r#"# yarn lockfile v1

lodash@^4.17.21:
  version "4.17.21"
  resolved "https://registry.yarnpkg.com/lodash/-/lodash-4.17.21.tgz"
  integrity sha512-v2kDE...

"@types/node@^18.0.0":
  version "18.11.18"
  resolved "https://registry.yarnpkg.com/@types/node/-/node-18.11.18.tgz"
"#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();

        let result = parse_yarn_lock(file.path()).unwrap();
        assert_eq!(result.ecosystem, "npm");
        assert_eq!(result.dependencies.len(), 2);

        let lodash = result.dependencies.iter().find(|d| d.name == "lodash").unwrap();
        assert_eq!(lodash.version, "4.17.21");

        let types_node = result.dependencies.iter().find(|d| d.name == "@types/node").unwrap();
        assert_eq!(types_node.version, "18.11.18");
    }

    #[test]
    fn test_parse_yarn_berry() {
        let content = r#"__metadata:
  version: 6

"lodash@npm:^4.17.21":
  version: 4.17.21
  resolution: "lodash@npm:4.17.21"
  checksum: abc123

"@types/node@npm:^18.0.0":
  version: 18.11.18
  resolution: "@types/node@npm:18.11.18"
"#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();

        let result = parse_yarn_lock(file.path()).unwrap();
        assert_eq!(result.ecosystem, "npm");
        assert_eq!(result.dependencies.len(), 2);

        let lodash = result.dependencies.iter().find(|d| d.name == "lodash").unwrap();
        assert_eq!(lodash.version, "4.17.21");
    }
}
