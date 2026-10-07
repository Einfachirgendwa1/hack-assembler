use crate::{Comp, Dest, Instruction, Jump, MetaInstruction};
use chumsky::error::Rich;
use chumsky::prelude::{empty, just};
use chumsky::{IterParser, Parser, extra, text};

pub fn parser<'a>() -> impl Parser<'a, &'a str, Vec<Instruction>, extra::Err<Rich<'a, char>>> {
    let a_instruction = just("@").ignore_then(
        text::int::<_, extra::Err<Rich<'a, char>>>(10)
            .from_str()
            .unwrapped()
            .map_with(|i, extra| Instruction {
                meta: MetaInstruction::AInstruction(i),
                span: extra.span(),
            })
            .or(text::ident().map_with(|s: &str, extra| Instruction {
                meta: MetaInstruction::UnresolvedAInstruction(s.to_string()),
                span: extra.span(),
            })),
    );

    let label = text::ident()
        .delimited_by(just("("), just(")"))
        .map_with(|str: &str, extra| Instruction {
            meta: MetaInstruction::Label(str.to_string()),
            span: extra.span(),
        });

    let dest = text::ident().then_ignore(just("=")).map(|s: &str| {
        let c = |ch: char| s.contains(ch) as i8;
        Dest(c('M') | (c('D') << 1) | (c('A') << 2))
    });

    let c_instruction = dest
        .or(empty().to(Dest(0)))
        .then(comp())
        .then(jump().or(empty().to(Jump(0))))
        .map_with(|((dest, comp), jump), extra| Instruction {
            meta: MetaInstruction::CInstruction(dest, comp, jump),
            span: extra.span(),
        });

    a_instruction
        .or(c_instruction)
        .or(label)
        .padded()
        .repeated()
        .collect::<Vec<_>>()
}

pub fn comp<'a>() -> impl Parser<'a, &'a str, Comp, extra::Err<Rich<'a, char>>> {
    just("0")
        .map(|_| 0b0_101010)
        .or(just("1").map(|_| 0b0_111111))
        .or(just("-1").map(|_| 0b0_111010))
        .or(just("D+1").map(|_| 0b0_011111))
        .or(just("A+1").map(|_| 0b0_110111))
        .or(just("M+1").map(|_| 0b1_110111))
        .or(just("D-1").map(|_| 0b0_001111))
        .or(just("A-1").map(|_| 0b0_110011))
        .or(just("M-1").map(|_| 0b1_110011))
        .or(just("D+A").map(|_| 0b0_000010))
        .or(just("D+M").map(|_| 0b1_000010))
        .or(just("D-A").map(|_| 0b0_010011))
        .or(just("D-M").map(|_| 0b1_010011))
        .or(just("A-D").map(|_| 0b0_000111))
        .or(just("M-D").map(|_| 0b1_000111))
        .or(just("D&A").map(|_| 0b0_000000))
        .or(just("D&M").map(|_| 0b1_000000))
        .or(just("D|A").map(|_| 0b0_010101))
        .or(just("D|M").map(|_| 0b1_010101))
        .or(just("!D").map(|_| 0b0_001101))
        .or(just("!A").map(|_| 0b0_110001))
        .or(just("!M").map(|_| 0b1_110001))
        .or(just("-D").map(|_| 0b0_001111))
        .or(just("-A").map(|_| 0b0_110011))
        .or(just("-M").map(|_| 0b1_110011))
        .or(just("D").map(|_| 0b0_001100))
        .or(just("A").map(|_| 0b0_110000))
        .or(just("M").map(|_| 0b1_110000))
        .map(Comp)
}

fn jump<'a>() -> impl Parser<'a, &'a str, Jump, extra::Err<Rich<'a, char>>> {
    just(";").ignore_then(
        just("JGT")
            .map(|_| 1)
            .or(just("JEQ").map(|_| 2))
            .or(just("JGE").map(|_| 3))
            .or(just("JLT").map(|_| 4))
            .or(just("JNE").map(|_| 5))
            .or(just("JLE").map(|_| 6))
            .or(just("JMP").map(|_| 7))
            .map(Jump),
    )
}
