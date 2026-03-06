use std::path::Path;

use tracing::warn;

/// Represents the supported file formats for some applications.
#[derive(Clone, Debug, Default, strum_macros::Display, Eq, PartialEq)]
pub enum FileFormat {
    Json,
    MCB,
    MX2,
    MXP,
    MZ2,
    MealMaster,
    Rezkonv,
    Txt,
    Xml,
    #[default]
    Unknown,
}

impl FileFormat {
    /// Determines the file format based on the file extension.
    ///
    /// This function analyzes the given filename and returns the corresponding
    /// `FileFormat` variant based on the file extension. The comparison is
    /// case-insensitive.
    pub fn from_filename(filename: &str) -> Self {
        let filename_lower = filename.to_lowercase();
        let ext = Path::new(&filename_lower)
            .extension()
            .and_then(|s| s.to_str())
            .or_else(|| {
                filename_lower
                    .strip_prefix('.')
                    .filter(|s| !s.is_empty() && !s.contains('.'))
            })
            .unwrap_or("");

        match ext {
            "json" => Self::Json,
            "mcb" => Self::MCB,
            "mx2" => Self::MX2,
            "mxp" => Self::MXP,
            "mz2" => Self::MZ2,
            "mm" | "mmf" => Self::MealMaster,
            "rzk" | "rk" => Self::Rezkonv,
            "txt" => Self::Txt,
            "xml" => Self::Xml,
            _ => {
                warn!("File '{filename}' has an unknown file format: {ext}");
                Self::Unknown
            }
        }
    }

    /// Gets the list of supported file extensions.
    pub fn extensions<'a>() -> Vec<&'a str> {
        vec![
            ".cook",
            ".crumb",
            ".json",
            ".mcb",
            ".md",
            ".mmf",
            ".mx2",
            ".mxp",
            ".mz2",
            ".mm",
            ".mmf",
            ".paprikarecipes",
            ".rzk",
            ".rk",
            ".rzk",
            ".txt",
            ".xml",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod from_filename {
        use super::*;

        #[test]
        fn test_json_format() {
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("recipe.json"),
                FileFormat::Json
            );
            pretty_assertions::assert_eq!(FileFormat::from_filename("data.JSON"), FileFormat::Json);
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("config.Json"),
                FileFormat::Json
            );
        }

        #[test]
        fn test_mcb_format() {
            pretty_assertions::assert_eq!(FileFormat::from_filename("recipe.mcb"), FileFormat::MCB);
            pretty_assertions::assert_eq!(FileFormat::from_filename("data.MCB"), FileFormat::MCB);
            pretty_assertions::assert_eq!(FileFormat::from_filename("file.Mcb"), FileFormat::MCB);
        }

        #[test]
        fn test_mx2_format() {
            pretty_assertions::assert_eq!(FileFormat::from_filename("recipe.mx2"), FileFormat::MX2);
            pretty_assertions::assert_eq!(FileFormat::from_filename("data.MX2"), FileFormat::MX2);
            pretty_assertions::assert_eq!(FileFormat::from_filename("file.Mx2"), FileFormat::MX2);
        }

        #[test]
        fn test_mxp_format() {
            pretty_assertions::assert_eq!(FileFormat::from_filename("recipe.mxp"), FileFormat::MXP);
            pretty_assertions::assert_eq!(FileFormat::from_filename("data.MXP"), FileFormat::MXP);
            pretty_assertions::assert_eq!(FileFormat::from_filename("file.Mxp"), FileFormat::MXP);
        }

        #[test]
        fn test_mz2_format() {
            pretty_assertions::assert_eq!(FileFormat::from_filename("recipe.mz2"), FileFormat::MZ2);
            pretty_assertions::assert_eq!(FileFormat::from_filename("data.MZ2"), FileFormat::MZ2);
            pretty_assertions::assert_eq!(FileFormat::from_filename("file.Mz2"), FileFormat::MZ2);
        }

        #[test]
        fn test_meal_master_format() {
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("recipe.mm"),
                FileFormat::MealMaster
            );
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("data.MM"),
                FileFormat::MealMaster
            );
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("file.mmf"),
                FileFormat::MealMaster
            );
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("recipe.MMF"),
                FileFormat::MealMaster
            );
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("data.Mm"),
                FileFormat::MealMaster
            );
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("file.Mmf"),
                FileFormat::MealMaster
            );
        }

        #[test]
        fn test_rezkonv_format() {
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("recipe.rzk"),
                FileFormat::Rezkonv
            );
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("data.RZK"),
                FileFormat::Rezkonv
            );
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("file.rk"),
                FileFormat::Rezkonv
            );
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("recipe.RK"),
                FileFormat::Rezkonv
            );
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("data.Rzk"),
                FileFormat::Rezkonv
            );
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("file.Rk"),
                FileFormat::Rezkonv
            );
        }

        #[test]
        fn test_txt_format() {
            pretty_assertions::assert_eq!(FileFormat::from_filename("recipe.txt"), FileFormat::Txt);
            pretty_assertions::assert_eq!(FileFormat::from_filename("data.TXT"), FileFormat::Txt);
            pretty_assertions::assert_eq!(FileFormat::from_filename("file.Txt"), FileFormat::Txt);
        }

        #[test]
        fn test_xml_format() {
            pretty_assertions::assert_eq!(FileFormat::from_filename("recipe.xml"), FileFormat::Xml);
            pretty_assertions::assert_eq!(FileFormat::from_filename("data.XML"), FileFormat::Xml);
            pretty_assertions::assert_eq!(FileFormat::from_filename("file.Xml"), FileFormat::Xml);
        }

        #[test]
        fn test_unknown_formats() {
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("recipe.pdf"),
                FileFormat::Unknown
            );
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("data.doc"),
                FileFormat::Unknown
            );
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("file.xyz"),
                FileFormat::Unknown
            );
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("unknown.abc"),
                FileFormat::Unknown
            );
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("test.csv"),
                FileFormat::Unknown
            );
        }

        #[test]
        fn test_edge_cases() {
            // No extension
            pretty_assertions::assert_eq!(FileFormat::from_filename("recipe"), FileFormat::Unknown);
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("no_extension"),
                FileFormat::Unknown
            );

            // Empty filename
            pretty_assertions::assert_eq!(FileFormat::from_filename(""), FileFormat::Unknown);

            // Only extension
            pretty_assertions::assert_eq!(FileFormat::from_filename(".json"), FileFormat::Json);
            pretty_assertions::assert_eq!(FileFormat::from_filename(".xml"), FileFormat::Xml);
            pretty_assertions::assert_eq!(
                FileFormat::from_filename(".unknown"),
                FileFormat::Unknown
            );

            // Multiple dots
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("recipe.backup.json"),
                FileFormat::Json
            );
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("data.old.xml"),
                FileFormat::Xml
            );
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("file.v1.2.mcb"),
                FileFormat::MCB
            );

            // Dot at the end
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("recipe."),
                FileFormat::Unknown
            );
        }

        #[test]
        fn test_paths_with_directories() {
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("/path/to/recipe.json"),
                FileFormat::Json
            );
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("../recipes/data.xml"),
                FileFormat::Xml
            );
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("./local/file.mcb"),
                FileFormat::MCB
            );
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("C:\\Users\\recipes\\meal.mm"),
                FileFormat::MealMaster
            );
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("~/documents/recipe.mxp"),
                FileFormat::MXP
            );
        }

        #[test]
        fn test_case_insensitive_matching() {
            // Test various case combinations
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("recipe.JSON"),
                FileFormat::Json
            );
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("recipe.Json"),
                FileFormat::Json
            );
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("recipe.jSoN"),
                FileFormat::Json
            );

            pretty_assertions::assert_eq!(FileFormat::from_filename("data.XML"), FileFormat::Xml);
            pretty_assertions::assert_eq!(FileFormat::from_filename("data.Xml"), FileFormat::Xml);
            pretty_assertions::assert_eq!(FileFormat::from_filename("data.xMl"), FileFormat::Xml);

            pretty_assertions::assert_eq!(FileFormat::from_filename("file.TXT"), FileFormat::Txt);
            pretty_assertions::assert_eq!(FileFormat::from_filename("file.Txt"), FileFormat::Txt);
            pretty_assertions::assert_eq!(FileFormat::from_filename("file.tXt"), FileFormat::Txt);
        }

        #[test]
        fn test_special_characters() {
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("recipe with spaces.json"),
                FileFormat::Json
            );
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("recipe-with-dashes.xml"),
                FileFormat::Xml
            );
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("recipe_with_underscores.txt"),
                FileFormat::Txt
            );
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("recipe@symbol.mcb"),
                FileFormat::MCB
            );
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("recipe#hash.mx2"),
                FileFormat::MX2
            );
        }

        #[test]
        fn test_unicode_filenames() {
            pretty_assertions::assert_eq!(
                FileFormat::from_filename("recette.json"),
                FileFormat::Json
            );
            pretty_assertions::assert_eq!(FileFormat::from_filename("レシピ.xml"), FileFormat::Xml);
            pretty_assertions::assert_eq!(FileFormat::from_filename("рецепт.txt"), FileFormat::Txt);
            pretty_assertions::assert_eq!(FileFormat::from_filename("食谱.mcb"), FileFormat::MCB);
        }
    }
}
