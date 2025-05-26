use std::borrow::Cow;

use crate::core::model::recipe::Sections;

#[derive(Debug)]
pub(super) enum Instruction<'a> {
    Line(Cow<'a, str>),
    Section(Cow<'a, str>),
}

#[derive(Clone, Debug)]
pub(super) enum Ingredient<'a> {
    Line(Cow<'a, str>),
    Section(Cow<'a, str>),
}

pub(super) trait ToSections<'a> {
    fn to_sections(&self) -> Sections;
}

impl ToSections<'_> for Vec<Ingredient<'_>> {
    fn to_sections(&self) -> Sections {
        self.iter()
            .fold(Sections::new(), |mut acc, ing| {
                match ing {
                    Ingredient::Line(name) => {
                        let name = name.split_whitespace().collect::<Vec<_>>().join(" ");
                        let name = name.trim().to_string();

                        if let Some((_, lines)) = acc.last_mut() {
                            lines.push(name);
                        } else {
                            acc.push(("".into(), vec![name]));
                        }
                    }
                    Ingredient::Section(section) => {
                        acc.push((section.to_string(), Vec::new()));
                    }
                }
                acc
            })
            .into_iter()
            .map(|(section, lines)| {
                let merged = (0..lines.len())
                    .filter_map(|i| {
                        let line = &lines[i];

                        if line.ends_with(';') && i + 2 < lines.len() {
                            Some((i, format!("{} {}", line, lines[i + 2])))
                        } else if i > 1 && lines[i - 2].ends_with(';') {
                            None
                        } else {
                            Some((i, line.clone()))
                        }
                    })
                    .map(|(_, line)| line)
                    .collect();

                (section, merged)
            })
            .collect()
    }
}

impl ToSections<'_> for Vec<Instruction<'_>> {
    fn to_sections(&self) -> Sections {
        self.iter()
            .fold(Sections::new(), |mut acc, ins| {
                match ins {
                    Instruction::Section(section) => {
                        acc.push((section.to_string(), Vec::new()));
                    }
                    Instruction::Line(line) => {
                        let line = line.split_whitespace().collect::<Vec<_>>().join(" ");
                        let line = line.trim().to_string();

                        if !line.is_empty() {
                            if acc.is_empty() {
                                acc.push(("".into(), Vec::new()));
                            }

                            if let Some((_, lines)) = acc.last_mut() {
                                lines.push(line);
                            }
                        } else if let Some((_, lines)) = acc.last_mut() {
                            lines.push(line);
                        }
                    }
                }
                acc
            })
            .into_iter()
            .map(|(section, mut lines)| {
                if let Some(l) = lines.last() {
                    if l.is_empty() {
                        lines.pop();
                    }
                }
                (section, lines)
            })
            .collect()
    }
}

pub(super) fn is_vchar_or_space(c: char) -> bool {
    !c.is_control() && (c != '\n' && c != '\r')
}
