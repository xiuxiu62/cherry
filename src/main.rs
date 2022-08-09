use std::path::PathBuf;

use cherry::{
    error::{Error, Result, SerdeError},
    Config, Editor, FrameBuffer, Span, Terminal,
};

const DEFAULT_CONFIG: &str = "config.ron";

fn main() -> Result<()> {
    // tracing_subscriber::fmt::init();
    let mut app = App::new()?;
    app.run()?;

    let message = format!("{app:#?}");
    drop(app);

    println!("{message}");

    Ok(())
}

fn _raw_terminal() -> Result<()> {
    let config = load_config(DEFAULT_CONFIG)?;
    let mut terminal = Terminal::new(config)?;

    terminal.initialize()?;
    std::thread::sleep(std::time::Duration::from_millis(1000));

    Ok(())
}

#[derive(Debug)]
struct App(Editor);

impl App {
    pub fn new() -> Result<Self> {
        let config = load_config(DEFAULT_CONFIG)?;
        let terminal = Terminal::new(config)?;

        let view_span = Span {
            start: 0,
            end: terminal.size()?.1 as usize,
        };
        let buffer = FrameBuffer::try_from_path(PathBuf::from("config.ron"), view_span)?;
        let mut editor = Editor::new(terminal, buffer);
        editor.initialize()?;

        Ok(Self(editor))
    }

    pub fn run(&mut self) -> Result<()> {
        self.0.run()
    }
}

fn load_config(path: &str) -> Result<Config> {
    let contents = std::fs::read_to_string(path)?;

    match ron::from_str(&contents) {
        Ok(config) => Ok(config),
        Err(err) => Err(Error::Serde(SerdeError::Deserialize(err.to_string()))),
    }
}
