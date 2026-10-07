use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    file: Option<String>,
}

impl Cli {
    pub fn file() -> String {
        Cli::parse().file.unwrap_or("main.asm".to_string())
    }
}
