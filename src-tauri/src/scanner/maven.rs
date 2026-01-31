//! Maven / Gradle 依存関係パーサー

use super::{Dependency, ScanDependencies, ScanError};
use std::fs;
use std::path::Path;

/// gradle.lockfile をパース
pub fn parse_gradle_lockfile(path: &Path) -> Result<ScanDependencies, ScanError> {
    let content = fs::read_to_string(path)?;
    let mut dependencies = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // Skip empty lines, comments, and metadata
        if trimmed.is_empty()
            || trimmed.starts_with('#')
            || trimmed.starts_with("empty=")
        {
            continue;
        }

        // Format: group:artifact:version=configuration
        // Example: com.google.guava:guava:31.1-jre=compileClasspath
        if let Some(equals_pos) = trimmed.find('=') {
            let coords = &trimmed[..equals_pos];
            if let Some(dep) = parse_maven_coordinates(coords) {
                dependencies.push(dep);
            }
        }
    }

    // Deduplicate
    dependencies.sort_by(|a, b| (&a.name, &a.version).cmp(&(&b.name, &b.version)));
    dependencies.dedup_by(|a, b| a.name == b.name && a.version == b.version);

    Ok(ScanDependencies {
        ecosystem: "Maven".to_string(),
        source_file: path.to_string_lossy().to_string(),
        dependencies,
    })
}

/// pom.xml から依存関係を抽出（簡易パーサー）
pub fn parse_pom_xml(path: &Path) -> Result<ScanDependencies, ScanError> {
    let content = fs::read_to_string(path)?;
    let mut dependencies = Vec::new();

    // Simple regex-free XML parsing for dependencies
    // Looking for <dependency>...</dependency> blocks
    let mut in_dependency = false;
    let mut current_group_id = String::new();
    let mut current_artifact_id = String::new();
    let mut current_version = String::new();
    let mut in_dependencies_section = false;

    for line in content.lines() {
        let trimmed = line.trim();

        // Track if we're in a <dependencies> section
        if trimmed.contains("<dependencies>") || trimmed.contains("<dependencies ") {
            in_dependencies_section = true;
        }
        if trimmed.contains("</dependencies>") {
            in_dependencies_section = false;
        }

        // Only parse dependencies within <dependencies> section
        if !in_dependencies_section {
            continue;
        }

        if trimmed.contains("<dependency>") || trimmed.contains("<dependency ") {
            in_dependency = true;
            current_group_id.clear();
            current_artifact_id.clear();
            current_version.clear();
        }

        if in_dependency {
            if let Some(value) = extract_xml_value(trimmed, "groupId") {
                current_group_id = value;
            }
            if let Some(value) = extract_xml_value(trimmed, "artifactId") {
                current_artifact_id = value;
            }
            if let Some(value) = extract_xml_value(trimmed, "version") {
                // Skip property references like ${project.version}
                if !value.starts_with('$') {
                    current_version = value;
                }
            }
        }

        if trimmed.contains("</dependency>") {
            in_dependency = false;
            if !current_group_id.is_empty()
                && !current_artifact_id.is_empty()
                && !current_version.is_empty()
            {
                dependencies.push(Dependency {
                    name: format!("{}:{}", current_group_id, current_artifact_id),
                    version: current_version.clone(),
                    ecosystem: "Maven".to_string(),
                });
            }
        }
    }

    Ok(ScanDependencies {
        ecosystem: "Maven".to_string(),
        source_file: path.to_string_lossy().to_string(),
        dependencies,
    })
}

/// Maven座標 (group:artifact:version) をパース
fn parse_maven_coordinates(coords: &str) -> Option<Dependency> {
    let parts: Vec<&str> = coords.split(':').collect();
    if parts.len() >= 3 {
        let group_id = parts[0];
        let artifact_id = parts[1];
        let version = parts[2];

        return Some(Dependency {
            name: format!("{}:{}", group_id, artifact_id),
            version: version.to_string(),
            ecosystem: "Maven".to_string(),
        });
    }
    None
}

/// XML タグから値を抽出
fn extract_xml_value(line: &str, tag: &str) -> Option<String> {
    let open_tag = format!("<{}>", tag);
    let close_tag = format!("</{}>", tag);

    if let Some(start) = line.find(&open_tag) {
        if let Some(end) = line.find(&close_tag) {
            let value_start = start + open_tag.len();
            if value_start < end {
                return Some(line[value_start..end].trim().to_string());
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
    fn test_parse_gradle_lockfile() {
        let content = r#"
# This is a Gradle generated file
# Manual edits can break the build and are not recommended.
com.google.guava:guava:31.1-jre=compileClasspath,runtimeClasspath
org.slf4j:slf4j-api:1.7.36=compileClasspath,runtimeClasspath
"#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();

        let result = parse_gradle_lockfile(file.path()).unwrap();
        assert_eq!(result.ecosystem, "Maven");
        assert_eq!(result.dependencies.len(), 2);
        assert_eq!(result.dependencies[0].name, "com.google.guava:guava");
        assert_eq!(result.dependencies[0].version, "31.1-jre");
    }

    #[test]
    fn test_parse_maven_coordinates() {
        let dep = parse_maven_coordinates("com.example:mylib:1.0.0").unwrap();
        assert_eq!(dep.name, "com.example:mylib");
        assert_eq!(dep.version, "1.0.0");
    }
}
