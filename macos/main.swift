import AppKit
import Security

// Login files and Keychain entries are read-only. No tokens or raw responses are logged.
func credentials(_ provider: Provider) throws -> (token: String, account: String?) {
    let home = FileManager.default.homeDirectoryForCurrentUser
    let environment = ProcessInfo.processInfo.environment
    let base = environment[provider == .codex ? "CODEX_HOME" : "CLAUDE_CONFIG_DIR"]
        .map { URL(fileURLWithPath: $0) } ?? home.appendingPathComponent(provider == .codex ? ".codex" : ".claude")
    var data = try? Data(contentsOf: base.appendingPathComponent(provider == .codex ? "auth.json" : ".credentials.json"))
    // Claude Code for macOS normally stores its OAuth login in the login Keychain.
    // This read happens only after the user explicitly enables Claude from the menu.
    if provider == .claude && data == nil && environment["CLAUDE_CONFIG_DIR"] == nil {
        let query: [String: Any] = [kSecClass as String: kSecClassGenericPassword,
                                    kSecAttrService as String: "Claude Code-credentials",
                                    kSecReturnData as String: true,
                                    kSecMatchLimit as String: kSecMatchLimitOne]
        var item: CFTypeRef?
        if SecItemCopyMatching(query as CFDictionary, &item) == errSecSuccess { data = item as? Data }
    }
    guard let data, let root = (try? JSONSerialization.jsonObject(with: data)) as? [String: Any],
          let tokens = root[provider == .codex ? "tokens" : "claudeAiOauth"] as? [String: Any],
          let token = tokens[provider == .codex ? "access_token" : "accessToken"] as? String,
          !token.isEmpty else { throw UsageError.noLogin }
    return (token, tokens["account_id"] as? String)
}

final class NoRedirects: NSObject, URLSessionTaskDelegate {
    func urlSession(_ session: URLSession, task: URLSessionTask,
                    willPerformHTTPRedirection response: HTTPURLResponse,
                    newRequest request: URLRequest, completionHandler: @escaping (URLRequest?) -> Void) {
        completionHandler(nil)
    }
}

@MainActor final class AppDelegate: NSObject, NSApplicationDelegate {
    private var status: NSStatusItem!
    private var timer: Timer?
    private var readings: [Provider: Usage] = [:]
    private var messages: [Provider: String] = [:]
    private var busy = false
    private let defaults = UserDefaults.standard
    private let network = URLSession(configuration: .ephemeral, delegate: NoRedirects(), delegateQueue: nil)
    private var remaining: Bool { !defaults.bool(forKey: "showUsed") }
    private var claudeEnabled: Bool { defaults.bool(forKey: "claudeEnabled") }

    func applicationDidFinishLaunching(_ notification: Notification) {
        status = NSStatusBar.system.statusItem(withLength: NSStatusItem.variableLength)
        render()
        refresh()
        timer = Timer.scheduledTimer(withTimeInterval: 120, repeats: true) { [weak self] _ in
            Task { @MainActor in self?.refresh() }
        }
    }

    private func percentage(_ window: WindowUsage?) -> String {
        guard let window else { return "—" }
        return String(format: "%.0f%%", remaining ? 100 - window.used : window.used)
    }
    private func add(_ menu: NSMenu, _ title: String, _ action: Selector? = nil, key: String = "") -> NSMenuItem {
        let item = NSMenuItem(title: title, action: action, keyEquivalent: key)
        item.target = self
        if action == nil { item.isEnabled = false }
        menu.addItem(item)
        return item
    }
    private func render() {
        let codex = readings[.codex]
        let mode = remaining ? I18n.t("left", "剩余") : I18n.t("used", "已用")
        status.button?.title = "Codex \(percentage(codex?.fiveHour))/\(percentage(codex?.weekly))"
        status.button?.toolTip = I18n.t("5-hour / weekly capacity ", "5 小时 / 每周额度 ") + mode
        let menu = NSMenu()
        add(menu, "CCT 0.4.7 · macOS")
        for provider in Provider.allCases where provider == .codex || claudeEnabled {
            menu.addItem(.separator())
            add(menu, provider.rawValue + " · " + (remaining ? I18n.t("Remaining", "剩余额度") : I18n.t("Used", "已用额度")))
            if let message = messages[provider] {
                add(menu, message)
            } else if let usage = readings[provider] {
                for (label, window) in [(I18n.t("5-hour", "5 小时"), usage.fiveHour), (I18n.t("Weekly", "每周"), usage.weekly)] {
                    add(menu, "\(label): \(percentage(window))")
                    if let reset = window?.reset {
                        let date = DateFormatter.localizedString(from: reset, dateStyle: .short, timeStyle: .short)
                        add(menu, "  " + I18n.t("Resets: ", "重置时间：") + date)
                    }
                }
            } else { add(menu, I18n.t("Loading…", "正在加载…")) }
        }
        menu.addItem(.separator())
        add(menu, I18n.t("Refresh now", "立即刷新"), #selector(refreshNow), key: "r").isEnabled = !busy
        add(menu, I18n.t("Show remaining", "显示剩余额度"), #selector(toggleRemaining)).state = remaining ? .on : .off
        add(menu, I18n.t("Enable Claude (may request Keychain access)", "启用 Claude（可能请求钥匙串访问）"), #selector(toggleClaude)).state = claudeEnabled ? .on : .off
        add(menu, I18n.t("Refreshes every 2 minutes", "每 2 分钟自动刷新"))
        add(menu, I18n.t("Open Releases", "打开版本发布页"), #selector(openReleases))
        add(menu, I18n.t("Quit", "退出"), #selector(quit), key: "q")
        status.menu = menu
    }
    @objc private func refreshNow() { refresh() }
    @objc private func toggleRemaining() { defaults.set(remaining, forKey: "showUsed"); render() }
    @objc private func toggleClaude() {
        defaults.set(!claudeEnabled, forKey: "claudeEnabled")
        if !claudeEnabled { readings[.claude] = nil; messages[.claude] = nil }
        render()
        refresh()
    }
    @objc private func openReleases() {
        NSWorkspace.shared.open(URL(string: "https://github.com/wenkesong-li/claude-codex-taskbar/releases")!)
    }
    @objc private func quit() { NSApplication.shared.terminate(nil) }

    private func errorText(_ error: Error) -> String {
        switch error {
        case UsageError.noLogin: return I18n.t("No CLI subscription login found", "未找到 CLI 订阅登录信息")
        case UsageError.expired: return I18n.t("Login expired; sign in again in the CLI", "登录已过期，请在 CLI 中重新登录")
        case UsageError.rateLimited: return I18n.t("Rate limited; try again later", "请求过于频繁，请稍后重试")
        case UsageError.badResponse: return I18n.t("Usage service returned an unexpected response", "用量服务返回了无法识别的数据")
        default: return I18n.t("Could not fetch usage; check the network", "无法获取用量，请检查网络")
        }
    }
    private func fetch(_ provider: Provider) async throws -> Usage {
        let auth = try credentials(provider)
        let url = provider == .codex ? "https://chatgpt.com/backend-api/wham/usage" : "https://api.anthropic.com/api/oauth/usage"
        var request = URLRequest(url: URL(string: url)!, timeoutInterval: 15)
        request.setValue("Bearer \(auth.token)", forHTTPHeaderField: "Authorization")
        request.setValue("application/json", forHTTPHeaderField: "Accept")
        if provider == .codex, let account = auth.account { request.setValue(account, forHTTPHeaderField: "ChatGPT-Account-Id") }
        if provider == .claude { request.setValue("oauth-2025-04-20", forHTTPHeaderField: "anthropic-beta") }
        let (data, response) = try await network.data(for: request)
        guard let http = response as? HTTPURLResponse else { throw UsageError.badResponse }
        switch http.statusCode {
        case 200: return try Usage.parse(data, provider: provider)
        case 401, 403: throw UsageError.expired
        case 429: throw UsageError.rateLimited
        default: throw UsageError.network
        }
    }
    private func refresh() {
        guard !busy else { return }
        busy = true
        render()
        Task { @MainActor in
            for provider in Provider.allCases where provider == .codex || claudeEnabled {
                do { readings[provider] = try await fetch(provider); messages[provider] = nil }
                catch { readings[provider] = nil; messages[provider] = errorText(error) }
            }
            if !claudeEnabled { readings[.claude] = nil; messages[.claude] = nil }
            busy = false
            render()
        }
    }
}

let app = NSApplication.shared
let delegate = AppDelegate()
app.setActivationPolicy(.accessory)
app.delegate = delegate
app.run()
