use crate::cli::Cli;
use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::Parser;
use chumsky::span::SimpleSpan;
use color_eyre::Result;
use std::fs::{read_to_string, write};
use std::process::exit;

mod cli;
mod compiler;
mod parser;

#[derive(Debug)]
struct Instruction {
    meta: MetaInstruction,
    span: SimpleSpan,
}

#[derive(Debug)]
enum MetaInstruction {
    AInstruction(i16),
    CInstruction(Dest, Comp, Jump),
    UnresolvedAInstruction(String),
    Label(String),
}
#[derive(Debug, Clone)]
struct Dest(i8);

#[derive(Debug, Clone)]
struct Jump(i8);
#[derive(Debug)]
struct Comp(i8);

fn main() -> Result<()> {
    let file = Cli::file();
    let content = read_to_string(file)?;

    let parsing_result = match parser::parser().parse(content.as_str()).into_result() {
        Ok(val) => val,
        Err(errs) => {
            for e in errs {
                let span = ("input", e.span().into_range());
                Report::build(ReportKind::Error, span.clone())
                    .with_message("Failed to parse.")
                    .with_label(
                        Label::new(span)
                            .with_message(e.to_string())
                            .with_color(Color::Red),
                    )
                    .finish()
                    .eprint(("input", Source::from(&content)))?;
            }

            exit(1)
        }
    };

    let output = compiler::compile(parsing_result, &content)
        .iter()
        .map(|i| format!("{i:016b}"))
        .collect::<Vec<_>>()
        .join("\n");

    write("output.hack", output)?;

    Ok(())
}
