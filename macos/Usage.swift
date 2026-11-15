import Foundation

enum Provider: String, CaseIterable {
    case codex = "Codex"
    case claude = "Claude"
}

enum UsageError: Error {
    case noLogin, expired, rateLimited, badResponse, network
}

struct WindowUsage {
    let used: Double
    let reset: Date?
}

struct Usage {
    var fiveHour: WindowUsage?
    var weekly: WindowUsage?

    static func parse(_ data: Data, provider: Provider, now: Date = Date()) throws -> Usage {
        guard let root = try JSONSerialization.jsonObject(with: data) as? [String: Any] else {
            throw UsageError.badResponse
        }
        func window(_ object: Any?, claude: Bool) -> WindowUsage? {
            guard let value = object as? [String: Any],
                  let used = value[claude ? "utilization" : "used_percent"] as? Double,
                  used.isFinite else { return nil }
            var reset: Date?
            if claude, let stamp = value["resets_at"] as? String {
                let formatter = ISO8601DateFormatter()
                reset = formatter.date(from: stamp)
                if reset == nil {
                    formatter.formatOptions.insert(.withFractionalSeconds)
                    reset = formatter.date(from: stamp)
                }
            } else if let seconds = value["reset_at"] as? Double {
                reset = Date(timeIntervalSince1970: seconds)
            } else if let seconds = value["reset_after_seconds"] as? Double {
                reset = now.addingTimeInterval(seconds)
            }
            return WindowUsage(used: min(100, max(0, used)), reset: reset)
        }
        if provider == .claude {
            guard root["five_hour"] != nil || root["seven_day"] != nil else {
                throw UsageError.badResponse
            }
            return Usage(fiveHour: window(root["five_hour"], claude: true),
                         weekly: window(root["seven_day"], claude: true))
        }
        guard let limits = root["rate_limit"] as? [String: Any] else {
            throw UsageError.badResponse
        }
        var usage = Usage()
        for (name, secondary) in [("primary_window", false), ("secondary_window", true)] {
            guard let value = limits[name] as? [String: Any] else { continue }
            let duration = value["limit_window_seconds"] as? Int
            let reading = window(value, claude: false)
            if duration == 18000 || (duration == nil && !secondary) {
                usage.fiveHour = reading
            } else if duration == 604800 || (duration == nil && secondary) {
                usage.weekly = reading
            }
        }
        return usage
    }
}

enum I18n {
    static func isChinese(_ tag: String) -> Bool {
        let parts = tag.lowercased().replacingOccurrences(of: "_", with: "-").split(separator: "-")
        guard parts.first == "zh", !parts.contains(where: { ["hant", "tw", "hk", "mo"].contains(String($0)) }) else {
            return false
        }
        return parts.count == 1 || parts.contains(where: { ["hans", "cn", "sg"].contains(String($0)) })
    }
    static var chinese: Bool {
        let override = ProcessInfo.processInfo.environment["CCT_LANG"]?.trimmingCharacters(in: .whitespaces)
        return isChinese(override ?? Locale.preferredLanguages.first ?? "en")
    }
    static func t(_ english: String, _ chinese: String) -> String { self.chinese ? chinese : english }
}
