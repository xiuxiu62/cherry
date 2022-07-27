use crossterm::style::Color;

pub struct Config {
    pub foreground_color: Option<Color>,
    pub background_color: Option<Color>,
    pub underline_color: Option<Color>,
    pub line_wrapping: bool,
    pub mouse_capture: bool,
}

impl Config {
    pub fn new(
        foreground_color: Option<Color>,
        background_color: Option<Color>,
        underline_color: Option<Color>,
        line_wrapping: bool,
        mouse_capture: bool,
    ) -> Self {
        Self {
            foreground_color,
            background_color,
            underline_color,
            line_wrapping,
            mouse_capture,
        }
    }
}
