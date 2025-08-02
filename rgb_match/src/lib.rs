#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        first = first ^second;
        second = first ^ second;
        first = first ^second;

        Self {
            first,
            second,
            ..
        }
    }
}