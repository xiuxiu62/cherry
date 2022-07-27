mod config;
mod editor;
mod error;
mod keymap;
mod terminal;
mod text_buffer;

use std::rc::Rc;

pub use config::Config;
use editor::Editor;
use error::{Error, Result, SerdeError};
pub use keymap::CHAR_MAP;
use terminal::Terminal;
use text_buffer::TextBuffer;

const DEFAULT_CONFIG: &'static str = "config.ron";

fn main() -> Result<()> {
    let config = Rc::new(load_config(DEFAULT_CONFIG)?);
    let terminal = Terminal::new(config)?;
    let buffer = TextBuffer::default();
    let mut editor = Editor::new(terminal, buffer);

    editor.initialize()?;
    editor.run()?;

    Ok(())
}

fn load_config(path: &str) -> Result<Config> {
    let contents = std::fs::read_to_string(path)?;

    match ron::from_str(&contents) {
        Ok(config) => Ok(config),
        Err(err) => Err(Error::Serde(SerdeError::Deserialize(err.to_string()))),
    }
}

// fn main() {
//     let theme = config::ThemeConfig::new(
//         Some(config::ColorConfig::Red),
//         Some(config::ColorConfig::Rgb {
//             r: 0xcc,
//             g: 0x44,
//             b: 0xff,
//         }),
//         Some(config::ColorConfig::AnsiValue(0xff)),
//     );

//     let config = Config::new(theme, false, false, true);
//     // let data = toml::ser::to_string_pretty(&config).unwrap();
//     let pretty = ron::ser::PrettyConfig::new()
//         .separate_tuple_members(true)
//         .enumerate_arrays(true);

//     let s = ron::ser::to_string(&config).unwrap();
//     println!("config:\n{s}\n");

//     let pretty_s = ron::ser::to_string_pretty(&config, pretty).unwrap();
//     println!("pretty config:\n{pretty_s}\n");
//     // let data = ron::ser::to_string_pretty(&config);

//     // println!("{data}");
// }
