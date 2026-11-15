import Foundation

func check(_ condition: @autoclosure () -> Bool, _ message: String) {
    guard condition() else { fatalError(message) }
}
let codex = Data(#"{"rate_limit":{"primary_window":{"used_percent":21,"limit_window_seconds":604800,"reset_at":1800000000},"secondary_window":{"used_percent":42,"limit_window_seconds":18000,"reset_after_seconds":60}}}"#.utf8)
let usage = try Usage.parse(codex, provider: .codex, now: Date(timeIntervalSince1970: 1000))
check(usage.fiveHour?.used == 42, "Duration identifies the five-hour window")
check(usage.weekly?.used == 21, "Duration identifies the weekly window")
check(usage.fiveHour?.reset == Date(timeIntervalSince1970: 1060), "Relative reset time")
let empty = try Usage.parse(Data(#"{"rate_limit":{}}"#.utf8), provider: .codex)
check(empty.fiveHour == nil && empty.weekly == nil, "Missing windows stay unknown")
let claude = try Usage.parse(Data(#"{"five_hour":{"utilization":33,"resets_at":"2026-11-15T10:00:00Z"},"seven_day":{"utilization":55}}"#.utf8), provider: .claude)
check(claude.fiveHour?.used == 33 && claude.weekly?.used == 55, "Claude windows")
check(claude.fiveHour?.reset != nil, "ISO reset timestamp")
do {
    _ = try Usage.parse(Data(#"{"error":"unauthorized"}"#.utf8), provider: .codex)
    fatalError("An error response must never display zero")
} catch UsageError.badResponse { }
check(I18n.isChinese("zh-Hans-CN"), "Simplified Chinese detection")
check(!I18n.isChinese("zh-Hant-TW"), "Traditional Chinese keeps fallback")
check(!I18n.isChinese("en-US"), "English detection")
print("macOS usage parser and locale tests passed")
