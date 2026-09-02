#[derive(Default, Eq, PartialEq, Debug, Clone)]
pub struct States {
    pub autologin: AutologinState,
    pub demo: DemoState,
    pub signups: SignupsState,
    pub production: ProductionState,
}

#[derive(Default, Eq, PartialEq, Debug, Copy, Clone)]
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
        if value { Self::On } else { Self::Off }
    }
}

#[derive(Default, Eq, PartialEq, Debug, Copy, Clone)]
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
        if value { Self::On } else { Self::Off }
    }
}

#[derive(Default, Eq, PartialEq, Debug, Copy, Clone)]
pub enum SignupsState {
    #[default]
    Off,
    On,
}

impl From<SignupsState> for bool {
    fn from(value: SignupsState) -> Self {
        match value {
            SignupsState::Off => false,
            SignupsState::On => true,
        }
    }
}

impl From<&SignupsState> for bool {
    fn from(value: &SignupsState) -> Self {
        match value {
            SignupsState::Off => false,
            SignupsState::On => true,
        }
    }
}

impl From<bool> for SignupsState {
    fn from(value: bool) -> Self {
        if value { Self::On } else { Self::Off }
    }
}

#[derive(Default, Eq, PartialEq, Debug, Copy, Clone)]
pub enum ProductionState {
    #[default]
    Off,
    On,
}

impl From<ProductionState> for bool {
    fn from(value: ProductionState) -> Self {
        match value {
            ProductionState::Off => false,
            ProductionState::On => true,
        }
    }
}

impl From<&ProductionState> for bool {
    fn from(value: &ProductionState) -> Self {
        match value {
            ProductionState::Off => false,
            ProductionState::On => true,
        }
    }
}

impl From<bool> for ProductionState {
    fn from(value: bool) -> Self {
        if value { Self::On } else { Self::Off }
    }
}
