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

impl From<ColorConfig> for Color {
    fn from(color_config: ColorConfig) -> Self {
        match color_config {
            ColorConfig::Reset => Self::Reset,
            ColorConfig::Black => Self::Black,
            ColorConfig::DarkGrey => Self::DarkGrey,
            ColorConfig::Red => Self::Red,
            ColorConfig::DarkRed => Self::DarkRed,
            ColorConfig::Green => Self::Green,
            ColorConfig::DarkGreen => Self::DarkGreen,
            ColorConfig::Yellow => Self::Yellow,
            ColorConfig::DarkYellow => Self::DarkYellow,
            ColorConfig::Blue => Self::Blue,
            ColorConfig::DarkBlue => Self::DarkBlue,
            ColorConfig::Magenta => Self::Magenta,
            ColorConfig::DarkMagenta => Self::DarkMagenta,
            ColorConfig::Cyan => Self::Cyan,
            ColorConfig::DarkCyan => Self::DarkCyan,
            ColorConfig::White => Self::White,
            ColorConfig::Rgb { r, g, b } => Self::Rgb { r, g, b },
            ColorConfig::AnsiValue(value) => Self::AnsiValue(value),
        }
    }
}
