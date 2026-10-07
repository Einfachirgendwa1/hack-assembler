use crate::{Instruction, MetaInstruction};
use ariadne::{Color, Label, Report, ReportKind, Source};
use std::collections::HashMap;
use std::mem::take;

pub fn compile(mut instructions: Vec<Instruction>, source: &str) -> Vec<i16> {
    let mut map = HashMap::new();

    for i in 0i16..16 {
        map.insert(format!("R{i}"), i);
    }

    let mut i = 0;

    for Instruction { meta, span } in &mut instructions {
        if let MetaInstruction::Label(str) = meta {
            let s = take(str);

            if map.contains_key(&s) {
                let span = ("input", span.into_range());

                Report::build(ReportKind::Warning, span.clone())
                    .with_message(format!("'{s}' declared multiple times!"))
                    .with_label(
                        Label::new(span)
                            .with_message("Label exists already")
                            .with_color(Color::Yellow),
                    )
                    .finish()
                    .eprint(("input", Source::from(source)))
                    .ok();
            }

            map.insert(s, i);
        } else {
            i += 1;
        }
    }

    let mut res = vec![];
    let mut register_counter = 16;

    for Instruction { meta, .. } in instructions {
        match meta {
            MetaInstruction::AInstruction(addr) => res.push(!(1 << 15) & addr),
            MetaInstruction::UnresolvedAInstruction(label) => {
                if !map.contains_key(&label) {
                    map.insert(label.clone(), register_counter);
                    register_counter += 1;
                }
                res.push(!(1 << 15) & map[&label]);
            }
            MetaInstruction::CInstruction(dest, comp, jump) => res.push(
                (0b111 << 13) | ((comp.0 as i16) << 6) | ((dest.0 as i16) << 3) | (jump.0 as i16),
            ),

            _ => {}
        }
    }

    res
}
