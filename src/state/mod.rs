use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)] // Why am I doing this? -_-
pub enum ThemeChoice {
    System,
    Dark,
    LightGray,
    Nord,
}

impl ThemeChoice {
    pub const fn all() -> [Self; 4] {
        [Self::System, Self::Dark, Self::LightGray, Self::Nord]
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::System => "System",
            Self::Dark => "Dark",
            Self::LightGray => "Light Gray",
            Self::Nord => "Nord",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextScale {
    Pct90,
    Pct100,
    Pct110,
    Pct125,
    Pct140,
}

impl TextScale {
    pub const fn all() -> [Self; 5] {
        [Self::Pct90, Self::Pct100, Self::Pct110, Self::Pct125, Self::Pct140]
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Pct90 => "90%",
            Self::Pct100 => "100%",
            Self::Pct110 => "110%",
            Self::Pct125 => "125%",
            Self::Pct140 => "140%",
        }
    }

    pub const fn as_multiplier(self) -> f32 {
        match self {
            Self::Pct90 => 0.90,
            Self::Pct100 => 1.00,
            Self::Pct110 => 1.10,
            Self::Pct125 => 1.25,
            Self::Pct140 => 1.40,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiSettings {
    pub theme:ThemeChoice,
    pub text_scale: TextScale,
}

impl Default for UiSettings {
    fn default() -> Self {
        Self {
            theme: ThemeChoice::System,
            text_scale: TextScale::Pct100,
        }
    }
}

#[derive(Debug, Default)]
pub struct ToastState {
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersistedState {
    pub settings: UiSettings,
}

impl Default for PersistedState {
    fn default() -> Self {
        Self {
            settings: UiSettings::default(),
        }
    }
}

#[derive(Debug)]
pub struct AppState {
    pub active_tab: ActiveTab,
    pub input_users_raw: String,
    pub comparison_output: String,
    pub toast: ToastState,
    pub persisted: PersistedState,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            active_tab: ActiveTab::Home,
            input_users_raw: String::new(),
            comparison_output: String::new(),
            toast: ToastState::default(),
            persisted: PersistedState::default(),
        }
    }
}