#![deny(unsafe_code)]

use cherry::{
    error::{Error, Result, SerdeError},
    Config, Editor, FrameBuffer, Span, Terminal,
};
use std::path::PathBuf;
use structopt::StructOpt;

const DEFAULT_CONFIG: &str = "~/.config/cherry/config.ron";

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(short, long, parse(from_os_str), about = "Alternate config path")]
    pub config: Option<PathBuf>,
    #[structopt(parse(from_os_str), about = "Entry to be edited")]
    pub path: Option<PathBuf>,
}

fn main() -> Result<()> {
    // tracing_subscriber::fmt::init();
    let options = Options::from_args();
    let config = {
        let path = match options.config {
            Some(path) => path,
            None => PathBuf::from(DEFAULT_CONFIG),
        };

        load_config(path)
    }?;

    let terminal = Terminal::new(config)?;
    let view_span = Span {
        start: 0,
        end: terminal.size()?.1 as usize,
    };
    let buffer = match options.path {
        Some(path) => FrameBuffer::try_from_path(path, view_span)?,
        None => FrameBuffer::new(vec![], view_span),
    };

    let mut editor = Editor::new(terminal, buffer);
    editor.initialize()?;
    editor.run()?;

    let message = format!("{}", editor.buffer);
    let history = editor.format_history();
    drop(editor);

    println!("{message}\n");
    println!("{history}",);

    Ok(())
}

fn load_config(path: PathBuf) -> Result<Config> {
    let contents = std::fs::read_to_string(path)?;

    match ron::from_str(&contents) {
        Ok(config) => Ok(config),
        Err(err) => Err(Error::Serde(SerdeError::Deserialize(err.to_string()))),
    }
}
