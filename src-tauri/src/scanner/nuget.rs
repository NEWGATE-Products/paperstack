//! NuGet 依存関係パーサー

use super::{Dependency, ScanDependencies, ScanError};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// packages.lock.json をパース
pub fn parse_packages_lock(path: &Path) -> Result<ScanDependencies, ScanError> {
    let content = fs::read_to_string(path)?;
    let lock_file: NuGetLockFile =
        serde_json::from_str(&content).map_err(|e| ScanError::Parse(e.to_string()))?;

    let mut dependencies = Vec::new();

    // Iterate over all target frameworks
    for (_framework, packages) in lock_file.dependencies {
        for (name, info) in packages {
            // Skip "type": "Project" entries (these are project references, not packages)
            if info.package_type.as_deref() == Some("Project") {
                continue;
            }

            if let Some(ref version) = info.resolved {
                dependencies.push(Dependency {
                    name: name.clone(),
                    version: version.clone(),
                    ecosystem: "NuGet".to_string(),
                });
            }
        }
    }

    // Deduplicate (same package may appear in multiple frameworks)
    dependencies.sort_by(|a, b| (&a.name, &a.version).cmp(&(&b.name, &b.version)));
    dependencies.dedup_by(|a, b| a.name == b.name && a.version == b.version);

    Ok(ScanDependencies {
        ecosystem: "NuGet".to_string(),
        source_file: path.to_string_lossy().to_string(),
        dependencies,
    })
}

// --- JSON Types ---

#[derive(Debug, Deserialize)]
struct NuGetLockFile {
    #[serde(default)]
    #[allow(dead_code)]
    version: i32,
    #[serde(default)]
    dependencies: HashMap<String, HashMap<String, NuGetPackage>>,
}

#[derive(Debug, Deserialize)]
struct NuGetPackage {
    #[serde(rename = "type")]
    package_type: Option<String>,
    resolved: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    requested: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_packages_lock() {
        let content = r#"{
  "version": 1,
  "dependencies": {
    "net6.0": {
      "Newtonsoft.Json": {
        "type": "Transitive",
        "resolved": "13.0.1"
      },
      "Microsoft.Extensions.Logging": {
        "type": "Direct",
        "resolved": "6.0.0",
        "requested": "[6.0.0, )"
      },
      "MyProject": {
        "type": "Project"
      }
    }
  }
}"#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();

        let result = parse_packages_lock(file.path()).unwrap();
        assert_eq!(result.ecosystem, "NuGet");
        assert_eq!(result.dependencies.len(), 2);
        
        let names: Vec<&str> = result.dependencies.iter().map(|d| d.name.as_str()).collect();
        assert!(names.contains(&"Newtonsoft.Json"));
        assert!(names.contains(&"Microsoft.Extensions.Logging"));
    }
}
