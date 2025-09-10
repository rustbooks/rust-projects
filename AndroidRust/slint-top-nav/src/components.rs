use slint::SharedString;

#[derive(Default, Clone)]
pub struct MenuItem {
    pub name: SharedString,
    pub icon: SharedString,
}

#[derive(Clone)]
pub struct FormData {
    pub menu: String,
    pub input1: String,
    pub input2: String,
}