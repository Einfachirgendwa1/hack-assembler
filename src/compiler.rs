use crate::MetaInstruction;
use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::span::Spanned;
use std::collections::HashMap;
use std::mem::take;
use std::process::exit;

pub fn compile(mut instructions: Vec<Spanned<MetaInstruction>>, source: &str) -> Vec<i16> {
    let mut map = HashMap::new();

    for i in 0i16..16 {
        map.insert(format!("R{i}"), (i, None));
    }

    let mut i = 0;

    for instruction in &mut instructions {
        println!("{:?}", instruction);
        let span = instruction.span;
        if let MetaInstruction::Label(str) = &mut instruction.inner {
            let s = take(str);

            if map.contains_key(&s) {
                let span = ("input", span.into_range());

                let mut report = Report::build(ReportKind::Error, span.clone())
                    .with_message(format!("'{s}' declared multiple times!"));

                if let Some(first) = map[&s].1.clone() {
                    report = report.with_label(
                        Label::new(("input", first))
                            .with_message("First declared here")
                            .with_color(Color::Cyan),
                    );
                }

                report
                    .with_label(
                        Label::new(span)
                            .with_message("Redeclared here")
                            .with_color(Color::Red),
                    )
                    .finish()
                    .eprint(("input", Source::from(source)))
                    .ok();

                exit(1)
            }

            map.insert(s, (i, Some(span.into_range())));
        } else {
            i += 1;
        }
    }

    let mut res = vec![];
    let mut register_counter = 16;

    for instruction in instructions {
        match instruction.inner {
            MetaInstruction::AInstruction(addr) => res.push(!(1 << 15) & addr),
            MetaInstruction::UnresolvedAInstruction(label) => {
                if !map.contains_key(&label) {
                    map.insert(label.clone(), (register_counter, None));
                    register_counter += 1;
                }
                res.push(!(1 << 15) & map[&label].0);
            }
            MetaInstruction::CInstruction(dest, comp, jump) => res.push(
                (0b111 << 13) | ((comp.0 as i16) << 6) | ((dest.0 as i16) << 3) | (jump.0 as i16),
            ),

            _ => {}
        }
    }

    res
}
