//! Hex (Elixir) 依存関係パーサー

use super::{Dependency, ScanDependencies, ScanError};
use std::fs;
use std::path::Path;

/// mix.lock をパース (Elixir term形式)
pub fn parse_mix_lock(path: &Path) -> Result<ScanDependencies, ScanError> {
    let content = fs::read_to_string(path)?;
    let mut dependencies = Vec::new();

    // mix.lock format (Elixir terms):
    // %{
    //   "package_name": {:hex, :package_name, "1.0.0", "hash", [:mix], [...], "hexpm", "hash"},
    //   ...
    // }

    for line in content.lines() {
        let trimmed = line.trim();

        // Skip empty lines and brackets
        if trimmed.is_empty() || trimmed == "%{" || trimmed == "}" {
            continue;
        }

        // Parse line like: "phoenix": {:hex, :phoenix, "1.6.15", ...},
        if let Some(dep) = parse_mix_lock_line(trimmed) {
            dependencies.push(dep);
        }
    }

    Ok(ScanDependencies {
        ecosystem: "Hex".to_string(),
        source_file: path.to_string_lossy().to_string(),
        dependencies,
    })
}

/// mix.lock の1行をパース
fn parse_mix_lock_line(line: &str) -> Option<Dependency> {
    // Format: "package_name": {:hex, :package_name, "version", ...},
    
    // Find the package name (first quoted string)
    let quote_start = line.find('"')?;
    let rest = &line[quote_start + 1..];
    let quote_end = rest.find('"')?;
    let package_name = &rest[..quote_end];

    // Find version (third element in tuple, after :hex and :atom)
    // Look for pattern like: "1.0.0"
    let tuple_start = line.find("{:")?;
    let tuple_content = &line[tuple_start..];

    // Find version string - it's the first quoted string after :hex, :name
    // Pattern: {:hex, :name, "version", ...
    let mut in_tuple = false;
    let mut quote_count = 0;
    let mut version_start = None;
    let mut version_end = None;

    for (i, c) in tuple_content.char_indices() {
        if c == '{' {
            in_tuple = true;
        }
        if in_tuple && c == '"' {
            quote_count += 1;
            if quote_count == 1 {
                version_start = Some(i + 1);
            } else if quote_count == 2 {
                version_end = Some(i);
                break;
            }
        }
    }

    if let (Some(start), Some(end)) = (version_start, version_end) {
        let version = &tuple_content[start..end];
        return Some(Dependency {
            name: package_name.to_string(),
            version: version.to_string(),
            ecosystem: "Hex".to_string(),
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
    fn test_parse_mix_lock() {
        let content = r#"%{
  "cowboy": {:hex, :cowboy, "2.9.0", "865dd8b6607e14cf03282e10e934f0c1a5407273a8a4e8469b4e2d1f0038bed5", [:make, :rebar3], [{:cowlib, "2.11.0", [hex: :cowlib, repo: "hexpm", optional: false]}, {:ranch, "1.8.0", [hex: :ranch, repo: "hexpm", optional: false]}], "hexpm", "2c729f934b4e1aa149aff882f57c6372c15399a20d54f65c8d67bef583021bde"},
  "phoenix": {:hex, :phoenix, "1.6.15", "9f1d6b2acf82b5bc235496a8ab7e4a44a09a11a98ab7c6c47e69d673c53f8fe3", [:mix], [{:jason, "~> 1.0", [hex: :jason, repo: "hexpm", optional: true]}], "hexpm", "hash"},
  "plug": {:hex, :plug, "1.14.0", "hashvalue", [:mix], [], "hexpm", "hash2"},
}
"#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();

        let result = parse_mix_lock(file.path()).unwrap();
        assert_eq!(result.ecosystem, "Hex");
        assert_eq!(result.dependencies.len(), 3);

        let phoenix = result.dependencies.iter().find(|d| d.name == "phoenix").unwrap();
        assert_eq!(phoenix.version, "1.6.15");

        let cowboy = result.dependencies.iter().find(|d| d.name == "cowboy").unwrap();
        assert_eq!(cowboy.version, "2.9.0");
    }

    #[test]
    fn test_parse_mix_lock_line() {
        let line = r#"  "phoenix": {:hex, :phoenix, "1.6.15", "hash", [:mix], [], "hexpm", "hash"},"#;
        let dep = parse_mix_lock_line(line).unwrap();
        assert_eq!(dep.name, "phoenix");
        assert_eq!(dep.version, "1.6.15");
    }
}
