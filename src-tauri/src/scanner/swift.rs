//! Swift Package Manager 依存関係パーサー

use super::{Dependency, ScanDependencies, ScanError};
use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Package.resolved をパース (JSON形式)
pub fn parse_package_resolved(path: &Path) -> Result<ScanDependencies, ScanError> {
    let content = fs::read_to_string(path)?;

    // Try v2 format first, then v1
    if let Ok(v2) = serde_json::from_str::<PackageResolvedV2>(&content) {
        return parse_v2(v2, path);
    }

    if let Ok(v1) = serde_json::from_str::<PackageResolvedV1>(&content) {
        return parse_v1(v1, path);
    }

    Err(ScanError::Parse("Unknown Package.resolved format".to_string()))
}

fn parse_v2(resolved: PackageResolvedV2, path: &Path) -> Result<ScanDependencies, ScanError> {
    let mut dependencies = Vec::new();

    for pin in resolved.pins {
        // Extract package name from URL
        let name = extract_package_name(&pin.location);

        if let Some(version) = pin.state.version {
            dependencies.push(Dependency {
                name,
                version,
                ecosystem: "SwiftURL".to_string(),
            });
        }
    }

    Ok(ScanDependencies {
        ecosystem: "SwiftURL".to_string(),
        source_file: path.to_string_lossy().to_string(),
        dependencies,
    })
}

fn parse_v1(resolved: PackageResolvedV1, path: &Path) -> Result<ScanDependencies, ScanError> {
    let mut dependencies = Vec::new();

    if let Some(object) = resolved.object {
        for pin in object.pins {
            let name = pin.package;

            if let Some(version) = pin.state.version {
                dependencies.push(Dependency {
                    name,
                    version,
                    ecosystem: "SwiftURL".to_string(),
                });
            }
        }
    }

    Ok(ScanDependencies {
        ecosystem: "SwiftURL".to_string(),
        source_file: path.to_string_lossy().to_string(),
        dependencies,
    })
}

/// URLからパッケージ名を抽出
fn extract_package_name(url: &str) -> String {
    // Remove .git suffix if present
    let url = url.strip_suffix(".git").unwrap_or(url);

    // Get the last path component
    if let Some(last_slash) = url.rfind('/') {
        return url[last_slash + 1..].to_string();
    }

    url.to_string()
}

// --- JSON Types for v2 format ---

#[derive(Debug, Deserialize)]
struct PackageResolvedV2 {
    #[serde(default)]
    pins: Vec<PinV2>,
    #[allow(dead_code)]
    version: i32,
}

#[derive(Debug, Deserialize)]
struct PinV2 {
    #[allow(dead_code)]
    identity: String,
    location: String,
    state: PinStateV2,
}

#[derive(Debug, Deserialize)]
struct PinStateV2 {
    #[serde(default)]
    version: Option<String>,
}

// --- JSON Types for v1 format ---

#[derive(Debug, Deserialize)]
struct PackageResolvedV1 {
    object: Option<ObjectV1>,
    #[allow(dead_code)]
    version: i32,
}

#[derive(Debug, Deserialize)]
struct ObjectV1 {
    pins: Vec<PinV1>,
}

#[derive(Debug, Deserialize)]
struct PinV1 {
    package: String,
    state: PinStateV1,
}

#[derive(Debug, Deserialize)]
struct PinStateV1 {
    #[serde(default)]
    version: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_package_resolved_v2() {
        let content = r#"{
  "pins": [
    {
      "identity": "alamofire",
      "kind": "remoteSourceControl",
      "location": "https://github.com/Alamofire/Alamofire.git",
      "state": {
        "revision": "abc123",
        "version": "5.6.4"
      }
    },
    {
      "identity": "swift-argument-parser",
      "kind": "remoteSourceControl",
      "location": "https://github.com/apple/swift-argument-parser",
      "state": {
        "revision": "def456",
        "version": "1.2.2"
      }
    }
  ],
  "version": 2
}"#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();

        let result = parse_package_resolved(file.path()).unwrap();
        assert_eq!(result.ecosystem, "SwiftURL");
        assert_eq!(result.dependencies.len(), 2);

        let alamofire = result.dependencies.iter().find(|d| d.name == "Alamofire").unwrap();
        assert_eq!(alamofire.version, "5.6.4");
    }

    #[test]
    fn test_parse_package_resolved_v1() {
        let content = r#"{
  "object": {
    "pins": [
      {
        "package": "Alamofire",
        "repositoryURL": "https://github.com/Alamofire/Alamofire.git",
        "state": {
          "branch": null,
          "revision": "abc123",
          "version": "5.6.4"
        }
      }
    ]
  },
  "version": 1
}"#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();

        let result = parse_package_resolved(file.path()).unwrap();
        assert_eq!(result.ecosystem, "SwiftURL");
        assert_eq!(result.dependencies.len(), 1);
        assert_eq!(result.dependencies[0].name, "Alamofire");
        assert_eq!(result.dependencies[0].version, "5.6.4");
    }

    #[test]
    fn test_extract_package_name() {
        assert_eq!(
            extract_package_name("https://github.com/Alamofire/Alamofire.git"),
            "Alamofire"
        );
        assert_eq!(
            extract_package_name("https://github.com/apple/swift-argument-parser"),
            "swift-argument-parser"
        );
    }
}
