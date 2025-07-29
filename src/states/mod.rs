pub mod dashboard;
pub mod dataview;
pub mod login;
pub mod map;

#[derive(Clone, PartialEq)]
pub struct UserData {
    pub email: String,
    pub api_key: String,
}

#[derive(Clone, PartialEq)]
pub enum DashboardView {
    Data,
    Map,
}

#[derive(Clone, PartialEq)]
pub enum AppState {
    Login,
    Dashboard(UserData, DashboardView),
}
