// 脆弱性関連の型定義

/** 脆弱性の深刻度 */
export type Severity = "critical" | "high" | "medium" | "low";

/** データソース */
export type VulnSource = "osv" | "nvd" | "github";

/** エコシステム */
export type Ecosystem = 
  | "npm" 
  | "crates.io" 
  | "PyPI" 
  | "Go" 
  | "Maven" 
  | "NuGet" 
  | "RubyGems" 
  | "Packagist" 
  | "Pub" 
  | "Hex" 
  | "CocoaPods" 
  | "SwiftURL";

/** 脆弱性情報 */
export interface Vulnerability {
  id: string;
  source: VulnSource;
  severity: Severity;
  cvssScore: number | null;
  title: string;
  description: string | null;
  affectedPackage: string;
  affectedEcosystem: string;
  affectedVersions: string | null;
  fixedVersions: string | null;
  publishedAt: string | null;
  references: string[];
  fetchedAt: string | null;
}

/** 脆弱性フィルター */
export interface VulnFilter {
  ecosystem?: string;
  severity?: string;
  search?: string;
}

/** 脆弱性マッチ結果（スキャン時） */
export interface VulnMatch {
  packageName: string;
  installedVersion: string;
  vulnerability: Vulnerability;
}

/** スキャン結果 */
export interface ScanResult {
  directory: string;
  ecosystems: string[];
  vulnerabilities: VulnMatch[];
  scannedAt: string;
  totalPackages: number;
}

/** 脆弱性一覧レスポンス */
export interface VulnListResponse {
  vulnerabilities: Vulnerability[];
  total: number;
  page: number;
  limit: number;
}

/** スキャン履歴 */
export interface ScanHistory {
  id: number;
  directory: string;
  ecosystem: string;
  vulnCount: number;
  scannedAt: string;
}

/** 深刻度に対応する色を取得 */
export function getSeverityColor(severity: Severity): string {
  switch (severity) {
    case "critical":
      return "#dc2626"; // red-600
    case "high":
      return "#ea580c"; // orange-600
    case "medium":
      return "#ca8a04"; // yellow-600
    case "low":
      return "#16a34a"; // green-600
    default:
      return "#6b7280"; // gray-500
  }
}

/** 深刻度の日本語表示 */
export function getSeverityLabel(severity: Severity): string {
  switch (severity) {
    case "critical":
      return "緊急";
    case "high":
      return "重要";
    case "medium":
      return "警告";
    case "low":
      return "注意";
    default:
      return "不明";
  }
}

/** エコシステムの表示名 */
export function getEcosystemLabel(ecosystem: string): string {
  switch (ecosystem) {
    case "npm":
      return "npm";
    case "crates.io":
      return "Cargo (Rust)";
    case "PyPI":
      return "pip (Python)";
    case "Go":
      return "Go";
    case "Maven":
      return "Maven/Gradle (Java)";
    case "NuGet":
      return "NuGet (.NET)";
    case "RubyGems":
      return "RubyGems (Ruby)";
    case "Packagist":
      return "Composer (PHP)";
    case "Pub":
      return "Pub (Dart/Flutter)";
    case "Hex":
      return "Hex (Elixir)";
    case "CocoaPods":
      return "CocoaPods (iOS)";
    case "SwiftURL":
      return "SwiftPM (Swift)";
    default:
      return ecosystem;
  }
}

/** 利用可能なエコシステム一覧 */
export const ECOSYSTEMS: { id: Ecosystem; label: string }[] = [
  { id: "npm", label: "npm (JavaScript)" },
  { id: "crates.io", label: "Cargo (Rust)" },
  { id: "PyPI", label: "pip (Python)" },
  { id: "Go", label: "Go" },
  { id: "Maven", label: "Maven/Gradle (Java)" },
  { id: "NuGet", label: "NuGet (.NET)" },
  { id: "RubyGems", label: "RubyGems (Ruby)" },
  { id: "Packagist", label: "Composer (PHP)" },
  { id: "Pub", label: "Pub (Dart/Flutter)" },
  { id: "Hex", label: "Hex (Elixir)" },
  { id: "CocoaPods", label: "CocoaPods (iOS)" },
  { id: "SwiftURL", label: "SwiftPM (Swift)" },
];

/** 深刻度一覧 */
export const SEVERITIES: { id: Severity; label: string }[] = [
  { id: "critical", label: "緊急" },
  { id: "high", label: "重要" },
  { id: "medium", label: "警告" },
  { id: "low", label: "注意" },
];
