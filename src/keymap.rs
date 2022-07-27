use crossterm::event::KeyCode;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[macro_export]
macro_rules! hashmap {
    (<$key_ty:ty, $value_ty:ty> [$($key:expr => $value:expr),*]) => {{
        let mut map: HashMap<$key_ty, $value_ty> = HashMap::new();
        $(map.insert($key.to_owned(), $value);)*

        map
    }}
}

lazy_static! {
    pub static ref CHAR_MAP: HashMap<KeyCode, char> = hashmap!(<KeyCode, char> [
       KeyCode::Char('a') => 'a',
       KeyCode::Char('b') => 'b',
       KeyCode::Char('c') => 'c',
       KeyCode::Char('d') => 'd',
       KeyCode::Char('e') => 'e',
       KeyCode::Char('f') => 'f',
       KeyCode::Char('g') => 'g',
       KeyCode::Char('h') => 'h',
       KeyCode::Char('i') => 'i',
       KeyCode::Char('j') => 'j',
       KeyCode::Char('k') => 'k',
       KeyCode::Char('l') => 'l',
       KeyCode::Char('m') => 'm',
       KeyCode::Char('n') => 'n',
       KeyCode::Char('o') => 'o',
       KeyCode::Char('p') => 'p',
       KeyCode::Char('q') => 'q',
       KeyCode::Char('r') => 'r',
       KeyCode::Char('s') => 's',
       KeyCode::Char('t') => 't',
       KeyCode::Char('u') => 'u',
       KeyCode::Char('v') => 'v',
       KeyCode::Char('w') => 'w',
       KeyCode::Char('x') => 'x',
       KeyCode::Char('y') => 'y',
       KeyCode::Char('z') => 'z',

       KeyCode::Char('A') => 'A',
       KeyCode::Char('B') => 'B',
       KeyCode::Char('C') => 'C',
       KeyCode::Char('D') => 'D',
       KeyCode::Char('E') => 'E',
       KeyCode::Char('F') => 'F',
       KeyCode::Char('G') => 'G',
       KeyCode::Char('H') => 'H',
       KeyCode::Char('I') => 'I',
       KeyCode::Char('J') => 'J',
       KeyCode::Char('K') => 'K',
       KeyCode::Char('L') => 'L',
       KeyCode::Char('M') => 'M',
       KeyCode::Char('N') => 'N',
       KeyCode::Char('O') => 'O',
       KeyCode::Char('P') => 'P',
       KeyCode::Char('Q') => 'Q',
       KeyCode::Char('R') => 'R',
       KeyCode::Char('S') => 'S',
       KeyCode::Char('T') => 'T',
       KeyCode::Char('U') => 'U',
       KeyCode::Char('V') => 'V',
       KeyCode::Char('W') => 'W',
       KeyCode::Char('X') => 'X',
       KeyCode::Char('Y') => 'Y',
       KeyCode::Char('Z') => 'Z',

       KeyCode::Char('1') => '1',
       KeyCode::Char('2') => '2',
       KeyCode::Char('3') => '3',
       KeyCode::Char('4') => '4',
       KeyCode::Char('5') => '5',
       KeyCode::Char('6') => '6',
       KeyCode::Char('7') => '7',
       KeyCode::Char('8') => '8',
       KeyCode::Char('9') => '9',
       KeyCode::Char('0') => '0',
       KeyCode::Char('=') => '=',
       KeyCode::Char('+') => '+',
       KeyCode::Char('-') => '-',
       KeyCode::Char('_') => '_',

       KeyCode::Char('!') => '!',
       KeyCode::Char('@') => '@',
       KeyCode::Char('#') => '#',
       KeyCode::Char('$') => '$',
       KeyCode::Char('%') => '%',
       KeyCode::Char('^') => '^',
       KeyCode::Char('&') => '&',
       KeyCode::Char('*') => '*',
       KeyCode::Char('(') => '(',
       KeyCode::Char(')') => ')',


       KeyCode::Char('[') => '[',
       KeyCode::Char('{') => '{',
       KeyCode::Char(']') => ']',
       KeyCode::Char('}') => '}',
       KeyCode::Char(',') => ',',
       KeyCode::Char('<') => '<',
       KeyCode::Char('.') => '.',
       KeyCode::Char('>') => '>',
       KeyCode::Char('/') => '/',
       KeyCode::Char('?') => '?',
       KeyCode::Char(';') => ';',
       KeyCode::Char(':') => ':',
       KeyCode::Char('\'') => '\'',
       KeyCode::Char('"') => '"',
       KeyCode::Char('`') => '`',
       KeyCode::Char('~') => '~',

       KeyCode::Char(' ') => ' ',
       KeyCode::Tab => '\t'
    ]);
}
