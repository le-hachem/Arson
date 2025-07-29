use super::UserData;

#[derive(Clone, PartialEq, Debug)]
pub enum Theme {
    Light,
    Blue,
    Dark,
    Terminal,
}

impl Default for Theme {
    fn default() -> Self {
        Self::Light
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum AppState {
    Login,
    Dashboard(UserData),
}

#[derive(Clone, PartialEq, Debug)]
pub enum DashboardView {
    Map,
    DataList,
}

impl Default for DashboardView {
    fn default() -> Self {
        Self::DataList
    }
}
