import { getStatusColor } from "../../types/rfc";

interface RfcStatusBadgeProps {
  status: string;
}

export function RfcStatusBadge({ status }: RfcStatusBadgeProps) {
  const color = getStatusColor(status);

  // Format status for display
  const displayStatus = status
    .split(" ")
    .map((word) => word.charAt(0) + word.slice(1).toLowerCase())
    .join(" ");

  return (
    <span
      className="rfc-status-badge"
      style={{
        backgroundColor: `${color}20`,
        color: color,
        borderColor: color,
      }}
    >
      {displayStatus}
    </span>
  );
}

