use std::borrow::Cow;
use std::collections::HashMap;
use std::env::temp_dir;
use std::fs::File;
use std::io;
use std::io::{Read, Seek};
use std::path::{Path, PathBuf};

use tracing::log::warn;
use url::Url;
use uuid::Uuid;
use zip::ZipArchive;

use schema_org::field::{
    ImageObjectImageFieldEnum, ItemListItemListElementFieldEnum, RecipeImageFieldEnum,
    RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum,
};
use schema_org::{ImageObject, Recipe};
use support::strings::auto_convert_to_utf8;

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
    type Section;
    fn to_sections(&self) -> Vec<Self::Section>;
}

impl ToSections<'_> for Vec<Ingredient<'_>> {
    type Section = RecipeRecipeIngredientFieldEnum;

    fn to_sections(&self) -> Vec<RecipeRecipeIngredientFieldEnum> {
        self.iter()
            .filter(|ing| match ing {
                Ingredient::Line(name) | Ingredient::Section(name) => !name.is_empty(),
            })
            .fold(Vec::new(), |mut acc, ing| {
                match ing {
                    Ingredient::Line(name) => {
                        let name_trimmed = name.split_whitespace().collect::<Vec<_>>().join(" ");
                        let name_trimmed = name_trimmed.trim_end_matches('-').trim().to_string();

                        if name.starts_with("           ") && name.ends_with("--") {
                            acc.push(RecipeRecipeIngredientFieldEnum::new_section(
                                &name_trimmed,
                                &[],
                            ));
                            return acc;
                        }

                        if let Some(schema_org::field::FieldEnum149::ItemList(list)) =
                            acc.last_mut()
                        {
                            list.item_list_element
                                .push(ItemListItemListElementFieldEnum::Text(name_trimmed));
                            list.number_of_items.get_mut(0).map(|i| *i + 1);
                        } else {
                            acc.push(RecipeRecipeIngredientFieldEnum::Text(name_trimmed));
                        }
                    }
                    Ingredient::Section(section) => {
                        acc.push(RecipeRecipeIngredientFieldEnum::new_section(section, &[]));
                    }
                }
                acc
            })
            .into_iter()
            .map(|element| match element {
                RecipeRecipeIngredientFieldEnum::ItemList(list) => {
                    let lines = list
                        .item_list_element
                        .into_iter()
                        .filter_map(|l| match l {
                            ItemListItemListElementFieldEnum::Text(s) => {
                                (!s.is_empty()).then_some(s)
                            }
                            ItemListItemListElementFieldEnum::ListItem(_)
                            | ItemListItemListElementFieldEnum::Thing(_) => None,
                        })
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
                        .map(|(_, s)| s)
                        .collect::<Vec<_>>();

                    RecipeRecipeIngredientFieldEnum::new_section(
                        &list.name[0],
                        merged
                            .iter()
                            .map(String::as_str)
                            .collect::<Vec<_>>()
                            .as_slice(),
                    )
                }
                RecipeRecipeIngredientFieldEnum::PropertyValue(v) => {
                    RecipeRecipeIngredientFieldEnum::PropertyValue(v)
                }
                RecipeRecipeIngredientFieldEnum::Text(s) => {
                    RecipeRecipeIngredientFieldEnum::Text(s)
                }
            })
            .collect()
    }
}

impl ToSections<'_> for Vec<Instruction<'_>> {
    type Section = RecipeRecipeInstructionsFieldEnum;

    fn to_sections(&self) -> Vec<RecipeRecipeInstructionsFieldEnum> {
        self.iter()
            .fold(Vec::new(), |mut acc, ins| {
                match ins {
                    Instruction::Section(section) => {
                        acc.push(RecipeRecipeInstructionsFieldEnum::new_section::<String>(
                            section,
                            vec![],
                        ));
                    }
                    Instruction::Line(line) => {
                        let line = line.split_whitespace().collect::<Vec<_>>().join(" ");

                        let line = match line.trim().find('.') {
                            Some(i) if i < 3 => line[i + 1..].trim().to_string(),
                            _ => line,
                        };

                        if !line.is_empty() {
                            if let Some(schema_org::field::FieldEnum141::ItemList(list)) =
                                acc.last_mut()
                            {
                                list.item_list_element
                                    .push(ItemListItemListElementFieldEnum::Text(line));
                                list.number_of_items.get_mut(0).map(|i| *i + 1);
                            } else {
                                acc.push(RecipeRecipeInstructionsFieldEnum::Text(line));
                            }
                        }
                    }
                }
                acc
            })
            .into_iter()
            .collect()
    }
}

pub(super) fn is_vchar_or_space(c: char) -> bool {
    !c.is_control() && (c != '\n' && c != '\r')
}

pub(super) fn extract_archive_contents<R>(
    mut archive: ZipArchive<R>,
) -> Result<(Vec<Recipe>, HashMap<String, PathBuf>)>
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

pub(super) fn update_recipe_image_paths(recipes: &mut [Recipe], images: &HashMap<String, PathBuf>) {
    for recipe in recipes {
        recipe
            .image
            .iter_mut()
            .for_each(|img| rewrite_image_id(img, images));
    }
}

fn rewrite_image_id(img: &mut RecipeImageFieldEnum, images: &HashMap<String, PathBuf>) {
    match img {
        RecipeImageFieldEnum::ImageObject(obj) => {
            obj.image.iter().for_each(|img| match img {
                ImageObjectImageFieldEnum::ImageObject(_) => {}
                ImageObjectImageFieldEnum::URL(path) => {
                    let Some(name) = Path::new(path).file_name().and_then(|s| s.to_str()) else {
                        return;
                    };

                    let Some(_) = images.get(name) else {
                        return;
                    };
                }
            });
        }
        RecipeImageFieldEnum::URL(path) => {
            let Some(name) = Path::new(path).file_name().and_then(|s| s.to_str()) else {
                return;
            };

            let Some(_) = images.get(name) else {
                return;
            };

            // obj.at_id = Some(path.to_string_lossy().into_owned());
        }
    }
}

pub(super) fn urls_to_image_object(urls: Vec<String>) -> Vec<RecipeImageFieldEnum> {
    urls.into_iter()
        .filter_map(|image| Url::parse(&image).ok())
        .map(|url| {
            RecipeImageFieldEnum::ImageObject(Box::from(ImageObject {
                url: vec![url.to_string()],
                ..Default::default()
            }))
        })
        .collect()
}
