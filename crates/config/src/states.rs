use support::on_off_state;

/// Runtime feature flags for the application.
#[derive(Default, Eq, PartialEq, Debug, Clone)]
pub struct States {
    pub autologin: AutologinState,
    pub demo: DemoState,
    pub signups: SignupsState,
    pub production: ProductionState,
}

on_off_state!(AutologinState);
on_off_state!(DemoState);
on_off_state!(SignupsState);
on_off_state!(ProductionState);
