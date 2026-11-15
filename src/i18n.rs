//! User-facing strings in English, Brazilian Portuguese, Spanish and Simplified Chinese.
//!
//! Each key holds one string per language, in `Lang::ALL` order. To add a language, add a
//! `Lang` variant (to the end of `Lang::ALL` too), map its Windows language ID in
//! `lang_from_langid`, and add its string to every key: the compiler lists the ones missing.

/// The discriminant is the index into each key's translations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lang {
    En = 0,
    PtBr = 1,
    Es = 2,
    ZhCn = 3,
}

impl Lang {
    pub const ALL: [Lang; 4] = [Lang::En, Lang::PtBr, Lang::Es, Lang::ZhCn];
}

/// Any Portuguese UI language maps to pt-BR and any Spanish one to Spanish (by the primary
/// language ID). Simplified Chinese locales map to zh-CN; everything else falls back to English.
pub fn lang_from_langid(langid: u16) -> Lang {
    if matches!(langid, 0x0804 | 0x1004 | 0x7804) {
        return Lang::ZhCn;
    }
    const LANG_PORTUGUESE: u16 = 0x16;
    const LANG_SPANISH: u16 = 0x0a;
    match langid & 0x3ff {
        LANG_PORTUGUESE => Lang::PtBr,
        LANG_SPANISH => Lang::Es,
        _ => Lang::En,
    }
}

/// The language named by the `CCT_LANG` environment variable (`en`, `pt`/`pt-BR`, `es`,
/// `zh`/`zh-CN`/`zh-SG`/`zh-Hans`), used to check translations and take screenshots without changing the
/// Windows display language. `None` for anything else.
pub fn lang_from_override(value: &str) -> Option<Lang> {
    let value = value.trim().to_ascii_lowercase();
    match value.split(['-', '_']).next()? {
        "en" => Some(Lang::En),
        "pt" => Some(Lang::PtBr),
        "es" => Some(Lang::Es),
        "zh" => {
            let parts: Vec<_> = value.split(['-', '_']).collect();
            if parts
                .iter()
                .any(|part| matches!(*part, "hant" | "tw" | "hk" | "mo"))
            {
                None
            } else if parts.len() == 1
                || parts
                    .iter()
                    .any(|part| matches!(*part, "hans" | "cn" | "sg"))
            {
                Some(Lang::ZhCn)
            } else {
                None
            }
        }
        _ => None,
    }
}

/// `CCT_LANG` when it names a supported language, otherwise the Windows display language.
pub fn detect_lang() -> Lang {
    if let Some(lang) = std::env::var("CCT_LANG")
        .ok()
        .and_then(|v| lang_from_override(&v))
    {
        return lang;
    }
    // SAFETY: no arguments, no preconditions.
    let langid = unsafe { windows::Win32::Globalization::GetUserDefaultUILanguage() };
    lang_from_langid(langid)
}

/// Replaces the first `{}` in a template string.
pub fn fmt(template: &str, value: &str) -> String {
    template.replacen("{}", value, 1)
}

/// Fills both `{}` of a template in order; a `{}` inside `first` stays literal.
pub fn fmt2(template: &str, first: &str, second: &str) -> String {
    match template.split_once("{}") {
        Some((head, rest)) => format!("{head}{first}{}", fmt(rest, second)),
        None => template.into(),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    MenuSettings,
    MenuRefresh,
    MenuCopyUsage,
    MenuAutostart,
    MenuExit,
    LabelWeek,
    LabelOpusWeek,
    LabelSonnetWeek,
    ExtraUsageEnabled,
    ClaudeExpired,
    ClaudeNoLogin,
    CodexExpired,
    CodexNoLogin,
    NoSubscription,
    StaleFmt,
    StaleReasonFmt,
    SessionDataFmt,
    ResetsFmt,
    ResetsInMinutesFmt,
    ResetsInHoursOnlyFmt,
    ResetsInHoursFmt,
    ResetsInDaysOnlyFmt,
    ResetsInDaysFmt,
    UpdatedFmt,
    NoRecentData,
    RateLimited,
    RateLimitedNextFmt,
    Loading,
    ErrorFmt,
    VersionFmt,
    UpdateCheck,
    UpdateChecking,
    UpdateUpToDate,
    UpdateAvailableFmt,
    UpdateOpen,
    UpdateFailedFmt,
    HintWidgetsButton,
    HiddenItems,
    LoginFoundStatus,
    SettingsPageTitle,
    SettingsGeneral,
    SettingsPlacement,
    PlacementLeft,
    PlacementRight,
    SettingsHardware,
    SettingsLimits,
    SwitchOn,
    SwitchOff,
    KeepAliveRow,
    KeepAliveSubClaude,
    KeepAliveSubCodex,
    SettingsLabel,
    SettingsLabelSub,
    SettingsLabelSubAccount,
    SettingsTitle,
    SettingsProviders,
    SettingsItems,
    SettingsRefresh,
    SettingsRefreshSub,
    SettingsWeekSuffix,
    NotifyNearLimit,
    ShowRemaining,
    Remaining,
    NotifyNearLimitSub,
    NotifyTitle,
    NotifyPlacementTitle,
    NotifyPlacementBody,
    NotifyMovedRight,
    NotifyMovedLeft,
    NotifyFiveHour,
    KeepAlivePromptBody,
    KeepAliveRenewedFmt,
    KeepAliveTriedFmt,
    MenuClaudeAccount,
    MenuSaveAccount,
    MenuRemoveAccount,
    AccountNotSaved,
    AccountAddHint,
    AccountExpiresFmt,
    AccountNeedsLogin,
    AgoMinutesFmt,
    AgoHoursFmt,
    AgoDaysFmt,
    SwitchBusyPrompt,
    SwitchUnsavedPrompt,
    RemoveAccountPromptFmt,
    SwitchFailedFmt,
    SaveFailedFmt,
    ErrNotPaid,
    ErrTrial,
    ErrNotSubscription,
    ErrNoIdentity,
    ErrConfigDir,
    ErrExpired,
    ErrUnreadableConfig,
    ErrLocked,
    ErrNothingChangedFmt,
    ErrNoSuchSlot,
}

impl Key {
    pub const ALL: &'static [Key] = &[
        Key::MenuSettings,
        Key::MenuRefresh,
        Key::MenuCopyUsage,
        Key::MenuAutostart,
        Key::MenuExit,
        Key::LabelWeek,
        Key::LabelOpusWeek,
        Key::LabelSonnetWeek,
        Key::ExtraUsageEnabled,
        Key::ClaudeExpired,
        Key::ClaudeNoLogin,
        Key::CodexExpired,
        Key::CodexNoLogin,
        Key::NoSubscription,
        Key::StaleFmt,
        Key::StaleReasonFmt,
        Key::SessionDataFmt,
        Key::ResetsFmt,
        Key::ResetsInMinutesFmt,
        Key::ResetsInHoursOnlyFmt,
        Key::ResetsInHoursFmt,
        Key::ResetsInDaysOnlyFmt,
        Key::ResetsInDaysFmt,
        Key::UpdatedFmt,
        Key::NoRecentData,
        Key::RateLimited,
        Key::RateLimitedNextFmt,
        Key::Loading,
        Key::ErrorFmt,
        Key::VersionFmt,
        Key::UpdateCheck,
        Key::UpdateChecking,
        Key::UpdateUpToDate,
        Key::UpdateAvailableFmt,
        Key::UpdateOpen,
        Key::UpdateFailedFmt,
        Key::HintWidgetsButton,
        Key::HiddenItems,
        Key::LoginFoundStatus,
        Key::SettingsPageTitle,
        Key::SettingsGeneral,
        Key::SettingsPlacement,
        Key::PlacementLeft,
        Key::PlacementRight,
        Key::SettingsHardware,
        Key::SettingsLimits,
        Key::SwitchOn,
        Key::SwitchOff,
        Key::KeepAliveRow,
        Key::KeepAliveSubClaude,
        Key::KeepAliveSubCodex,
        Key::SettingsLabel,
        Key::SettingsLabelSub,
        Key::SettingsLabelSubAccount,
        Key::SettingsTitle,
        Key::SettingsProviders,
        Key::SettingsItems,
        Key::SettingsRefresh,
        Key::SettingsRefreshSub,
        Key::SettingsWeekSuffix,
        Key::NotifyNearLimit,
        Key::ShowRemaining,
        Key::Remaining,
        Key::NotifyNearLimitSub,
        Key::NotifyTitle,
        Key::NotifyPlacementTitle,
        Key::NotifyPlacementBody,
        Key::NotifyMovedRight,
        Key::NotifyMovedLeft,
        Key::NotifyFiveHour,
        Key::KeepAlivePromptBody,
        Key::KeepAliveRenewedFmt,
        Key::KeepAliveTriedFmt,
        Key::MenuClaudeAccount,
        Key::MenuSaveAccount,
        Key::MenuRemoveAccount,
        Key::AccountNotSaved,
        Key::AccountAddHint,
        Key::AccountExpiresFmt,
        Key::AccountNeedsLogin,
        Key::AgoMinutesFmt,
        Key::AgoHoursFmt,
        Key::AgoDaysFmt,
        Key::SwitchBusyPrompt,
        Key::SwitchUnsavedPrompt,
        Key::RemoveAccountPromptFmt,
        Key::SwitchFailedFmt,
        Key::SaveFailedFmt,
        Key::ErrNotPaid,
        Key::ErrTrial,
        Key::ErrNotSubscription,
        Key::ErrNoIdentity,
        Key::ErrConfigDir,
        Key::ErrExpired,
        Key::ErrUnreadableConfig,
        Key::ErrLocked,
        Key::ErrNothingChangedFmt,
        Key::ErrNoSuchSlot,
    ];
}

pub fn t(lang: Lang, key: Key) -> &'static str {
    strings(key)[lang as usize]
}

/// Every translation of `key`, in `Lang::ALL` order. The array length comes from
/// `Lang::ALL`, so adding a language without translating every key does not compile.
fn strings(key: Key) -> [&'static str; Lang::ALL.len()] {
    match key {
        Key::MenuSettings => ["Settings", "Configurações", "Configuración", "设置"],
        Key::MenuRefresh => ["Refresh now", "Atualizar agora", "Actualizar ahora", "立即刷新"],
        Key::MenuCopyUsage => ["Copy usage", "Copiar uso", "Copiar uso", "复制用量"],
        Key::MenuAutostart => [
            "Start with Windows",
            "Iniciar com o Windows",
            "Iniciar con Windows",
            "开机启动",
        ],
        Key::MenuExit => ["Exit", "Sair", "Salir", "退出"],
        Key::LabelWeek => ["wk", "sem", "sem", "周"],
        Key::LabelOpusWeek => ["Opus wk", "Opus sem", "Opus sem", "Opus 周"],
        Key::LabelSonnetWeek => ["Sonnet wk", "Sonnet sem", "Sonnet sem", "Sonnet 周"],
        Key::ExtraUsageEnabled => [
            "Extra usage enabled",
            "Uso extra habilitado",
            "Uso adicional habilitado",
            "已启用额外用量",
        ],
        Key::ClaudeExpired => [
            "Claude login expired — open Claude Code to renew",
            "Login do Claude expirado — abra o Claude Code para renovar",
            "Sesión de Claude caducada — abre Claude Code para renovarla",
            "Claude 登录已过期，请打开 Claude Code 重新登录",
        ],
        Key::ClaudeNoLogin => [
            "Claude Code CLI login not found — desktop app logins aren't supported; run `claude` once to fix it",
            "Login do Claude Code CLI não encontrado — logins do app desktop não são suportados; rode `claude` uma vez para resolver",
            "No se encontró el inicio de sesión de la CLI de Claude Code — los inicios de sesión de la app de escritorio no son compatibles; ejecuta `claude` una vez para solucionarlo",
            "未找到 Claude Code CLI 登录信息；不支持桌面端登录，请先运行一次 `claude`",
        ],
        Key::CodexExpired => [
            "Codex login expired — run `codex login`",
            "Login do Codex expirado — rode `codex login`",
            "Sesión de Codex caducada — ejecuta `codex login`",
            "Codex 登录已过期，请运行 `codex login`",
        ],
        Key::CodexNoLogin => [
            "Log in by running `codex login`",
            "Faça login rodando `codex login`",
            "Inicia sesión ejecutando `codex login`",
            "请运行 `codex login` 登录",
        ],
        Key::NoSubscription => [
            "No subscription limits (API key login)",
            "Sem limites de assinatura (login por API key)",
            "Sin límites de suscripción (inicio de sesión con clave de API)",
            "API 密钥登录不提供订阅额度",
        ],
        Key::StaleFmt => [
            "Update failed; last data at {}",
            "Falha ao atualizar; último dado às {}",
            "Error al actualizar; último dato a las {}",
            "更新失败；上次数据时间：{}",
        ],
        Key::StaleReasonFmt => [
            "Update failed ({}); last data at {}",
            "Falha ao atualizar ({}); último dado às {}",
            "Error al actualizar ({}); último dato a las {}",
            "更新失败（{}）；上次数据时间：{}",
        ],
        Key::SessionDataFmt => [
            "Data from {} (last session)",
            "Dado de {} (última sessão)",
            "Dato de {} (última sesión)",
            "数据时间：{}（上次会话）",
        ],
        Key::ResetsFmt => [
            "resets at {}",
            "reseta às {}",
            "se reinicia a las {}",
            "重置时间：{}",
        ],
        Key::ResetsInMinutesFmt => ["in {}m", "em {}min", "en {}min", "{} 分钟后重置"],
        Key::ResetsInHoursOnlyFmt => ["in {}h", "em {}h", "en {}h", "{} 小时后重置"],
        Key::ResetsInHoursFmt => [
            "in {}h {}m",
            "em {}h {}min",
            "en {}h {}min",
            "{} 小时 {} 分钟后重置",
        ],
        Key::ResetsInDaysOnlyFmt => ["in {}d", "em {}d", "en {}d", "{} 天后重置"],
        Key::ResetsInDaysFmt => [
            "in {}d {}h",
            "em {}d {}h",
            "en {}d {}h",
            "{} 天 {} 小时后重置",
        ],
        Key::UpdatedFmt => [
            "updated at {}",
            "atualizado às {}",
            "actualizado a las {}",
            "更新时间：{}",
        ],
        Key::NoRecentData => [
            "No recent data",
            "Sem dado recente",
            "Sin datos recientes",
            "暂无近期数据",
        ],
        Key::RateLimited => [
            "rate limited",
            "limite de requisições",
            "límite de solicitudes",
            "请求过于频繁",
        ],
        Key::RateLimitedNextFmt => [
            "rate limited, next try {}",
            "limite de requisições, próxima tentativa às {}",
            "límite de solicitudes, próximo intento a las {}",
            "请求过于频繁，下次尝试：{}",
        ],
        Key::Loading => ["Loading…", "Carregando…", "Cargando…", "正在加载…"],
        Key::ErrorFmt => ["Error: {}", "Erro: {}", "Error: {}", "错误：{}"],
        Key::VersionFmt => ["Version {}", "Versão {}", "Versión {}", "版本 {}"],
        Key::UpdateCheck => [
            "Check for updates",
            "Verificar atualizações",
            "Buscar actualizaciones",
            "检查更新",
        ],
        Key::UpdateChecking => [
            "Checking…",
            "Verificando…",
            "Comprobando…",
            "正在检查…",
        ],
        Key::UpdateUpToDate => [
            "You have the latest version",
            "Você está na versão mais recente",
            "Tienes la versión más reciente",
            "已是最新版本",
        ],
        Key::UpdateAvailableFmt => [
            "Version {} is available",
            "A versão {} está disponível",
            "La versión {} está disponible",
            "发现新版本 {}",
        ],
        Key::UpdateOpen => [
            "Open the release page",
            "Abrir a página da versão",
            "Abrir la página de la versión",
            "打开版本发布页",
        ],
        Key::UpdateFailedFmt => [
            "Could not check for updates ({})",
            "Não foi possível verificar atualizações ({})",
            "No se pudo comprobar si hay actualizaciones ({})",
            "无法检查更新（{}）",
        ],
        Key::HintWidgetsButton => [
            "Turn off the Windows Widgets button to free this corner (Settings → Personalization → Taskbar → Widgets)",
            "Desative o botão de Widgets do Windows para liberar este canto (Configurações → Personalização → Barra de tarefas → Widgets)",
            "Desactiva el botón de Widgets de Windows para liberar esta esquina (Configuración → Personalización → Barra de tareas → Widgets)",
            "关闭 Windows 小组件按钮以腾出此处空间（设置 → 个性化 → 任务栏 → 小组件）",
        ],
        Key::HiddenItems => [
            "Not shown (no room):",
            "Não exibidos (sem espaço):",
            "No mostrados (sin espacio):",
            "因空间不足而隐藏：",
        ],
        Key::LoginFoundStatus => [
            "Login found",
            "Login encontrado",
            "Sesión encontrada",
            "已找到登录信息",
        ],
        Key::SettingsPageTitle => ["Settings", "Configurações", "Configuración", "设置"],
        Key::SettingsGeneral => ["General", "Geral", "General", "常规"],
        Key::SettingsPlacement => [
            "Strip position",
            "Posição da faixa",
            "Posición de la franja",
            "显示位置",
        ],
        Key::PlacementLeft => ["Left", "Esquerda", "Izquierda", "左侧"],
        Key::PlacementRight => ["Right", "Direita", "Derecha", "右侧"],
        Key::SettingsHardware => ["Hardware", "Hardware", "Hardware", "硬件"],
        Key::SettingsLimits => ["Limits", "Limites", "Límites", "额度"],
        Key::SwitchOn => ["On", "Ativado", "Activado", "开"],
        Key::SwitchOff => ["Off", "Desativado", "Desactivado", "关"],
        Key::KeepAliveRow => [
            "Keep login active",
            "Manter login ativo",
            "Mantener la sesión activa",
            "保持登录有效",
        ],
        Key::KeepAliveSubClaude => [
            "Experimental · runs `claude auth status` when the login expires",
            "Experimental · roda `claude auth status` quando o login expira",
            "Experimental · ejecuta `claude auth status` cuando la sesión caduca",
            "实验功能 · 登录过期时运行 `claude auth status`",
        ],
        Key::KeepAliveSubCodex => [
            "Experimental · runs `codex login status` when the login expires",
            "Experimental · roda `codex login status` quando o login expira",
            "Experimental · ejecuta `codex login status` cuando la sesión caduca",
            "实验功能 · 登录过期时运行 `codex login status`",
        ],
        Key::SettingsLabel => ["Label", "Rótulo", "Etiqueta", "标签"],
        Key::SettingsLabelSub => [
            "Shown before the gauges · up to 5 characters",
            "Aparece antes dos mostradores · até 5 caracteres",
            "Se muestra antes de los indicadores · hasta 5 caracteres",
            "显示在用量条前，最多 5 个字符",
        ],
        Key::SettingsLabelSubAccount => [
            "This account's label · up to 5 characters",
            "Rótulo desta conta · até 5 caracteres",
            "La etiqueta de esta cuenta · hasta 5 caracteres",
            "此账号的标签，最多 5 个字符",
        ],
        Key::SettingsTitle => [
            "CCT — Settings",
            "CCT — Configurações",
            "CCT — Configuración",
            "CCT — 设置",
        ],
        Key::SettingsProviders => ["Subscriptions", "Assinaturas", "Suscripciones", "订阅"],
        Key::SettingsItems => [
            "Items on the strip",
            "Itens na faixa",
            "Elementos en la franja",
            "任务栏显示项",
        ],
        Key::SettingsRefresh => [
            "Refresh (seconds)",
            "Atualização (segundos)",
            "Actualización (segundos)",
            "刷新间隔（秒）",
        ],
        Key::SettingsRefreshSub => [
            "Hardware 1–5 · Limits 60–3600",
            "Hardware 1–5 · Limites 60–3600",
            "Hardware 1–5 · Límites 60–3600",
            "硬件 1–5 · 额度 60–3600",
        ],
        Key::SettingsWeekSuffix => ["weekly", "semanal", "semanal", "每周"],
        Key::ShowRemaining => [
            "Show remaining instead of used",
            "Mostrar restante em vez de usado",
            "Mostrar lo que queda en lugar de lo usado",
            "显示剩余额度",
        ],
        Key::Remaining => ["left", "restante", "restante", "剩余"],
        Key::NotifyNearLimit => [
            "Notify near limit",
            "Avisar ao se aproximar do limite",
            "Avisar cerca del límite",
            "额度不足时提醒",
        ],
        Key::NotifyNearLimitSub => [
            "Windows notification at {} of a limit",
            "Notificação do Windows ao chegar a {} de um limite",
            "Notificación de Windows al llegar al {} de un límite",
            "用量达到 {} 时发送 Windows 通知",
        ],
        Key::NotifyTitle => [
            "CCT — Usage limits",
            "CCT — Limites de uso",
            "CCT — Límites de uso",
            "CCT — 用量额度",
        ],
        Key::NotifyPlacementTitle => [
            "CCT — Taskbar",
            "CCT — Barra de tarefas",
            "CCT — Barra de tareas",
            "CCT — 任务栏",
        ],
        Key::NotifyPlacementBody => [
            "CCT could not fit on the taskbar: there is no room in the left corner or next to the system tray. Close some windows or open Settings to choose fewer items.",
            "O CCT não coube na barra de tarefas: não há espaço no canto esquerdo nem ao lado da bandeja do sistema. Feche algumas janelas ou abra as Configurações para escolher menos itens.",
            "CCT no cabe en la barra de tareas: no hay espacio en la esquina izquierda ni junto a la bandeja del sistema. Cierra algunas ventanas o abre Configuración para elegir menos elementos.",
            "任务栏左侧和系统托盘旁均无足够空间显示 CCT。请关闭部分窗口，或在设置中减少显示项。",
        ],
        Key::NotifyMovedRight => [
            "CCT moved next to the system tray: there is no room in the left corner of the taskbar.",
            "O CCT foi para o lado da bandeja do sistema: não há espaço no canto esquerdo da barra de tarefas.",
            "CCT se movió junto a la bandeja del sistema: no hay espacio en la esquina izquierda de la barra de tareas.",
            "任务栏左侧空间不足，CCT 已移至系统托盘旁。",
        ],
        Key::NotifyMovedLeft => [
            "CCT moved to the left corner of the taskbar: there is no room next to the system tray.",
            "O CCT foi para o canto esquerdo da barra de tarefas: não há espaço ao lado da bandeja do sistema.",
            "CCT se movió a la esquina izquierda de la barra de tareas: no hay espacio junto a la bandeja del sistema.",
            "系统托盘旁空间不足，CCT 已移至任务栏左侧。",
        ],
        Key::NotifyFiveHour => ["5h", "5h", "5h", "5 小时"],
        Key::KeepAlivePromptBody => [
            "Logins for {} can expire when the CLI is not used for a while (Claude Code: about 8 hours), and the strip then shows —.\n\nKeep them active? When a login expires, the app runs the CLI's own status command in the background (`claude auth status` / `codex login status`: no prompt is sent, no quota is used) so the CLI renews its own login.\n\nExperimental. You can change this later in Settings.",
            "O login de {} pode expirar quando a CLI fica um tempo sem uso (Claude Code: cerca de 8 horas), e a faixa passa a mostrar —.\n\nManter ativo? Quando expirar, o app roda o comando de status da própria CLI em segundo plano (`claude auth status` / `codex login status`: nenhuma mensagem é enviada, não gasta cota) para a própria CLI renovar o login.\n\nExperimental. Você pode mudar isso depois nas Configurações.",
            "La sesión de {} puede caducar cuando la CLI pasa un tiempo sin usarse (Claude Code: unas 8 horas), y entonces la franja muestra —.\n\n¿Mantenerla activa? Cuando caduque, la app ejecuta en segundo plano el propio comando de estado de la CLI (`claude auth status` / `codex login status`: no se envía ningún mensaje ni se gasta cuota) para que la CLI renueve su propia sesión.\n\nExperimental. Puedes cambiarlo después en Configuración.",
            "如果长时间不使用 CLI，{} 的登录可能过期（Claude Code 约为 8 小时），用量条会显示 —。\n\n是否保持登录有效？登录过期时，程序将在后台运行 CLI 自带的状态命令（`claude auth status` / `codex login status`，不会发送提示词或消耗额度），由 CLI 自行续期登录。\n\n此功能为实验功能，可稍后在设置中更改。",
        ],
        Key::KeepAliveRenewedFmt => [
            "Login renewed by the CLI at {}",
            "Login renovado pela CLI às {}",
            "Sesión renovada por la CLI a las {}",
            "CLI 已于 {} 续期登录",
        ],
        Key::KeepAliveTriedFmt => [
            "CLI status check ran at {}; login still expired",
            "Verificação da CLI rodou às {}; login continua expirado",
            "La comprobación de la CLI se ejecutó a las {}; la sesión sigue caducada",
            "已于 {} 运行 CLI 状态检查；登录仍已过期",
        ],
        Key::MenuClaudeAccount => [
            "Claude account",
            "Conta do Claude",
            "Cuenta de Claude",
            "Claude 账号",
        ],
        Key::MenuSaveAccount => [
            "Save current account",
            "Salvar conta atual",
            "Guardar la cuenta actual",
            "保存当前账号",
        ],
        Key::MenuRemoveAccount => [
            "Remove saved account",
            "Remover conta salva",
            "Quitar una cuenta guardada",
            "移除已保存账号",
        ],
        Key::AccountNotSaved => [
            "Current login not saved",
            "Login atual não salvo",
            "La sesión actual no está guardada",
            "当前登录尚未保存",
        ],
        Key::AccountAddHint => [
            "To add another account: /login in Claude Code, then Save current account",
            "Para adicionar outra conta: /login no Claude Code e depois Salvar conta atual",
            "Para añadir otra cuenta: /login en Claude Code y después Guardar la cuenta actual",
            "添加其他账号：在 Claude Code 中运行 /login，然后保存当前账号",
        ],
        Key::AccountExpiresFmt => [
            "⚠ login expires in {}d",
            "⚠ login expira em {}d",
            "⚠ la sesión caduca en {}d",
            "⚠ 登录将在 {} 天后过期",
        ],
        Key::AccountNeedsLogin => [
            "⚠ needs /login",
            "⚠ precisa de /login",
            "⚠ necesita /login",
            "⚠ 需要运行 /login",
        ],
        Key::AgoMinutesFmt => ["{}m ago", "há {}min", "hace {}min", "{} 分钟前"],
        Key::AgoHoursFmt => ["{}h ago", "há {}h", "hace {}h", "{} 小时前"],
        Key::AgoDaysFmt => ["{}d ago", "há {}d", "hace {}d", "{} 天前"],
        Key::SwitchBusyPrompt => [
            "A Claude Code task is running or waiting for your answer. It will continue on the new account from its next request, and its first message there may use extra usage while the cache rebuilds.\n\nSwitch anyway?",
            "Uma tarefa do Claude Code está em andamento ou esperando sua resposta. Ela continua na conta nova a partir da próxima requisição, e a primeira mensagem lá pode gastar mais uso enquanto o cache é reconstruído.\n\nTrocar mesmo assim?",
            "Hay una tarea de Claude Code en curso o esperando tu respuesta. Seguirá en la cuenta nueva desde su próxima solicitud, y su primer mensaje allí puede gastar más uso mientras se reconstruye la caché.\n\n¿Cambiar de todos modos?",
            "Claude Code 任务正在运行或等待你的回复。从下一次请求开始，任务将使用新账号继续，首次请求可能因重建缓存而消耗更多额度。\n\n仍要切换吗？",
        ],
        Key::SwitchUnsavedPrompt => [
            "The current Claude login is not saved. A safety copy is kept, but it will not appear in the list.\n\nSwitch anyway? (Choose No, then Save current account, to keep it in the list.)",
            "O login atual do Claude não está salvo. Uma cópia de segurança é guardada, mas ela não aparece na lista.\n\nTrocar mesmo assim? (Escolha Não e depois Salvar conta atual para mantê-lo na lista.)",
            "La sesión actual de Claude no está guardada. Se conserva una copia de seguridad, pero no aparecerá en la lista.\n\n¿Cambiar de todos modos? (Elige No y después Guardar la cuenta actual para mantenerla en la lista.)",
            "当前 Claude 登录尚未保存。程序会保留安全副本，但该账号不会显示在列表中。\n\n仍要切换吗？（选择“否”，然后保存当前账号，可将其保留在列表中。）",
        ],
        Key::RemoveAccountPromptFmt => [
            "Remove {} from the saved accounts? Its saved login is deleted from this PC. The live Claude Code login is not changed.",
            "Remover {} das contas salvas? O login salvo é apagado deste PC. O login atual do Claude Code não muda.",
            "¿Quitar {} de las cuentas guardadas? Su sesión guardada se borra de este PC. La sesión actual de Claude Code no cambia.",
            "要从已保存账号中移除 {} 吗？此电脑上保存的登录信息将被删除，Claude Code 当前登录不受影响。",
        ],
        Key::SwitchFailedFmt => [
            "Could not switch the Claude account: {}",
            "Não foi possível trocar a conta do Claude: {}",
            "No se pudo cambiar la cuenta de Claude: {}",
            "无法切换 Claude 账号：{}",
        ],
        Key::SaveFailedFmt => [
            "Could not save the Claude account: {}",
            "Não foi possível salvar a conta do Claude: {}",
            "No se pudo guardar la cuenta de Claude: {}",
            "无法保存 Claude 账号：{}",
        ],
        Key::ErrNotPaid => [
            "only paid Claude plans (Pro, Max, Team, Enterprise) can be saved.",
            "só planos pagos do Claude (Pro, Max, Team, Enterprise) podem ser salvos.",
            "solo se pueden guardar los planes de pago de Claude (Pro, Max, Team, Enterprise).",
            "仅支持保存付费 Claude 订阅账号（Pro、Max、Team、Enterprise）。",
        ],
        Key::ErrTrial => [
            "accounts on a Claude Code trial can't be saved.",
            "contas em período de teste do Claude Code não podem ser salvas.",
            "las cuentas en periodo de prueba de Claude Code no se pueden guardar.",
            "无法保存处于 Claude Code 试用期的账号。",
        ],
        Key::ErrNotSubscription => [
            "this login is not a Claude subscription (an API key?).",
            "este login não é uma assinatura do Claude (uma chave de API?).",
            "esta sesión no es una suscripción de Claude (¿una clave de API?).",
            "此登录不是 Claude 订阅账号（可能使用了 API 密钥）。",
        ],
        Key::ErrNoIdentity => [
            "Claude Code has no account details yet; open it once and try again.",
            "o Claude Code ainda não tem os dados da conta; abra-o uma vez e tente de novo.",
            "Claude Code todavía no tiene los datos de la cuenta; ábrelo una vez y vuelve a intentarlo.",
            "Claude Code 暂无账号信息，请先打开一次再重试。",
        ],
        Key::ErrConfigDir => [
            "CLAUDE_CONFIG_DIR is set, so the login lives elsewhere; this is not supported.",
            "CLAUDE_CONFIG_DIR está definido, então o login fica em outro lugar; isso não é suportado.",
            "CLAUDE_CONFIG_DIR está definido, así que la sesión está en otro lugar; esto no es compatible.",
            "已设置 CLAUDE_CONFIG_DIR，登录信息位于其他目录，暂不支持。",
        ],
        Key::ErrExpired => [
            "that account's saved login has expired. Run /login as that account in Claude Code, then Save current account.",
            "o login salvo dessa conta expirou. Faça /login com essa conta no Claude Code e depois Salvar conta atual.",
            "la sesión guardada de esa cuenta caducó. Haz /login con esa cuenta en Claude Code y después Guardar la cuenta actual.",
            "此账号保存的登录已过期。请在 Claude Code 中以该账号运行 /login，然后保存当前账号。",
        ],
        Key::ErrUnreadableConfig => [
            "~/.claude.json could not be read; open Claude Code once and try again. Nothing was changed.",
            "não foi possível ler o ~/.claude.json; abra o Claude Code uma vez e tente de novo. Nada foi alterado.",
            "no se pudo leer ~/.claude.json; abre Claude Code una vez y vuelve a intentarlo. No se cambió nada.",
            "无法读取 ~/.claude.json，请先打开一次 Claude Code 再重试。未作任何更改。",
        ],
        Key::ErrLocked => [
            "Claude Code is busy renewing its login; try again in a few seconds. Nothing was changed.",
            "o Claude Code está renovando o login; tente de novo em alguns segundos. Nada foi alterado.",
            "Claude Code está renovando su sesión; vuelve a intentarlo en unos segundos. No se cambió nada.",
            "Claude Code 正在续期登录，请稍后重试。未作任何更改。",
        ],
        Key::ErrNothingChangedFmt => [
            "{}. Nothing was changed.",
            "{}. Nada foi alterado.",
            "{}. No se cambió nada.",
            "{}。未作任何更改。",
        ],
        Key::ErrNoSuchSlot => [
            "that account is no longer saved.",
            "essa conta não está mais salva.",
            "esa cuenta ya no está guardada.",
            "此账号已不在保存列表中。",
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simplified_chinese_language_detection() {
        for id in [0x0804, 0x1004, 0x7804] {
            assert_eq!(lang_from_langid(id), Lang::ZhCn, "{id:#06x}");
        }
        for tag in ["zh", "zh-CN", "zh-SG", "zh-Hans", " ZH_hans_CN "] {
            assert_eq!(lang_from_override(tag), Some(Lang::ZhCn), "{tag}");
        }
        for id in [0x0404, 0x0c04, 0x1404, 0x7c04] {
            assert_eq!(lang_from_langid(id), Lang::En, "{id:#06x}");
        }
        for tag in ["zh-TW", "zh-HK", "zh-MO", "zh-Hant", "zh-Hant-CN"] {
            assert_eq!(lang_from_override(tag), None, "{tag}");
        }
    }

    #[test]
    fn chinese_menu_and_reset_templates() {
        assert_eq!(t(Lang::ZhCn, Key::MenuSettings), "设置");
        assert_eq!(
            fmt2(t(Lang::ZhCn, Key::ResetsInHoursFmt), "2", "30"),
            "2 小时 30 分钟后重置"
        );
    }

    #[test]
    fn fmt2_fills_both_placeholders_in_order() {
        assert_eq!(fmt2("a ({}); b {}", "x", "y"), "a (x); b y");
    }

    #[test]
    fn fmt2_keeps_braces_inside_the_first_value() {
        assert_eq!(fmt2("({}) at {}", "odd {}", "09:41"), "(odd {}) at 09:41");
    }

    #[test]
    fn every_key_has_every_language() {
        for key in Key::ALL {
            for lang in Lang::ALL {
                assert!(!t(lang, *key).is_empty(), "{key:?} {lang:?}");
            }
        }
    }

    #[test]
    fn every_translation_keeps_the_english_placeholders() {
        for key in Key::ALL {
            let want = t(Lang::En, *key).matches("{}").count();
            for lang in Lang::ALL {
                assert_eq!(
                    t(lang, *key).matches("{}").count(),
                    want,
                    "{key:?} {lang:?}"
                );
            }
        }
    }

    #[test]
    fn lang_all_is_in_index_order() {
        for (i, lang) in Lang::ALL.into_iter().enumerate() {
            assert_eq!(lang as usize, i);
        }
    }

    #[test]
    fn key_all_lists_every_key_once() {
        let mut seen = std::collections::HashSet::new();
        for key in Key::ALL {
            assert!(seen.insert(format!("{key:?}")), "{key:?} twice");
        }
    }

    #[test]
    fn langid_maps_spanish_variants_to_spanish() {
        for id in [0x0c0a, 0x040a, 0x080a, 0x2c0a, 0x540a] {
            assert_eq!(lang_from_langid(id), Lang::Es, "{id:#06x}"); // es-ES, es-ES_tradnl, es-MX, es-AR, es-US
        }
        assert_eq!(lang_from_langid(0x0416), Lang::PtBr);
        assert_eq!(lang_from_langid(0x040c), Lang::En); // fr-FR falls back to English
    }

    #[test]
    fn cct_lang_override_accepts_language_tags() {
        assert_eq!(lang_from_override("en"), Some(Lang::En));
        assert_eq!(lang_from_override("EN-us"), Some(Lang::En));
        assert_eq!(lang_from_override("pt-BR"), Some(Lang::PtBr));
        assert_eq!(lang_from_override("pt"), Some(Lang::PtBr));
        assert_eq!(lang_from_override(" es-MX "), Some(Lang::Es));
        // Unknown or empty values fall back to the Windows language.
        assert_eq!(lang_from_override("de"), None);
        assert_eq!(lang_from_override(""), None);
    }

    #[test]
    fn langid_maps_portuguese_to_pt_br() {
        assert_eq!(lang_from_langid(0x0416), Lang::PtBr); // pt-BR
        assert_eq!(lang_from_langid(0x0816), Lang::PtBr); // pt-PT
        assert_eq!(lang_from_langid(0x0409), Lang::En);
        assert_eq!(lang_from_langid(0x0407), Lang::En); // de-DE falls back to English
    }

    #[test]
    fn the_refresh_row_reads_well_in_every_language() {
        let row = |lang| {
            [
                Key::SettingsRefresh,
                Key::SettingsRefreshSub,
                Key::SettingsHardware,
                Key::SettingsLimits,
            ]
            .map(|key| t(lang, key))
        };
        assert_eq!(
            row(Lang::En),
            [
                "Refresh (seconds)",
                "Hardware 1–5 · Limits 60–3600",
                "Hardware",
                "Limits"
            ]
        );
        assert_eq!(
            row(Lang::PtBr),
            [
                "Atualização (segundos)",
                "Hardware 1–5 · Limites 60–3600",
                "Hardware",
                "Limites"
            ]
        );
        assert_eq!(
            row(Lang::Es),
            [
                "Actualización (segundos)",
                "Hardware 1–5 · Límites 60–3600",
                "Hardware",
                "Límites"
            ]
        );
    }

    #[test]
    fn fmt_substitutes_placeholder() {
        assert_eq!(fmt(t(Lang::En, Key::ResetsFmt), "15:00"), "resets at 15:00");
    }
}
