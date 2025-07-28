pub mod dashboard;
pub mod login;

#[derive(Clone, PartialEq)]
pub struct UserData {
    pub email: String,
    pub api_key: String,
}

#[derive(Clone, PartialEq)]
pub enum AppState {
    Login,
    Dashboard(UserData),
}
