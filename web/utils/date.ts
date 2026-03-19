/**
 * Shared date/time formatting utilities — Vietnam timezone (UTC+7)
 * Uses manual offset to ensure correct time in SSR (Docker/Node) environments
 */
const VN_OFFSET_MS = 7 * 60 * 60 * 1000 // UTC+7

function toVN(s: string | null | undefined): Date | null {
  if (!s) return null
  // Normalize: API returns timestamps without timezone (e.g. "2026-03-16 04:22:28.559752")
  // Ensure parsed as UTC by adding T separator and Z suffix if missing
  let iso = s.replace(' ', 'T')
  if (!/[Zz+\-]\d{0,4}$/.test(iso)) iso += 'Z'
  const d = new Date(iso)
  if (isNaN(d.getTime())) return null
  return new Date(d.getTime() + VN_OFFSET_MS)
}

function pad(n: number): string {
  return n < 10 ? '0' + n : '' + n
}

/** Full datetime: "11:21 16/03/2026" */
export function fmtDateTime(s: string | null | undefined): string {
  const d = toVN(s)
  if (!d) return '—'
  return `${pad(d.getUTCHours())}:${pad(d.getUTCMinutes())} ${pad(d.getUTCDate())}/${pad(d.getUTCMonth() + 1)}/${d.getUTCFullYear()}`
}

/** Date only: "16/03/2026" */
export function fmtDateOnly(s: string | null | undefined): string {
  const d = toVN(s)
  if (!d) return '—'
  return `${pad(d.getUTCDate())}/${pad(d.getUTCMonth() + 1)}/${d.getUTCFullYear()}`
}

/** Today in Vietnam TZ as YYYY-MM-DD (for date inputs) */
export function todayVN(): string {
  const d = new Date(Date.now() + VN_OFFSET_MS)
  return `${d.getUTCFullYear()}-${pad(d.getUTCMonth() + 1)}-${pad(d.getUTCDate())}`
}

/** First day of current month in VN TZ as YYYY-MM-DD */
export function firstOfMonthVN(): string {
  const d = new Date(Date.now() + VN_OFFSET_MS)
  return `${d.getUTCFullYear()}-${pad(d.getUTCMonth() + 1)}-01`
}
