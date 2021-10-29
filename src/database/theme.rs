use crate::data::Data;

pub struct Theme {}

impl Theme {
    pub fn new() -> Theme {
        Theme {}
    }

    pub fn render(&self, data: Data) -> String {
        String::from("rendered")
    }
}
