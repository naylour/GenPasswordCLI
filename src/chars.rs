const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "0123456789";
const SYMBOLS: &str = "!@#$^*()-_=+[]{};:'\",.<>/|\\`~";

#[derive(Debug)]
pub struct GetCharsOption {
    pub digits: bool,
    pub lower: bool,
    pub upper: bool,
    pub symbol: bool,
}

impl Default for GetCharsOption {
    fn default() -> Self {
        Self {
            digits: true,
            lower: true,
            symbol: true,
            upper: true,
        }
    }
}

pub fn get_chars(option: GetCharsOption) -> String {
    let mut charset = String::new();

    if option.digits {
        charset.push_str(DIGITS);
    }
    if option.upper {
        charset.push_str(UPPERCASE);
    }
    if option.lower {
        charset.push_str(LOWERCASE);
    }
    if option.symbol {
        charset.push_str(SYMBOLS);
    }

    charset
}
