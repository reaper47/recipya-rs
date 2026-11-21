use support::on_off_state;

/// Runtime feature flags for the application.
#[derive(Default, Eq, PartialEq, Debug, Clone)]
pub struct States {
    pub autologin: AutologinState,
    pub demo: DemoState,
    pub signups: SignupsState,
    pub app_env: AppEnvState,
}

on_off_state!(AutologinState);
on_off_state!(DemoState);
on_off_state!(SignupsState);

const DEV: &str = "dev";
const TEST: &str = "test";
const PROD: &str = "prod";

/// Defines a the environment of the application.
#[derive(Default, Eq, PartialEq, Debug, Copy, Clone)]
pub enum AppEnvState {
    #[default]
    Dev,
    Test,
    Prod,
}

impl From<AppEnvState> for &str {
    fn from(value: AppEnvState) -> Self {
        match value {
            AppEnvState::Dev => DEV,
            AppEnvState::Test => TEST,
            AppEnvState::Prod => PROD,
        }
    }
}

impl From<&AppEnvState> for &str {
    fn from(value: &AppEnvState) -> Self {
        match value {
            AppEnvState::Dev => DEV,
            AppEnvState::Test => TEST,
            AppEnvState::Prod => PROD,
        }
    }
}

impl From<&str> for AppEnvState {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            TEST => Self::Test,
            PROD => Self::Prod,
            _ => Self::Dev,
        }
    }
}
