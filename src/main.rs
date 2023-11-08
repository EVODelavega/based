use clap::{Parser, ValueEnum};
use std::fmt;
// use std::io;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    Binary,
    Octal,
    Decimal,
    Hex,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short)]
    value: String,
    #[arg(short, long, value_enum, default_value_t = Mode::Decimal)]
    mode: Mode,
    #[arg(short, long, value_enum)]
    to: Option<Mode>,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Cli {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "value: {}, mode: {}", self.value, self.mode)
    }
}

impl Mode {
    fn to_radix(&self) -> u32 {
        match self {
            Mode::Binary => 2,
            Mode::Octal => 8,
            Mode::Decimal => 10,
            Mode::Hex => 16,
        }
    }

    fn print_val(&self, v: i64) {
        match self {
            Mode::Binary => println!("BIN: {:b}", v),
            Mode::Octal => println!("OCT: {:o}", v),
            Mode::Decimal => println!("DEC: {}", v),
            Mode::Hex => println!("HEX: {:#08X}", v),
        }
    }

    fn fmt_val(&self, v: i64) -> String {
        match self {
            Mode::Binary => format!("{:b}", v),
            Mode::Octal => format!("{:o}", v),
            Mode::Decimal => format!("{}", v),
            Mode::Hex => format!("{:#08X}", v),
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let input = i64::from_str_radix(cli.value.as_str(), cli.mode.to_radix()).expect("Failed to parse input value");
    print_values(input, cli.to);
}

fn print_values(v: i64, to: Option<Mode>) {
    match to {
        Some(m) => println!("Formatted: {}", m.fmt_val(v)),
        None => {
            Mode::Binary.print_val(v);
            Mode::Octal.print_val(v);
            Mode::Decimal.print_val(v);
            Mode::Hex.print_val(v);
        },
    }
}
