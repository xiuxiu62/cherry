// use crossterm::style::Color;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub foreground_color: Option<Color>,
    pub background_color: Option<Color>,
    pub underline_color: Option<Color>,
    pub alternate_screen: bool,
    pub line_wrapping: bool,
    pub mouse_capture: bool,
}

impl Config {
    pub fn new(
        foreground_color: Option<Color>,
        background_color: Option<Color>,
        underline_color: Option<Color>,
        alternate_screen: bool,
        line_wrapping: bool,
        mouse_capture: bool,
    ) -> Self {
        Self {
            foreground_color,
            background_color,
            underline_color,
            alternate_screen,
            line_wrapping,
            mouse_capture,
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Color {
    Reset,
    Black,
    DarkGrey,
    Red,
    DarkRed,
    Green,
    DarkGreen,
    Yellow,
    DarkYellow,
    Blue,
    DarkBlue,
    Magenta,
    DarkMagenta,
    Cyan,
    DarkCyan,
    White,
    Rgb { r: u8, g: u8, b: u8 },
    AnsiValue(u8),
}

impl Into<crossterm::style::Color> for Color {
    fn into(self) -> crossterm::style::Color {
        self.into()
    }
}
