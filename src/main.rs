mod args;
mod chars;

use args::get_args;
use chars::{GetCharsOption, get_chars};

use arboard::Clipboard;
use colored::*;
use rand::{random_range, rng, seq::IndexedRandom};
use zxcvbn::zxcvbn;

fn main() {
    let args = get_args();
    let mut clipboard = Clipboard::new().unwrap();
    let length: usize = args.length.unwrap_or(random_range(4..255));

    let mut chars = String::new();

    match args.chars {
        Some(ref _chars) => chars += _chars,
        None => {
            chars = get_chars(GetCharsOption {
                digits: !args.ndigits,
                lower: !args.nlower,
                upper: !args.nupper,
                symbol: !args.nsymbol,
            });
        }
    }

    let chars: &str = &chars;
    let chars: Vec<char> = chars.chars().collect();

    let mut password = String::new();
    let mut _rng = rng();

    for _ in 0..length {
        let _char = chars.choose(&mut _rng).unwrap();

        password.push(*_char);
    }

    println!("{}", password.green().bold());

    if args.score {
        let estimate = zxcvbn(&password, &[]);

        let answer = format!("Сила пароля = {}", estimate.score());

        println!("{}", answer.red().bold());
    }

    if args.clipboard {
        match clipboard.set_text(&password) {
            Ok(_) => println!("Скопировано в буфер обмена!"),
            Err(e) => eprintln!("Ошибка копирования в буфер обмена: {}", e),
        }
    }
}
