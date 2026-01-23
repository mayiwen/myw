pub mod link;
pub mod title;
#[derive(Clone, PartialEq)]
pub struct Login {
    pub token: String,
}
#[derive(Clone, PartialEq)]
pub struct SettingTab {
    pub value: u64,
}
