// RFC Types for RFCかんたんリーダー

/** RFCステータス */
export type RfcStatus =
  | "INTERNET STANDARD"
  | "PROPOSED STANDARD"
  | "DRAFT STANDARD"
  | "BEST CURRENT PRACTICE"
  | "INFORMATIONAL"
  | "EXPERIMENTAL"
  | "HISTORIC"
  | "UNKNOWN";

/** 要約レベル */
export type SummaryLevel = "easy" | "normal" | "technical";

/** RFC基本情報 */
export interface Rfc {
  id: string; // "RFC9114"
  number: number; // 9114
  title: string;
  abstract: string | null;
  status: RfcStatus | string;
  publishedDate: string | null; // "2022-06"
  authors: string[];
  keywords: string[];
  categories: string[];

  // AI生成コンテンツ
  summaryEasy: string | null;
  summaryNormal: string | null;
  summaryTechnical: string | null;
  implementationGuide: string | null;
  titleJa: string | null;
  abstractJa: string | null;

  // UI状態
  isBookmarked: boolean;
}

/** RFCフィルター条件 */
export interface RfcFilter {
  search?: string;
  rfcNumber?: number;
  status?: string[];
  categories?: string[];
  yearFrom?: number;
  yearTo?: number;
}

/** RFC一覧レスポンス */
export interface RfcListResponse {
  rfcs: Rfc[];
  total: number;
  page: number;
  limit: number;
}

/** RFCブックマーク */
export interface RfcBookmark {
  rfcId: string;
  memo: string | null;
  createdAt: string;
}

/** RFC閲覧履歴 */
export interface RfcHistory {
  rfcId: string;
  viewedAt: string;
}

/** RFCカテゴリ定義 */
export interface RfcCategoryDef {
  id: string;
  name: string;
  keywords: string[];
}

/** 定義済みRFCカテゴリ */
export const RFC_CATEGORIES: RfcCategoryDef[] = [
  { id: "http", name: "HTTP", keywords: ["http", "web", "uri", "url", "html"] },
  { id: "dns", name: "DNS", keywords: ["dns", "domain", "resolver"] },
  {
    id: "email",
    name: "メール",
    keywords: ["smtp", "imap", "pop", "email", "mail"],
  },
  {
    id: "security",
    name: "セキュリティ",
    keywords: ["tls", "ssl", "security", "crypto", "certificate"],
  },
  {
    id: "routing",
    name: "ルーティング",
    keywords: ["bgp", "ospf", "routing", "router"],
  },
  { id: "ipv6", name: "IPv6", keywords: ["ipv6", "icmpv6"] },
  {
    id: "transport",
    name: "TCP/UDP",
    keywords: ["tcp", "udp", "transport", "quic"],
  },
  { id: "other", name: "その他", keywords: [] },
];

/** RFCステータス定義 */
export const RFC_STATUSES: { value: string; label: string }[] = [
  { value: "INTERNET STANDARD", label: "Internet Standard" },
  { value: "PROPOSED STANDARD", label: "Proposed Standard" },
  { value: "DRAFT STANDARD", label: "Draft Standard" },
  { value: "BEST CURRENT PRACTICE", label: "Best Current Practice" },
  { value: "INFORMATIONAL", label: "Informational" },
  { value: "EXPERIMENTAL", label: "Experimental" },
  { value: "HISTORIC", label: "Historic" },
];

/** ステータスの表示色を取得 */
export function getStatusColor(status: string): string {
  switch (status) {
    case "INTERNET STANDARD":
      return "#22c55e"; // green
    case "PROPOSED STANDARD":
      return "#3b82f6"; // blue
    case "DRAFT STANDARD":
      return "#8b5cf6"; // purple
    case "BEST CURRENT PRACTICE":
      return "#f59e0b"; // amber
    case "INFORMATIONAL":
      return "#6b7280"; // gray
    case "EXPERIMENTAL":
      return "#ec4899"; // pink
    case "HISTORIC":
      return "#9ca3af"; // light gray
    default:
      return "#6b7280";
  }
}

/** カテゴリ名を取得 */
export function getCategoryName(categoryId: string): string {
  const category = RFC_CATEGORIES.find((c) => c.id === categoryId);
  return category?.name ?? categoryId;
}

