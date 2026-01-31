//! Packagist (Composer) 依存関係パーサー

use super::{Dependency, ScanDependencies, ScanError};
use serde::Deserialize;
use std::fs;
use std::path::Path;

/// composer.lock をパース
pub fn parse_composer_lock(path: &Path) -> Result<ScanDependencies, ScanError> {
    let content = fs::read_to_string(path)?;
    let lock_file: ComposerLock =
        serde_json::from_str(&content).map_err(|e| ScanError::Parse(e.to_string()))?;

    let mut dependencies = Vec::new();

    // Parse regular packages
    for package in lock_file.packages {
        dependencies.push(Dependency {
            name: package.name,
            version: normalize_composer_version(&package.version),
            ecosystem: "Packagist".to_string(),
        });
    }

    // Parse dev packages
    for package in lock_file.packages_dev {
        dependencies.push(Dependency {
            name: package.name,
            version: normalize_composer_version(&package.version),
            ecosystem: "Packagist".to_string(),
        });
    }

    Ok(ScanDependencies {
        ecosystem: "Packagist".to_string(),
        source_file: path.to_string_lossy().to_string(),
        dependencies,
    })
}

/// Composer バージョン文字列を正規化
/// "v1.2.3" -> "1.2.3"
fn normalize_composer_version(version: &str) -> String {
    version.strip_prefix('v').unwrap_or(version).to_string()
}

// --- JSON Types ---

#[derive(Debug, Deserialize)]
struct ComposerLock {
    #[serde(default)]
    packages: Vec<ComposerPackage>,
    #[serde(default, rename = "packages-dev")]
    packages_dev: Vec<ComposerPackage>,
}

#[derive(Debug, Deserialize)]
struct ComposerPackage {
    name: String,
    version: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_composer_lock() {
        let content = r#"{
    "_readme": [
        "This file locks the dependencies of your project to a known state"
    ],
    "content-hash": "abc123",
    "packages": [
        {
            "name": "laravel/framework",
            "version": "v9.52.0",
            "source": {}
        },
        {
            "name": "symfony/console",
            "version": "v6.2.5",
            "source": {}
        }
    ],
    "packages-dev": [
        {
            "name": "phpunit/phpunit",
            "version": "9.6.3",
            "source": {}
        }
    ]
}"#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();

        let result = parse_composer_lock(file.path()).unwrap();
        assert_eq!(result.ecosystem, "Packagist");
        assert_eq!(result.dependencies.len(), 3);

        // Check version normalization
        let laravel = result.dependencies.iter().find(|d| d.name == "laravel/framework").unwrap();
        assert_eq!(laravel.version, "9.52.0"); // v prefix removed
    }

    #[test]
    fn test_normalize_composer_version() {
        assert_eq!(normalize_composer_version("v1.2.3"), "1.2.3");
        assert_eq!(normalize_composer_version("1.2.3"), "1.2.3");
        assert_eq!(normalize_composer_version("v9.52.0"), "9.52.0");
    }
}
