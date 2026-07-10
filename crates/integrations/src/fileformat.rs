use std::path::Path;

use tracing::warn;

/// Represents the supported file formats for some applications.
#[derive(Clone, Debug, Default, strum_macros::Display, Eq, PartialEq)]
pub enum FileFormat {
    Csv,
    Hc,
    Html,
    Jpg,
    Json,
    MCB,
    MX2,
    MXP,
    MZ2,
    MealMaster,
    Png,
    Rezkonv,
    Rtk,
    Txt,
    Xml,
    Yaml,
    Zip,
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
            .unwrap_or_default();

        match ext {
            "csv" => Self::Csv,
            "hc" => Self::Hc,
            "html" => Self::Html,
            "jpg" => Self::Jpg,
            "json" => Self::Json,
            "mcb" => Self::MCB,
            "mx2" => Self::MX2,
            "mxp" => Self::MXP,
            "mz2" => Self::MZ2,
            "mm" | "mmf" => Self::MealMaster,
            "png" => Self::Png,
            "rtk" => Self::Rtk,
            "rzk" | "rk" => Self::Rezkonv,
            "txt" => Self::Txt,
            "xml" => Self::Xml,
            "yml" | "yaml" => Self::Yaml,
            "zip" => Self::Zip,
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
            ".csv",
            ".hc",
            ".html",
            // Omit: jpg,png - apps usually don't export recipes as jpg
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
            ".rk",
            ".rtk",
            ".rzk",
            ".rzk",
            ".txt",
            ".xml",
            ".yml",
            ".yaml",
            ".zip",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod from_filename {
        use super::*;

        macro_rules! assert_format {
            ($($filename:expr => $expected:expr),+ $(,)?) => {
                $(
                    pretty_assertions::assert_eq!(
                        FileFormat::from_filename($filename),
                        $expected,
                        "unexpected format for filename {:?}",
                        $filename
                    );
                )+
            };
        }

        #[test]
        fn test_csv_format() {
            assert_format!(
                "recipe.csv" => FileFormat::Csv,
                "data.CSV" => FileFormat::Csv,
                "config.Csv" => FileFormat::Csv,
            );
        }

        #[test]
        fn test_hc_format() {
            assert_format!(
                "recipe.hc" => FileFormat::Hc,
                "data.HC" => FileFormat::Hc,
            );
        }

        #[test]
        fn test_html_format() {
            assert_format!(
                "recipe.html" => FileFormat::Html,
                "data.HTML" => FileFormat::Html,
                "config.Html" => FileFormat::Html,
            );
        }

        #[test]
        fn test_jpg_format() {
            assert_format!(
                "test.jpg" => FileFormat::Jpg,
                "data.JPG" => FileFormat::Jpg,
            );
        }

        #[test]
        fn test_json_format() {
            assert_format!(
                "recipe.json" => FileFormat::Json,
                "data.JSON" => FileFormat::Json,
                "config.Json" => FileFormat::Json,
            );
        }

        #[test]
        fn test_mcb_format() {
            assert_format!(
                "recipe.mcb" => FileFormat::MCB,
                "data.MCB" => FileFormat::MCB,
                "file.Mcb" => FileFormat::MCB,
            );
        }

        #[test]
        fn test_mx2_format() {
            assert_format!(
                "recipe.mx2" => FileFormat::MX2,
                "data.MX2" => FileFormat::MX2,
                "file.Mx2" => FileFormat::MX2,
            );
        }

        #[test]
        fn test_mxp_format() {
            assert_format!(
                "recipe.mxp" => FileFormat::MXP,
                "data.MXP" => FileFormat::MXP,
                "file.Mxp" => FileFormat::MXP,
            );
        }

        #[test]
        fn test_mz2_format() {
            assert_format!(
                "recipe.mz2" => FileFormat::MZ2,
                "data.MZ2" => FileFormat::MZ2,
                "file.Mz2" => FileFormat::MZ2,
            );
        }

        #[test]
        fn test_meal_master_format() {
            assert_format!(
                "recipe.mm" => FileFormat::MealMaster,
                "data.MM" => FileFormat::MealMaster,
                "file.mmf" => FileFormat::MealMaster,
                "recipe.MMF" => FileFormat::MealMaster,
                "data.Mm" => FileFormat::MealMaster,
                "file.Mmf" => FileFormat::MealMaster,
            );
        }

        #[test]
        fn test_png_format() {
            assert_format!(
                "test.png" => FileFormat::Png,
                "data.PnG" => FileFormat::Png,
            );
        }

        #[test]
        fn test_rezkonv_format() {
            assert_format!(
                "recipe.rzk" => FileFormat::Rezkonv,
                "data.RZK" => FileFormat::Rezkonv,
                "file.rk" => FileFormat::Rezkonv,
                "recipe.RK" => FileFormat::Rezkonv,
                "data.Rzk" => FileFormat::Rezkonv,
                "file.Rk" => FileFormat::Rezkonv,
            );
        }

        #[test]
        fn test_rtk_format() {
            assert_format!(
                "recipe.rtk" => FileFormat::Rtk,
                "data.RTk" => FileFormat::Rtk,
            );
        }

        #[test]
        fn test_txt_format() {
            assert_format!(
                "recipe.txt" => FileFormat::Txt,
                "data.TXT" => FileFormat::Txt,
                "file.Txt" => FileFormat::Txt,
            );
        }

        #[test]
        fn test_xml_format() {
            assert_format!(
                "recipe.xml" => FileFormat::Xml,
                "data.XML" => FileFormat::Xml,
                "file.Xml" => FileFormat::Xml,
            );
        }

        #[test]
        fn test_yaml_format() {
            assert_format!(
                "recipe.yml" => FileFormat::Yaml,
                "recipe.YAML" => FileFormat::Yaml,
            );
        }

        #[test]
        fn test_unknown_formats() {
            assert_format!(
                "recipe.pdf" => FileFormat::Unknown,
                "data.doc" => FileFormat::Unknown,
                "file.xyz" => FileFormat::Unknown,
                "unknown.abc" => FileFormat::Unknown,
            );
        }

        #[test]
        fn test_edge_cases() {
            assert_format!(
                // No extension
                "recipe" => FileFormat::Unknown,
                "no_extension" => FileFormat::Unknown,
                // Empty filename
                "" => FileFormat::Unknown,
                // Only extension
                ".json" => FileFormat::Json,
                ".xml" => FileFormat::Xml,
                ".unknown" => FileFormat::Unknown,
                // Multiple dots
                "recipe.backup.json" => FileFormat::Json,
                "data.old.xml" => FileFormat::Xml,
                "file.v1.2.mcb" => FileFormat::MCB,
                // Dot at the end
                "recipe." => FileFormat::Unknown,
            );
        }

        #[test]
        fn test_paths_with_directories() {
            assert_format!(
                "/path/to/recipe.json" => FileFormat::Json,
                "../recipes/data.xml" => FileFormat::Xml,
                "./local/file.mcb" => FileFormat::MCB,
                "C:\\Users\\recipes\\meal.mm" => FileFormat::MealMaster,
                "~/documents/recipe.mxp" => FileFormat::MXP,
            );
        }

        #[test]
        fn test_case_insensitive_matching() {
            assert_format!(
                "recipe.JSON" => FileFormat::Json,
                "recipe.Json" => FileFormat::Json,
                "recipe.jSoN" => FileFormat::Json,
                "data.XML" => FileFormat::Xml,
                "data.Xml" => FileFormat::Xml,
                "data.xMl" => FileFormat::Xml,
                "file.TXT" => FileFormat::Txt,
                "file.Txt" => FileFormat::Txt,
                "file.tXt" => FileFormat::Txt,
            );
        }

        #[test]
        fn test_special_characters() {
            assert_format!(
                "recipe with spaces.json" => FileFormat::Json,
                "recipe-with-dashes.xml" => FileFormat::Xml,
                "recipe_with_underscores.txt" => FileFormat::Txt,
                "recipe@symbol.mcb" => FileFormat::MCB,
                "recipe#hash.mx2" => FileFormat::MX2,
            );
        }

        #[test]
        fn test_unicode_filenames() {
            assert_format!(
                "recette.json" => FileFormat::Json,
                "レシピ.xml" => FileFormat::Xml,
                "рецепт.txt" => FileFormat::Txt,
                "食谱.mcb" => FileFormat::MCB,
            );
        }

        #[test]
        fn test_zip_filenames() {
            assert_format!(
                "recipya.zip" => FileFormat::Zip,
            );
        }
    }
}
