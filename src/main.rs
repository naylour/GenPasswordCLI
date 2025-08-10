use arboard::Clipboard;
use clap::Parser;
use colored::*;
use rand::{rand_core::le, random_range, rng, seq::IndexedRandom};
use zxcvbn::zxcvbn;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'l', long, help = "Длина пароля(по умолчанию до 255)")]
    length: Option<usize>,

    #[arg(long, help = "Свой набор символов")]
    chars: Option<String>,

    #[arg(short = 's', long, help = "Без символов")]
    nsymbol: bool,

    #[arg(long, help = "Без верхнего регистра")]
    nupper: bool,

    #[arg(long, help = "Без нижнего регистра")]
    nlower: bool,

    #[arg(short = 'd', long, help = "Без чисел")]
    ndigits: bool,

    #[arg(long, help = "Скопировать в буфер обмена")]
    clipboard: bool,

    #[arg(long, help = "Показать силу пароля")]
    score: bool,
}

const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "0123456789";
const SYMBOLS: &str = "!@#$^*()-_=+[]{};:'\",.<>/|\\`~";

fn main() {
    let args = Args::parse();
    let mut clipboard = Clipboard::new().unwrap();
    let mut length: usize = 0;

    let mut chars = String::new();

    match args.length {
        Some(_length) => length = _length,
        None => length = random_range(4..255),
    }

    match args.chars {
        Some(ref _chars) => chars += _chars,
        None => {
            if !args.ndigits {
                chars += DIGITS;
            }
            if !args.nupper {
                chars += UPPERCASE;
            }
            if !args.nlower {
                chars += LOWERCASE;
            }
            if !args.nsymbol {
                chars += SYMBOLS;
            }
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
