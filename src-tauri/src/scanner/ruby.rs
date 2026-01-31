//! RubyGems 依存関係パーサー

use super::{Dependency, ScanDependencies, ScanError};
use std::fs;
use std::path::Path;

/// Gemfile.lock をパース
pub fn parse_gemfile_lock(path: &Path) -> Result<ScanDependencies, ScanError> {
    let content = fs::read_to_string(path)?;
    let mut dependencies = Vec::new();

    let mut in_specs_section = false;

    for line in content.lines() {
        // Detect GEM section with specs
        if line.trim() == "GEM" {
            continue;
        }

        // Detect specs section (indented with 2 spaces under GEM)
        if line == "  specs:" {
            in_specs_section = true;
            continue;
        }

        // New section starts (no leading spaces or different section)
        if !line.starts_with(' ') && !line.is_empty() {
            in_specs_section = false;
            continue;
        }

        // Parse gem entries in specs section
        // Format: "    gem_name (version)"
        if in_specs_section && line.starts_with("    ") && !line.starts_with("      ") {
            let trimmed = line.trim();
            if let Some(dep) = parse_gem_spec(trimmed) {
                dependencies.push(dep);
            }
        }
    }

    Ok(ScanDependencies {
        ecosystem: "RubyGems".to_string(),
        source_file: path.to_string_lossy().to_string(),
        dependencies,
    })
}

/// gem spec 行をパース
/// "rails (7.0.4)" -> Dependency { name: "rails", version: "7.0.4" }
fn parse_gem_spec(line: &str) -> Option<Dependency> {
    // Find the opening parenthesis
    if let Some(paren_start) = line.find('(') {
        if let Some(paren_end) = line.find(')') {
            let name = line[..paren_start].trim();
            let version = &line[paren_start + 1..paren_end];

            if !name.is_empty() && !version.is_empty() {
                return Some(Dependency {
                    name: name.to_string(),
                    version: version.to_string(),
                    ecosystem: "RubyGems".to_string(),
                });
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_gemfile_lock() {
        let content = r#"GEM
  remote: https://rubygems.org/
  specs:
    actioncable (7.0.4)
      actionpack (= 7.0.4)
    actionpack (7.0.4)
      rack (~> 2.2, >= 2.2.0)
    rails (7.0.4)
      actioncable (= 7.0.4)
    rack (2.2.4)

PLATFORMS
  ruby

DEPENDENCIES
  rails (~> 7.0.4)

BUNDLED WITH
   2.3.26
"#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();

        let result = parse_gemfile_lock(file.path()).unwrap();
        assert_eq!(result.ecosystem, "RubyGems");
        assert_eq!(result.dependencies.len(), 4);

        let names: Vec<&str> = result.dependencies.iter().map(|d| d.name.as_str()).collect();
        assert!(names.contains(&"rails"));
        assert!(names.contains(&"rack"));
    }

    #[test]
    fn test_parse_gem_spec() {
        let dep = parse_gem_spec("rails (7.0.4)").unwrap();
        assert_eq!(dep.name, "rails");
        assert_eq!(dep.version, "7.0.4");

        let dep2 = parse_gem_spec("nokogiri (1.13.10-x86_64-linux)").unwrap();
        assert_eq!(dep2.name, "nokogiri");
        assert_eq!(dep2.version, "1.13.10-x86_64-linux");
    }
}
