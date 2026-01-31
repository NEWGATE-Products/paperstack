//! 依存関係スキャナーモジュール
//! 各パッケージマネージャーのロックファイルを解析し、依存関係を抽出

pub mod cargo;
pub mod cocoapods;
pub mod dart;
pub mod elixir;
pub mod go;
pub mod maven;
pub mod npm;
pub mod nuget;
pub mod php;
pub mod pip;
pub mod ruby;
pub mod swift;

use serde::{Deserialize, Serialize};
use std::path::Path;

/// 検出した依存関係
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub ecosystem: String,
}

/// スキャン結果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanDependencies {
    pub ecosystem: String,
    pub source_file: String,
    pub dependencies: Vec<Dependency>,
}

/// エコシステムの種類
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Ecosystem {
    Npm,
    Cargo,
    PyPI,
    Go,
    Maven,
    NuGet,
    RubyGems,
    Packagist,
    Pub,
    Hex,
    CocoaPods,
    SwiftURL,
}

#[allow(dead_code)]
impl Ecosystem {
    /// OSV API用のエコシステム名を取得
    pub fn osv_name(&self) -> &'static str {
        match self {
            Ecosystem::Npm => "npm",
            Ecosystem::Cargo => "crates.io",
            Ecosystem::PyPI => "PyPI",
            Ecosystem::Go => "Go",
            Ecosystem::Maven => "Maven",
            Ecosystem::NuGet => "NuGet",
            Ecosystem::RubyGems => "RubyGems",
            Ecosystem::Packagist => "Packagist",
            Ecosystem::Pub => "Pub",
            Ecosystem::Hex => "Hex",
            Ecosystem::CocoaPods => "CocoaPods",
            Ecosystem::SwiftURL => "SwiftURL",
        }
    }

    /// 表示用の名前を取得
    pub fn display_name(&self) -> &'static str {
        match self {
            Ecosystem::Npm => "npm",
            Ecosystem::Cargo => "Cargo",
            Ecosystem::PyPI => "pip",
            Ecosystem::Go => "Go",
            Ecosystem::Maven => "Maven/Gradle",
            Ecosystem::NuGet => "NuGet",
            Ecosystem::RubyGems => "RubyGems",
            Ecosystem::Packagist => "Composer",
            Ecosystem::Pub => "Pub",
            Ecosystem::Hex => "Hex",
            Ecosystem::CocoaPods => "CocoaPods",
            Ecosystem::SwiftURL => "SwiftPM",
        }
    }
}

impl std::fmt::Display for Ecosystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.osv_name())
    }
}

/// ディレクトリ内のすべての依存関係ファイルをスキャン
pub fn scan_directory(dir_path: &Path) -> Result<Vec<ScanDependencies>, ScanError> {
    let mut results = Vec::new();

    // npm / pnpm
    let package_lock = dir_path.join("package-lock.json");
    if package_lock.exists() {
        match npm::parse_package_lock(&package_lock) {
            Ok(deps) => results.push(deps),
            Err(e) => eprintln!("Warning: Failed to parse package-lock.json: {}", e),
        }
    }

    let pnpm_lock = dir_path.join("pnpm-lock.yaml");
    if pnpm_lock.exists() {
        match npm::parse_pnpm_lock(&pnpm_lock) {
            Ok(deps) => results.push(deps),
            Err(e) => eprintln!("Warning: Failed to parse pnpm-lock.yaml: {}", e),
        }
    }

    // yarn
    let yarn_lock = dir_path.join("yarn.lock");
    if yarn_lock.exists() {
        match npm::parse_yarn_lock(&yarn_lock) {
            Ok(deps) => results.push(deps),
            Err(e) => eprintln!("Warning: Failed to parse yarn.lock: {}", e),
        }
    }

    // Cargo
    let cargo_lock = dir_path.join("Cargo.lock");
    if cargo_lock.exists() {
        match cargo::parse_cargo_lock(&cargo_lock) {
            Ok(deps) => results.push(deps),
            Err(e) => eprintln!("Warning: Failed to parse Cargo.lock: {}", e),
        }
    }

    // pip
    let requirements = dir_path.join("requirements.txt");
    if requirements.exists() {
        match pip::parse_requirements(&requirements) {
            Ok(deps) => results.push(deps),
            Err(e) => eprintln!("Warning: Failed to parse requirements.txt: {}", e),
        }
    }

    let poetry_lock = dir_path.join("poetry.lock");
    if poetry_lock.exists() {
        match pip::parse_poetry_lock(&poetry_lock) {
            Ok(deps) => results.push(deps),
            Err(e) => eprintln!("Warning: Failed to parse poetry.lock: {}", e),
        }
    }

    let pipfile_lock = dir_path.join("Pipfile.lock");
    if pipfile_lock.exists() {
        match pip::parse_pipfile_lock(&pipfile_lock) {
            Ok(deps) => results.push(deps),
            Err(e) => eprintln!("Warning: Failed to parse Pipfile.lock: {}", e),
        }
    }

    // Go
    let go_sum = dir_path.join("go.sum");
    if go_sum.exists() {
        match go::parse_go_sum(&go_sum) {
            Ok(deps) => results.push(deps),
            Err(e) => eprintln!("Warning: Failed to parse go.sum: {}", e),
        }
    }

    // Maven / Gradle
    let gradle_lock = dir_path.join("gradle.lockfile");
    if gradle_lock.exists() {
        match maven::parse_gradle_lockfile(&gradle_lock) {
            Ok(deps) => results.push(deps),
            Err(e) => eprintln!("Warning: Failed to parse gradle.lockfile: {}", e),
        }
    }

    let pom_xml = dir_path.join("pom.xml");
    if pom_xml.exists() {
        match maven::parse_pom_xml(&pom_xml) {
            Ok(deps) => results.push(deps),
            Err(e) => eprintln!("Warning: Failed to parse pom.xml: {}", e),
        }
    }

    // NuGet
    let packages_lock = dir_path.join("packages.lock.json");
    if packages_lock.exists() {
        match nuget::parse_packages_lock(&packages_lock) {
            Ok(deps) => results.push(deps),
            Err(e) => eprintln!("Warning: Failed to parse packages.lock.json: {}", e),
        }
    }

    // RubyGems
    let gemfile_lock = dir_path.join("Gemfile.lock");
    if gemfile_lock.exists() {
        match ruby::parse_gemfile_lock(&gemfile_lock) {
            Ok(deps) => results.push(deps),
            Err(e) => eprintln!("Warning: Failed to parse Gemfile.lock: {}", e),
        }
    }

    // Packagist (Composer)
    let composer_lock = dir_path.join("composer.lock");
    if composer_lock.exists() {
        match php::parse_composer_lock(&composer_lock) {
            Ok(deps) => results.push(deps),
            Err(e) => eprintln!("Warning: Failed to parse composer.lock: {}", e),
        }
    }

    // Pub (Dart/Flutter)
    let pubspec_lock = dir_path.join("pubspec.lock");
    if pubspec_lock.exists() {
        match dart::parse_pubspec_lock(&pubspec_lock) {
            Ok(deps) => results.push(deps),
            Err(e) => eprintln!("Warning: Failed to parse pubspec.lock: {}", e),
        }
    }

    // Hex (Elixir)
    let mix_lock = dir_path.join("mix.lock");
    if mix_lock.exists() {
        match elixir::parse_mix_lock(&mix_lock) {
            Ok(deps) => results.push(deps),
            Err(e) => eprintln!("Warning: Failed to parse mix.lock: {}", e),
        }
    }

    // CocoaPods
    let podfile_lock = dir_path.join("Podfile.lock");
    if podfile_lock.exists() {
        match cocoapods::parse_podfile_lock(&podfile_lock) {
            Ok(deps) => results.push(deps),
            Err(e) => eprintln!("Warning: Failed to parse Podfile.lock: {}", e),
        }
    }

    // SwiftPM
    let package_resolved = dir_path.join("Package.resolved");
    if package_resolved.exists() {
        match swift::parse_package_resolved(&package_resolved) {
            Ok(deps) => results.push(deps),
            Err(e) => eprintln!("Warning: Failed to parse Package.resolved: {}", e),
        }
    }

    if results.is_empty() {
        return Err(ScanError::NoDependencyFiles);
    }

    Ok(results)
}

/// スキャンエラー
#[derive(Debug, thiserror::Error)]
pub enum ScanError {
    #[error("No dependency files found")]
    NoDependencyFiles,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(String),
}
