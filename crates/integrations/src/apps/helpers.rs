use std::borrow::Cow;
use std::collections::HashMap;
use std::env::temp_dir;
use std::fs::File;
use std::io;
use std::io::{Read, Seek};
use std::path::{Path, PathBuf};

use recipe_schema::{ImageObjectOrUrl, ImageObjectType, RecipeSchema, Sections};
use support::strings::auto_convert_to_utf8;
use tracing::log::warn;
use url::Url;
use uuid::Uuid;
use zip::ZipArchive;

use crate::Result;
use crate::apps::{cookmate, mastercook::parse_mx2};

#[derive(Debug)]
pub(super) enum Instruction<'a> {
    Line(Cow<'a, str>),
    Section(Cow<'a, str>),
}

#[derive(Clone, Debug)]
pub enum Ingredient<'a> {
    Line(Cow<'a, str>),
    Section(Cow<'a, str>),
}

/// Reads the entire contents of a reader and converts it to a UTF-8 string.
pub(super) fn read_file<R>(mut r: R) -> Result<String>
where
    R: Read + Seek,
{
    let mut buffer = Vec::new();
    r.read_to_end(&mut buffer)?;
    Ok(auto_convert_to_utf8(&buffer).replace("\r\n", "\n"))
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
                        let name_trimmed = name.split_whitespace().collect::<Vec<_>>().join(" ");
                        let name_trimmed = name_trimmed.trim_end_matches('-').trim().to_string();

                        if name.starts_with("           ") && name.ends_with("--") {
                            acc.push((name_trimmed, Vec::new()));
                            return acc;
                        }

                        if let Some((_, lines)) = acc.last_mut() {
                            lines.push(name_trimmed);
                        } else {
                            acc.push(("".into(), vec![name_trimmed]));
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
                let lines = lines
                    .into_iter()
                    .filter(|l| !l.is_empty())
                    .collect::<Vec<_>>();

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

                        let line = match line.trim().find('.') {
                            Some(i) if i < 3 => line[i + 1..].trim().to_string(),
                            _ => line,
                        };

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
                if let Some(l) = lines.last()
                    && l.is_empty()
                {
                    lines.pop();
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
) -> Result<(Vec<RecipeSchema>, HashMap<String, PathBuf>)>
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
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)?;

                let cursor = io::Cursor::new(buffer);
                let r = parse_mx2(cursor)?;
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
    images: &HashMap<String, PathBuf>,
) {
    for recipe in recipes {
        let Some(recipe_images) = recipe.image.as_mut() else {
            continue;
        };

        match recipe_images {
            ImageObjectOrUrl::ImageObject(obj) => rewrite_image_id(obj, images),
            ImageObjectOrUrl::ImageObjects(objects) => {
                objects
                    .iter_mut()
                    .for_each(|obj| rewrite_image_id(obj, images));
            }
            _ => {}
        }
    }
}

fn rewrite_image_id(obj: &mut ImageObjectType, images: &HashMap<String, PathBuf>) {
    let Some(id) = obj.at_id.as_deref() else {
        return;
    };

    let Some(name) = Path::new(id).file_name().and_then(|s| s.to_str()) else {
        return;
    };

    let Some(path) = images.get(name) else {
        return;
    };

    obj.at_id = Some(path.to_string_lossy().into_owned());
}

pub(super) fn urls_to_image_object(urls: Vec<String>) -> Option<ImageObjectOrUrl> {
    let urls: Vec<Url> = urls
        .into_iter()
        .filter_map(|image| Url::parse(&image).ok())
        .collect();

    (!urls.is_empty()).then_some(ImageObjectOrUrl::Urls(urls))
}
