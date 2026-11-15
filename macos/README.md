# CCT for macOS

Native menu-bar companion for macOS 13 or later, with a universal binary for Apple Silicon and Intel.
Download the macOS DMG or ZIP from [this fork's Releases](https://github.com/wenkesong-li/claude-codex-taskbar/releases).
Move CCT.app to Applications and open it. The app has no Dock icon; click its menu-bar label.

The first macOS version shows Codex five-hour and weekly capacity, reset timestamps, manual refresh,
and a remaining/used toggle. It refreshes every two minutes. Simplified Chinese and English are
supported, following the system language; `CCT_LANG=zh-CN` or `en` overrides it when launching the binary.
Codex must already be signed into its CLI with a ChatGPT subscription, in `~/.codex/auth.json`
or `$CODEX_HOME/auth.json`. API-key logins do not provide subscription usage.

Claude is optional and disabled by default. Enabling it reads the existing Claude Code credentials
file or its `Claude Code-credentials` login Keychain entry. macOS may request Keychain access.
All credentials remain read-only; no login is refreshed or switched by CCT. Errors display an
em dash rather than a fake zero. Network requests go only to the enabled providers' usage endpoints;
redirects are refused. The release-page menu item opens this fork in your browser.

Unlike Windows, this initial macOS version does not include hardware counters, account switching,
quota notifications, keep-alive, or an in-app login-item toggle. To start it at login, add CCT in
macOS System Settings > General > Login Items. Settings are stored in macOS UserDefaults.

The macOS app is ad-hoc signed for bundle integrity, **not Developer ID signed or notarized**.
Gatekeeper may require explicit approval in System Settings > Privacy & Security. The build checks
both architectures, bundle signature, parsing/locale tests and process startup on a GitHub runner;
real-account usage and the menu layout still need testing on a physical Mac.

Build with Xcode command-line tools: `bash macos/build.sh`. Outputs are in `dist-macos/`.
