#[derive(Default, Eq, PartialEq, Debug, Clone)]
pub struct States {
    pub autologin: AutologinState,
    pub demo: DemoState,
}

#[derive(Default, Eq, PartialEq, Debug, Clone)]
pub enum AutologinState {
    #[default]
    Off,
    On,
}

impl From<AutologinState> for bool {
    fn from(value: AutologinState) -> Self {
        match value {
            AutologinState::Off => false,
            AutologinState::On => true,
        }
    }
}

impl From<&AutologinState> for bool {
    fn from(value: &AutologinState) -> Self {
        match value {
            AutologinState::Off => false,
            AutologinState::On => true,
        }
    }
}

impl From<bool> for AutologinState {
    fn from(value: bool) -> Self {
        match value {
            false => AutologinState::Off,
            true => AutologinState::On,
        }
    }
}

#[derive(Default, Eq, PartialEq, Debug, Clone)]
pub enum DemoState {
    #[default]
    Off,
    On,
}

impl From<DemoState> for bool {
    fn from(value: DemoState) -> Self {
        match value {
            DemoState::Off => false,
            DemoState::On => true,
        }
    }
}

impl From<&DemoState> for bool {
    fn from(value: &DemoState) -> Self {
        match value {
            DemoState::Off => false,
            DemoState::On => true,
        }
    }
}

impl From<bool> for DemoState {
    fn from(value: bool) -> Self {
        match value {
            false => DemoState::Off,
            true => DemoState::On,
        }
    }
}
