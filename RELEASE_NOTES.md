# CCT 0.4.7 — Chinese UI and macOS preview

## Downloads

- **Windows 10/11 x64:** portable `.exe` or the Windows ZIP. Extract and run the executable.
- **macOS 13+ Apple Silicon and Intel:** universal `.dmg` or `.zip`. Move CCT.app to Applications.
- SHA-256 checksum files are provided for both platforms.

## Windows

- Adds 98 Simplified Chinese translations for settings, menus, usage, reset times and errors.
- Detects Simplified Chinese Windows automatically; supports `CCT_LANG=zh-CN` overrides.
- Retains the existing Windows features and English, Portuguese and Spanish translations.
- Update checks now point to this fork's Releases.

## macOS preview

- New native menu-bar app with Chinese/English UI, Codex five-hour and weekly capacity,
  reset times, remaining/used toggle, manual refresh and automatic refresh every two minutes.
- Optional Claude support reads the existing credentials file or login Keychain entry after
  you enable it; macOS may ask for Keychain access.
- Requires an existing CLI subscription login. Credentials are read-only; errors show `—`.
- This preview does not include Windows hardware counters, account switching, notifications,
  or keep-alive. Add the app to Login Items manually if desired.
- Ad-hoc signed, **not Apple Developer ID signed or notarized**; macOS may require explicit approval
  in Privacy & Security. No Apple signing credentials were used.
- Automated checks cover parsers, language detection, both binary architectures, signature integrity
  and startup. Real-account usage and visual layout have not been verified on a physical Mac.

This fork is independent of OpenAI and Anthropic. Provider usage endpoints are undocumented and may change.
