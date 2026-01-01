/**
 * Flaticon-style icons for the Paper News app
 * Icons are inline SVGs for better performance and styling flexibility
 * Design inspiration from https://www.flaticon.com/
 */

interface IconProps {
  size?: number;
  className?: string;
  color?: string;
}

// 📚 Books / Papers icon
export function IconBooks({ size = 20, className = "", color: _color = "currentColor" }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      className={className}
      xmlns="http://www.w3.org/2000/svg"
    >
      <path d="M12 8h12v48H12a4 4 0 01-4-4V12a4 4 0 014-4z" fill="#4A90A4" />
      <path d="M24 8h12v48H24V8z" fill="#5BA4B8" />
      <path d="M36 8h12v48H36V8z" fill="#6BB8CC" />
      <path d="M48 8h4a4 4 0 014 4v40a4 4 0 01-4 4h-4V8z" fill="#7CCCE0" />
      <path d="M16 14h4v4h-4zM28 14h4v4h-4zM40 14h4v4h-4z" fill="#FFF" opacity="0.6" />
    </svg>
  );
}

// 📄 Document / RFC icon
export function IconDocument({ size = 20, className = "", color: _color = "currentColor" }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      className={className}
      xmlns="http://www.w3.org/2000/svg"
    >
      <path d="M12 4h28l12 12v40a4 4 0 01-4 4H12a4 4 0 01-4-4V8a4 4 0 014-4z" fill="#F5F5F5" />
      <path d="M40 4v12h12L40 4z" fill="#E0E0E0" />
      <path d="M16 24h24v2H16zM16 32h24v2H16zM16 40h16v2H16z" fill="#9E9E9E" />
      <path d="M12 4h28l12 12v40a4 4 0 01-4 4H12a4 4 0 01-4-4V8a4 4 0 014-4z" stroke="#BDBDBD" strokeWidth="1" fill="none" />
    </svg>
  );
}

// 🔍 Search icon
export function IconSearch({ size = 20, className = "", color = "currentColor" }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      className={className}
      xmlns="http://www.w3.org/2000/svg"
    >
      <circle cx="28" cy="28" r="16" stroke={color} strokeWidth="6" fill="none" />
      <path d="M40 40l16 16" stroke={color} strokeWidth="6" strokeLinecap="round" />
    </svg>
  );
}

// ⭐ Star (filled) icon
export function IconStarFilled({ size = 20, className = "", color = "#F59E0B" }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      className={className}
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        d="M32 4l8.5 17.2 19 2.8-13.7 13.4 3.2 18.6L32 46.5 14.9 56l3.2-18.6L4.5 24l19-2.8L32 4z"
        fill={color}
      />
    </svg>
  );
}

// ☆ Star (empty) icon
export function IconStarEmpty({ size = 20, className = "", color = "currentColor" }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      className={className}
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        d="M32 4l8.5 17.2 19 2.8-13.7 13.4 3.2 18.6L32 46.5 14.9 56l3.2-18.6L4.5 24l19-2.8L32 4z"
        stroke={color}
        strokeWidth="3"
        fill="none"
      />
    </svg>
  );
}

// 🔄 Refresh / Sync icon
export function IconRefresh({ size = 20, className = "", color = "currentColor" }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      className={className}
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        d="M32 8A24 24 0 0156 32"
        stroke={color}
        strokeWidth="5"
        strokeLinecap="round"
        fill="none"
      />
      <path
        d="M32 56A24 24 0 018 32"
        stroke={color}
        strokeWidth="5"
        strokeLinecap="round"
        fill="none"
      />
      <path d="M52 16l4-8v8h-8" fill={color} />
      <path d="M12 48l-4 8v-8h8" fill={color} />
    </svg>
  );
}

// ⚙️ Settings / Gear icon
export function IconSettings({ size = 20, className = "", color = "currentColor" }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      className={className}
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        d="M32 20a12 12 0 100 24 12 12 0 000-24z"
        stroke={color}
        strokeWidth="4"
        fill="none"
      />
      <path
        d="M32 4v8M32 52v8M4 32h8M52 32h8M12.5 12.5l5.7 5.7M45.8 45.8l5.7 5.7M12.5 51.5l5.7-5.7M45.8 18.2l5.7-5.7"
        stroke={color}
        strokeWidth="4"
        strokeLinecap="round"
      />
    </svg>
  );
}

// 📅 Calendar / Date icon
export function IconCalendar({ size = 20, className = "", color = "currentColor" }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      className={className}
      xmlns="http://www.w3.org/2000/svg"
    >
      <rect x="8" y="12" width="48" height="44" rx="4" fill="#FF6B6B" />
      <rect x="8" y="24" width="48" height="32" rx="0" fill="#FFF" />
      <path d="M16 4v12M48 4v12" stroke={color} strokeWidth="4" strokeLinecap="round" />
      <rect x="16" y="32" width="8" height="8" fill="#E0E0E0" rx="1" />
      <rect x="28" y="32" width="8" height="8" fill="#E0E0E0" rx="1" />
      <rect x="40" y="32" width="8" height="8" fill="#E0E0E0" rx="1" />
      <rect x="16" y="44" width="8" height="8" fill="#E0E0E0" rx="1" />
      <rect x="28" y="44" width="8" height="8" fill="#E0E0E0" rx="1" />
    </svg>
  );
}

// 👤 User / Person icon
export function IconUser({ size = 20, className = "", color: _color = "currentColor" }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      className={className}
      xmlns="http://www.w3.org/2000/svg"
    >
      <circle cx="32" cy="20" r="12" fill="#6B7280" />
      <path d="M8 56c0-13.3 10.7-24 24-24s24 10.7 24 24" fill="#6B7280" />
    </svg>
  );
}

// ⚠️ Warning icon
export function IconWarning({ size = 20, className = "", color: _color = "#DC2626" }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      className={className}
      xmlns="http://www.w3.org/2000/svg"
    >
      <path d="M32 4L4 56h56L32 4z" fill="#FCD34D" stroke="#F59E0B" strokeWidth="2" />
      <path d="M32 24v16" stroke="#92400E" strokeWidth="4" strokeLinecap="round" />
      <circle cx="32" cy="48" r="3" fill="#92400E" />
    </svg>
  );
}

// 💡 Lightbulb / Idea icon
export function IconLightbulb({ size = 20, className = "", color: _color = "currentColor" }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      className={className}
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        d="M32 4C20.9 4 12 12.9 12 24c0 7.4 4 13.9 10 17.4V48c0 2.2 1.8 4 4 4h12c2.2 0 4-1.8 4-4v-6.6c6-3.5 10-10 10-17.4 0-11.1-8.9-20-20-20z"
        fill="#FCD34D"
      />
      <path d="M24 52h16v4c0 2.2-1.8 4-4 4h-8c-2.2 0-4-1.8-4-4v-4z" fill="#9CA3AF" />
      <path d="M26 40h12M26 44h12" stroke="#F59E0B" strokeWidth="2" />
    </svg>
  );
}

// 📝 Edit / Pencil icon (also for 一般 summary level)
export function IconEdit({ size = 20, className = "", color = "currentColor" }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      className={className}
      xmlns="http://www.w3.org/2000/svg"
    >
      <path d="M8 56h48" stroke={color} strokeWidth="4" strokeLinecap="round" />
      <path
        d="M44 8l12 12-28 28H16V36L44 8z"
        fill="#FCD34D"
        stroke={color}
        strokeWidth="3"
      />
      <path d="M40 12l12 12" stroke={color} strokeWidth="3" />
    </svg>
  );
}

// 🔧 Wrench / Technical icon
export function IconWrench({ size = 20, className = "", color: _color = "currentColor" }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      className={className}
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        d="M48 12a16 16 0 00-22.6 22.6L12 48l4 4 13.4-13.4A16 16 0 0048 12z"
        fill="#6B7280"
      />
      <path d="M44 8l12 4-4 12" stroke="#6B7280" strokeWidth="4" fill="none" />
    </svg>
  );
}

// 🎈 Balloon / Easy mode icon
export function IconBalloon({ size = 20, className = "", color: _color = "currentColor" }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      className={className}
      xmlns="http://www.w3.org/2000/svg"
    >
      <ellipse cx="32" cy="24" rx="16" ry="20" fill="#FF6B6B" />
      <path d="M28 44c0 0 4 4 4 12" stroke="#9CA3AF" strokeWidth="2" fill="none" />
      <path d="M28 44l8 0" stroke="#6B7280" strokeWidth="2" />
      <ellipse cx="26" cy="18" rx="4" ry="6" fill="#FFF" opacity="0.3" />
    </svg>
  );
}

// 💻 Computer / Code icon
export function IconComputer({ size = 20, className = "", color: _color = "currentColor" }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      className={className}
      xmlns="http://www.w3.org/2000/svg"
    >
      <rect x="4" y="8" width="56" height="36" rx="4" fill="#374151" />
      <rect x="8" y="12" width="48" height="28" fill="#10B981" />
      <path d="M16 20l8 8-8 8M28 36h12" stroke="#FFF" strokeWidth="3" strokeLinecap="round" />
      <path d="M24 52h16v4H24z" fill="#6B7280" />
      <rect x="16" y="56" width="32" height="4" rx="2" fill="#9CA3AF" />
    </svg>
  );
}

// 🏷️ Tag / Label icon
export function IconTag({ size = 20, className = "", color: _color = "currentColor" }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      className={className}
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        d="M4 8v20l28 28 20-20L24 8H4z"
        fill="#4ECDC4"
        stroke="#3DB9AC"
        strokeWidth="2"
      />
      <circle cx="16" cy="20" r="4" fill="#FFF" />
    </svg>
  );
}

// 🔗 Link icon
export function IconLink({ size = 20, className = "", color = "currentColor" }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      className={className}
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        d="M24 40l-8 8a8 8 0 01-11.3-11.3l8-8"
        stroke={color}
        strokeWidth="5"
        strokeLinecap="round"
        fill="none"
      />
      <path
        d="M40 24l8-8a8 8 0 0111.3 11.3l-8 8"
        stroke={color}
        strokeWidth="5"
        strokeLinecap="round"
        fill="none"
      />
      <path d="M24 40l16-16" stroke={color} strokeWidth="5" strokeLinecap="round" />
    </svg>
  );
}

// 📖 Open Book / Read icon
export function IconBook({ size = 20, className = "", color: _color = "currentColor" }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      className={className}
      xmlns="http://www.w3.org/2000/svg"
    >
      <path d="M32 12v44" stroke="#6B7280" strokeWidth="2" />
      <path d="M32 12c-8-4-16-4-24 0v40c8-4 16-4 24 0" fill="#4A90A4" />
      <path d="M32 12c8-4 16-4 24 0v40c-8-4-16-4-24 0" fill="#5BA4B8" />
      <path d="M16 20h8M16 28h10M16 36h8" stroke="#FFF" strokeWidth="2" opacity="0.6" />
      <path d="M40 20h8M40 28h10M40 36h8" stroke="#FFF" strokeWidth="2" opacity="0.6" />
    </svg>
  );
}

// 🗑️ Trash / Delete icon
export function IconTrash({ size = 20, className = "", color: _color = "currentColor" }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      className={className}
      xmlns="http://www.w3.org/2000/svg"
    >
      <path d="M8 16h48" stroke="#DC2626" strokeWidth="4" strokeLinecap="round" />
      <path d="M24 16V8h16v8" stroke="#DC2626" strokeWidth="4" fill="none" />
      <path d="M12 16l4 40h32l4-40" fill="#FEE2E2" stroke="#DC2626" strokeWidth="3" />
      <path d="M24 24v24M32 24v24M40 24v24" stroke="#DC2626" strokeWidth="2" />
    </svg>
  );
}

// ❌ Error / Cross icon
export function IconError({ size = 20, className = "", color = "#DC2626" }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      className={className}
      xmlns="http://www.w3.org/2000/svg"
    >
      <circle cx="32" cy="32" r="28" fill="#FEE2E2" stroke={color} strokeWidth="3" />
      <path d="M20 20l24 24M44 20L20 44" stroke={color} strokeWidth="5" strokeLinecap="round" />
    </svg>
  );
}

// ✨ Sparkle / Magic icon
export function IconSparkle({ size = 20, className = "", color = "#F59E0B" }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      className={className}
      xmlns="http://www.w3.org/2000/svg"
    >
      <path d="M32 4l4 12 12 4-12 4-4 12-4-12-12-4 12-4 4-12z" fill={color} />
      <path d="M12 32l2 6 6 2-6 2-2 6-2-6-6-2 6-2 2-6z" fill={color} opacity="0.7" />
      <path d="M48 40l2 6 6 2-6 2-2 6-2-6-6-2 6-2 2-6z" fill={color} opacity="0.5" />
    </svg>
  );
}

// 📰 News / Newspaper icon
export function IconNews({ size = 20, className = "", color: _color = "currentColor" }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      className={className}
      xmlns="http://www.w3.org/2000/svg"
    >
      <rect x="4" y="8" width="48" height="48" rx="4" fill="#F5F5F5" stroke="#BDBDBD" strokeWidth="2" />
      <rect x="56" y="16" width="4" height="40" rx="2" fill="#E0E0E0" />
      <rect x="12" y="16" width="20" height="12" fill="#4A90A4" />
      <path d="M12 32h32v2H12zM12 38h28v2H12zM12 44h32v2H12z" fill="#9E9E9E" />
    </svg>
  );
}

// 🤖 Robot / AI icon
export function IconRobot({ size = 20, className = "", color: _color = "currentColor" }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      className={className}
      xmlns="http://www.w3.org/2000/svg"
    >
      <rect x="12" y="16" width="40" height="36" rx="4" fill="#6B7280" />
      <circle cx="24" cy="32" r="6" fill="#10B981" />
      <circle cx="40" cy="32" r="6" fill="#10B981" />
      <rect x="20" y="44" width="24" height="4" rx="2" fill="#374151" />
      <path d="M32 4v12" stroke="#6B7280" strokeWidth="4" />
      <circle cx="32" cy="4" r="4" fill="#F59E0B" />
      <rect x="4" y="28" width="8" height="12" rx="2" fill="#6B7280" />
      <rect x="52" y="28" width="8" height="12" rx="2" fill="#6B7280" />
    </svg>
  );
}

// 💬 Chat / LLM icon
export function IconChat({ size = 20, className = "", color: _color = "currentColor" }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      className={className}
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        d="M8 12h40a4 4 0 014 4v24a4 4 0 01-4 4H24l-12 12V44H8a4 4 0 01-4-4V16a4 4 0 014-4z"
        fill="#4A90A4"
      />
      <circle cx="20" cy="28" r="3" fill="#FFF" />
      <circle cx="32" cy="28" r="3" fill="#FFF" />
      <circle cx="44" cy="28" r="3" fill="#FFF" />
    </svg>
  );
}

// 🧮 Calculator / Algorithm icon
export function IconCalculator({ size = 20, className = "", color: _color = "currentColor" }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      className={className}
      xmlns="http://www.w3.org/2000/svg"
    >
      <rect x="8" y="4" width="48" height="56" rx="4" fill="#374151" />
      <rect x="12" y="8" width="40" height="16" rx="2" fill="#10B981" />
      <rect x="12" y="28" width="10" height="10" rx="2" fill="#6B7280" />
      <rect x="27" y="28" width="10" height="10" rx="2" fill="#6B7280" />
      <rect x="42" y="28" width="10" height="10" rx="2" fill="#6B7280" />
      <rect x="12" y="42" width="10" height="10" rx="2" fill="#6B7280" />
      <rect x="27" y="42" width="10" height="10" rx="2" fill="#6B7280" />
      <rect x="42" y="42" width="10" height="10" rx="2" fill="#F59E0B" />
    </svg>
  );
}

// 🏗️ Building / Architecture icon
export function IconBuilding({ size = 20, className = "", color: _color = "currentColor" }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      className={className}
      xmlns="http://www.w3.org/2000/svg"
    >
      <rect x="8" y="24" width="24" height="36" fill="#6B7280" />
      <rect x="32" y="8" width="24" height="52" fill="#9CA3AF" />
      <rect x="12" y="28" width="6" height="8" fill="#FCD34D" />
      <rect x="22" y="28" width="6" height="8" fill="#FCD34D" />
      <rect x="12" y="40" width="6" height="8" fill="#FCD34D" />
      <rect x="22" y="40" width="6" height="8" fill="#FCD34D" />
      <rect x="36" y="12" width="6" height="8" fill="#FCD34D" />
      <rect x="46" y="12" width="6" height="8" fill="#FCD34D" />
      <rect x="36" y="24" width="6" height="8" fill="#FCD34D" />
      <rect x="46" y="24" width="6" height="8" fill="#FCD34D" />
      <rect x="36" y="36" width="6" height="8" fill="#FCD34D" />
      <rect x="46" y="36" width="6" height="8" fill="#FCD34D" />
      <rect x="36" y="48" width="6" height="8" fill="#FCD34D" />
      <rect x="46" y="48" width="6" height="8" fill="#FCD34D" />
    </svg>
  );
}

// 📭 Empty mailbox icon
export function IconMailbox({ size = 20, className = "", color: _color = "currentColor" }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      className={className}
      xmlns="http://www.w3.org/2000/svg"
    >
      <path d="M8 24h48v28a4 4 0 01-4 4H12a4 4 0 01-4-4V24z" fill="#6B7280" />
      <path d="M8 24c0-8.8 7.2-16 16-16h16c8.8 0 16 7.2 16 16" fill="#9CA3AF" />
      <rect x="28" y="32" width="8" height="12" rx="2" fill="#374151" />
      <path d="M32 8v-4" stroke="#F59E0B" strokeWidth="3" strokeLinecap="round" />
    </svg>
  );
}

// 🔮 Crystal ball / AI Generate icon
export function IconCrystalBall({ size = 20, className = "", color: _color = "currentColor" }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      className={className}
      xmlns="http://www.w3.org/2000/svg"
    >
      <circle cx="32" cy="28" r="24" fill="url(#crystalGradient)" />
      <ellipse cx="24" cy="20" rx="8" ry="4" fill="#FFF" opacity="0.3" />
      <path d="M16 52h32l4 8H12l4-8z" fill="#6B7280" />
      <defs>
        <linearGradient id="crystalGradient" x1="8" y1="4" x2="56" y2="52" gradientUnits="userSpaceOnUse">
          <stop stopColor="#818CF8" />
          <stop offset="1" stopColor="#6366F1" />
        </linearGradient>
      </defs>
    </svg>
  );
}

// 🌐 Globe / Translate icon
export function IconGlobe({ size = 20, className = "", color: _color = "currentColor" }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      className={className}
      xmlns="http://www.w3.org/2000/svg"
    >
      <circle cx="32" cy="32" r="28" fill="#4A90A4" />
      <ellipse cx="32" cy="32" rx="12" ry="28" stroke="#FFF" strokeWidth="2" fill="none" />
      <path d="M4 32h56" stroke="#FFF" strokeWidth="2" />
      <path d="M8 20h48M8 44h48" stroke="#FFF" strokeWidth="1.5" opacity="0.7" />
    </svg>
  );
}

// 📌 Pin icon
export function IconPin({ size = 20, className = "", color: _color = "currentColor" }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      className={className}
      xmlns="http://www.w3.org/2000/svg"
    >
      <circle cx="32" cy="20" r="16" fill="#FF6B6B" />
      <path d="M32 36v24" stroke="#FF6B6B" strokeWidth="4" />
      <circle cx="32" cy="20" r="6" fill="#FFF" opacity="0.4" />
    </svg>
  );
}

// ▲ Expand up icon
export function IconChevronUp({ size = 20, className = "", color = "currentColor" }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      className={className}
      xmlns="http://www.w3.org/2000/svg"
    >
      <path d="M16 40l16-16 16 16" stroke={color} strokeWidth="5" strokeLinecap="round" strokeLinejoin="round" fill="none" />
    </svg>
  );
}

// ▼ Expand down icon
export function IconChevronDown({ size = 20, className = "", color = "currentColor" }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      className={className}
      xmlns="http://www.w3.org/2000/svg"
    >
      <path d="M16 24l16 16 16-16" stroke={color} strokeWidth="5" strokeLinecap="round" strokeLinejoin="round" fill="none" />
    </svg>
  );
}

// PDF document icon
export function IconPdf({ size = 20, className = "", color: _color = "currentColor" }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      className={className}
      xmlns="http://www.w3.org/2000/svg"
    >
      <path d="M12 4h28l12 12v40a4 4 0 01-4 4H12a4 4 0 01-4-4V8a4 4 0 014-4z" fill="#DC2626" />
      <path d="M40 4v12h12L40 4z" fill="#991B1B" />
      <text x="32" y="42" textAnchor="middle" fontSize="14" fontWeight="bold" fill="#FFF">PDF</text>
    </svg>
  );
}

