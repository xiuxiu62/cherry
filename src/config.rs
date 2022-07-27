use crossterm::style::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub theme: ThemeConfig,
    pub alternate_screen: bool,
    pub line_wrapping: bool,
    pub mouse_capture: bool,
}

impl Config {
    pub fn new(
        theme: ThemeConfig,
        alternate_screen: bool,
        line_wrapping: bool,
        mouse_capture: bool,
    ) -> Self {
        Self {
            theme,
            alternate_screen,
            line_wrapping,
            mouse_capture,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ThemeConfig {
    pub foreground_color: Option<ColorConfig>,
    pub background_color: Option<ColorConfig>,
    pub underline_color: Option<ColorConfig>,
}

impl ThemeConfig {
    pub fn new(
        foreground_color: Option<ColorConfig>,
        background_color: Option<ColorConfig>,
        underline_color: Option<ColorConfig>,
    ) -> Self {
        Self {
            foreground_color,
            background_color,
            underline_color,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ColorConfig {
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

impl Into<crossterm::style::Color> for ColorConfig {
    fn into(self) -> crossterm::style::Color {
        match self {
            Self::Reset => Color::Reset,
            Self::Black => Color::Black,
            Self::DarkGrey => Color::DarkGrey,
            Self::Red => Color::Red,
            Self::DarkRed => Color::DarkRed,
            Self::Green => Color::Green,
            Self::DarkGreen => Color::DarkGreen,
            Self::Yellow => Color::Yellow,
            Self::DarkYellow => Color::DarkYellow,
            Self::Blue => Color::Blue,
            Self::DarkBlue => Color::DarkBlue,
            Self::Magenta => Color::Magenta,
            Self::DarkMagenta => Color::DarkMagenta,
            Self::Cyan => Color::Cyan,
            Self::DarkCyan => Color::DarkCyan,
            Self::White => Color::White,
            Self::Rgb { r, g, b } => Color::Rgb { r, g, b },
            Self::AnsiValue(value) => Color::AnsiValue(value),
        }
    }
}
