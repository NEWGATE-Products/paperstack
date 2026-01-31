import type { Severity } from "../../types/vuln";
import { getSeverityColor, getSeverityLabel } from "../../types/vuln";

interface SeverityBadgeProps {
  severity: Severity;
  showLabel?: boolean;
}

export function SeverityBadge({ severity, showLabel = true }: SeverityBadgeProps) {
  const color = getSeverityColor(severity);
  const label = getSeverityLabel(severity);

  return (
    <span
      className="severity-badge"
      style={{
        backgroundColor: `${color}20`,
        color: color,
        border: `1px solid ${color}40`,
      }}
    >
      {showLabel ? label : severity.toUpperCase()}
    </span>
  );
}
