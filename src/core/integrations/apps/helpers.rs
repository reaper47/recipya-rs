use std::borrow::Cow;
use std::collections::HashMap;
use std::env::temp_dir;
use std::fs::File;
use std::io;
use std::io::{Read, Seek};
use std::path::Path;

use tracing::log::warn;
use uuid::Uuid;
use zip::ZipArchive;

use crate::core::integrations::Result;
use crate::core::integrations::apps::cookmate;
use crate::core::integrations::apps::mastercook::parse_mx2;
use crate::core::model::recipe::Sections;
use crate::core::scraper::schema::{ImageObjectOrUrl, RecipeSchema};

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

pub(super) fn extract_archive_contents<R>(
    mut archive: ZipArchive<R>,
) -> Result<(Vec<RecipeSchema>, HashMap<String, std::path::PathBuf>)>
where
    R: Read + Seek,
{
    let mut recipes = Vec::new();
    let mut images = HashMap::new();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_name = file.name().to_string();
        let ext = Path::new(&file_name)
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();

        match ext.to_lowercase().as_str() {
            "mx2" => {
                let r = parse_mx2(file)?;
                recipes.extend(r);
            }
            "jpg" => {
                let tmp_path = temp_dir().join(format!("{}.jpg", Uuid::new_v4()));
                let mut tmp_file = File::create(tmp_path.clone())?;
                io::copy(&mut file, &mut tmp_file)?;

                if let Some(name) = Path::new(&file_name).file_name().and_then(|s| s.to_str()) {
                    images.insert(name.to_string(), tmp_path);
                } else {
                    warn!("Could not get file name from: {file_name}");
                }
            }
            "xml" => {
                let r = cookmate::parse(file)?;
                recipes.extend(r);
            }
            _ => {
                warn!("Unzip .mcb archive, skipping file: {file_name}");
            }
        }
    }

    Ok((recipes, images))
}

pub(super) fn update_recipe_image_paths(
    recipes: &mut [RecipeSchema],
    images: &HashMap<String, std::path::PathBuf>,
) {
    for recipe in recipes {
        let Some(recipe_images) = &mut recipe.image else {
            continue;
        };

        for image in recipe_images.iter_mut() {
            if let ImageObjectOrUrl::ImageObject(obj) = image {
                if let Some(id) = &obj.at_id {
                    let name = Path::new(id)
                        .file_name()
                        .and_then(|s| s.to_str())
                        .unwrap_or_default();

                    if let Some(path) = images.get(name) {
                        obj.at_id = Some(path.to_string_lossy().into_owned());
                    }
                }
            }
        }
    }
}
