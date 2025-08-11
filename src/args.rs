use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'l', long, help = "Длина пароля(по умолчанию до 255)")]
    pub length: Option<usize>,

    #[arg(long, help = "Свой набор символов")]
    pub chars: Option<String>,

    #[arg(short = 's', long, help = "Без символов")]
    pub nsymbol: bool,

    #[arg(long, help = "Без верхнего регистра")]
    pub nupper: bool,

    #[arg(long, help = "Без нижнего регистра")]
    pub nlower: bool,

    #[arg(short = 'd', long, help = "Без чисел")]
    pub ndigits: bool,

    #[arg(long, help = "Скопировать в буфер обмена")]
    pub clipboard: bool,

    #[arg(long, help = "Показать силу пароля")]
    pub score: bool,
}

pub fn get_args() -> Args {
    Args::parse()
}
