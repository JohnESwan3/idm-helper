#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveTab {
    Home,
    Settings,
    Approvers,
}

impl ActiveTab {
    pub const fn all() -> [Self; 3] {
        [Self::Home, Self::Settings, Self::Approvers]
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Home => "Home",
            Self::Settings => "Settings",
            Self::Approvers => "Approvers",
        }
    }
}

#[derive(Debug, Default)]
pub struct ToastState {
    pub message: Option<String>,
}

#[derive(Debug)]
pub struct AppState {
    pub active_tab: ActiveTab,
    pub input_users_raw: String,
    pub comparison_output: String,
    pub toast: ToastState,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            active_tab: ActiveTab::Home,
            input_users_raw: String::new(),
            comparison_output: String::new(),
            toast: ToastState::default(),
        }
    }
}