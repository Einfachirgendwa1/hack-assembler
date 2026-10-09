use crate::{new_dest, Comp, Dest, Jump, MetaInstruction};
use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::error::Rich;
use chumsky::input::MapExtra;
use chumsky::prelude::{any, choice, empty, end, just};
use chumsky::span::Spanned;
use chumsky::text::{newline, whitespace, Char};
use chumsky::{extra, text, IterParser, Parser};
use std::process::exit;

pub fn parser<'a>(
    source: Source,
) -> impl Parser<'a, &'a str, Vec<Spanned<MetaInstruction>>, extra::Err<Rich<'a, char>>> {
    let comment = just("//")
        .ignore_then(any().and_is(newline().not()).repeated().collect::<String>())
        .map(MetaInstruction::Comment);

    let end_of_line = newline().or(end());
    let termination = any()
        .and_is(whitespace())
        .and_is(end_of_line.not())
        .repeated()
        .ignore_then(end_of_line)
        .or(just("//").ignore_then(any().and_is(end_of_line.not()).ignore_then(end_of_line)));

    let a_instruction = just("@")
        .ignore_then(
            text::int(10)
                .from_str()
                .map_with(move |res, extra: &mut MapExtra<&str, _>| match res {
                    Ok(val) => MetaInstruction::AInstruction(val),
                    Err(err) => {
                        let span = ("input", extra.span().into_range());

                        Report::build(ReportKind::Error, span.clone())
                            .with_message("Failed to parse number!")
                            .with_label(
                                Label::new(span)
                                    .with_message(err.to_string())
                                    .with_color(Color::Red),
                            )
                            .finish()
                            .eprint(("input", source.clone()))
                            .ok();

                        exit(1)
                    }
                })
                .or(label_text().map(MetaInstruction::UnresolvedAInstruction)),
        )
        .then_ignore(termination);

    let label = label_text()
        .delimited_by(just("("), just(")"))
        .map(MetaInstruction::Label)
        .then_ignore(termination);

    let c_instruction = dest()
        .or(empty().to(Dest(0)))
        .then(comp())
        .then(jump().or(empty().to(Jump(0))))
        .map(|((dest, comp), jump)| MetaInstruction::CInstruction(dest, comp, jump))
        .then_ignore(termination);

    a_instruction
        .or(c_instruction)
        .or(label)
        .or(comment)
        .spanned()
        .padded()
        .repeated()
        .collect::<Vec<_>>()
}

fn label_text<'a>() -> impl Parser<'a, &'a str, String, extra::Err<Rich<'a, char>>> {
    any()
        .filter(|c: &char| c.is_ident_continue() || ['_', '.', '$'].contains(c))
        .repeated()
        .at_least(1)
        .collect()
}

pub fn comp<'a>() -> impl Parser<'a, &'a str, Comp, extra::Err<Rich<'a, char>>> {
    choice([
        just("0").to(0b0_101010),
        just("1").to(0b0_111111),
        just("-1").to(0b0_111010),
        just("D+1").to(0b0_011111),
        just("A+1").to(0b0_110111),
        just("M+1").to(0b1_110111),
        just("D-1").to(0b0_001110),
        just("A-1").to(0b0_110010),
        just("M-1").to(0b1_110010),
        just("D+A").to(0b0_000010),
        just("D+M").to(0b1_000010),
        just("D-A").to(0b0_010011),
        just("D-M").to(0b1_010011),
        just("A-D").to(0b0_000111),
        just("M-D").to(0b1_000111),
        just("D&A").to(0b0_000000),
        just("D&M").to(0b1_000000),
        just("D|A").to(0b0_010101),
        just("D|M").to(0b1_010101),
        just("!D").to(0b0_001101),
        just("!A").to(0b0_110001),
        just("!M").to(0b1_110001),
        just("-D").to(0b0_001111),
        just("-A").to(0b0_110011),
        just("-M").to(0b1_110011),
        just("D").to(0b0_001100),
        just("A").to(0b0_110000),
        just("M").to(0b1_110000),
    ])
    .map(Comp)
}

fn dest<'a>() -> impl Parser<'a, &'a str, Dest, extra::Err<Rich<'a, char>>> {
    choice([
        just("ADM").to(new_dest(1, 1, 1)),
        just("AD").to(new_dest(1, 1, 0)),
        just("AM").to(new_dest(1, 0, 1)),
        just("A").to(new_dest(1, 0, 0)),
        just("DM").to(new_dest(0, 1, 1)),
        just("MD").to(new_dest(0, 1, 1)),
        just("D").to(new_dest(0, 1, 0)),
        just("M").to(new_dest(0, 0, 1)),
    ])
    .then_ignore(just("="))
}

fn jump<'a>() -> impl Parser<'a, &'a str, Jump, extra::Err<Rich<'a, char>>> {
    just(";").ignore_then(
        choice([
            just("JGT").to(1),
            just("JEQ").to(2),
            just("JGE").to(3),
            just("JLT").to(4),
            just("JNE").to(5),
            just("JLE").to(6),
            just("JMP").to(7),
        ])
        .map(Jump),
    )
}
