mod action;
mod aggregate_rating;
mod audio_object;
mod breadcrumb_list;
mod clip;
mod comment;
mod country;
mod creative_work;
mod defined_term;
mod defined_term_set;
mod distance;
mod duration;
mod energy;
mod entry_point;
mod enumeration;
mod event;
mod how_to;
mod how_to_section;
mod how_to_step;
mod how_to_supply;
mod how_to_tool;
mod image_object;
mod interaction_counter;
mod item_list;
mod language;
mod list_item;
mod mass;
mod measurement_type_enumeration;
mod media_object;
mod monetary_amount;
mod music_album;
mod music_composition;
mod music_playlist;
mod music_recording;
mod music_release;
mod nutrition_information;
mod organization;
mod person;
mod place;
mod postal_address;
mod product;
mod property_value;
mod qualitative_value;
mod quantitative_value;
mod rating;
mod recipe;
mod review;
mod size_specification;
mod structured_value;
mod text_object;
mod thing;
mod video_object;
mod web_content;
mod web_page;
mod web_page_element;

pub mod enums;
pub mod field;
pub(crate) mod helpers;

pub use action::*;
pub use aggregate_rating::*;
pub use audio_object::*;
pub use breadcrumb_list::*;
pub use clip::*;
pub use comment::*;
pub use country::*;
pub use creative_work::*;
pub use defined_term::*;
pub use defined_term_set::*;
pub use distance::*;
pub use duration::*;
pub use energy::*;
pub use entry_point::*;
pub use enumeration::*;
pub use event::*;
pub use how_to::*;
pub use how_to_section::*;
pub use how_to_step::*;
pub use how_to_supply::*;
pub use how_to_tool::*;
pub use image_object::*;
pub use interaction_counter::*;
pub use item_list::*;
pub use language::*;
pub use list_item::*;
pub use mass::*;
pub use measurement_type_enumeration::*;
pub use media_object::*;
pub use monetary_amount::*;
pub use music_album::*;
pub use music_composition::*;
pub use music_playlist::*;
pub use music_recording::*;
pub use music_release::*;
pub use nutrition_information::*;
pub use organization::*;
pub use person::*;
pub use place::*;
pub use postal_address::*;
pub use product::*;
pub use property_value::*;
pub use qualitative_value::*;
pub use quantitative_value::*;
pub use rating::*;
pub use recipe::*;
pub use review::*;
pub use size_specification::*;
pub use structured_value::*;
pub use text_object::*;
pub use thing::*;
pub use video_object::*;
pub use web_content::*;
pub use web_page::*;
pub use web_page_element::*;

use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Returns the context for schema.org.
pub fn at_context() -> Option<String> {
    Some(String::from("https://schema.org"))
}

/// Enumeration of all possible @type values.
#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub enum AtType {
    AggregateRating,
    Comment,
    CreativeWork,
    Duration,
    Energy,
    HowToStep,
    HowToTool,
    ImageObject,
    ItemList,
    ListItem,
    MusicAlbum,
    NutritionInformation,
    Organization,
    Person,
    QuantitativeValue,
    Rating,
    #[default]
    Recipe,
    Review,
    VideoObject,
}

impl Display for AtType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Comment => "Comment".to_string(),
                Self::CreativeWork => "CreativeWork".to_string(),
                Self::Duration => "Duration".to_string(),
                Self::Energy => "Energy".to_string(),
                Self::Recipe => "Recipe".to_string(),
                Self::AggregateRating => "AggregateRating".to_string(),
                Self::HowToStep => "HowToStep".to_string(),
                Self::HowToTool => "HowToTool".to_string(),
                Self::ImageObject => "ImageObject".to_string(),
                Self::ItemList => "ItemList".to_string(),
                Self::ListItem => "ListItem".to_string(),
                Self::MusicAlbum => "MusicAlbum".to_string(),
                Self::NutritionInformation => "NutritionInformation".to_string(),
                Self::Organization => "Organization".to_string(),
                Self::Person => "Person".to_string(),
                Self::QuantitativeValue => "QuantitativeValue".to_string(),
                Self::Rating => "Rating".to_string(),
                Self::Review => "Review".to_string(),
                Self::VideoObject => "VideoObject".to_string(),
            }
        )
    }
}

impl AtType {
    /// Sets the value of the `@type` attribute.
    pub fn to_opt(&self) -> Option<String> {
        match self {
            Self::AggregateRating => Some(Self::AggregateRating.to_string()),
            Self::Comment => Some(Self::Comment.to_string()),
            Self::CreativeWork => Some(Self::CreativeWork.to_string()),
            Self::Duration => Some(Self::Duration.to_string()),
            Self::Energy => Some(Self::Energy.to_string()),
            Self::HowToStep => Some(Self::HowToStep.to_string()),
            Self::HowToTool => Some(Self::HowToTool.to_string()),
            Self::ImageObject => Some(Self::ImageObject.to_string()),
            Self::ItemList => Some(Self::ItemList.to_string()),
            Self::ListItem => Some(Self::ListItem.to_string()),
            Self::MusicAlbum => Some(Self::MusicAlbum.to_string()),
            Self::NutritionInformation => Some(Self::NutritionInformation.to_string()),
            Self::Organization => Some(Self::Organization.to_string()),
            Self::Person => Some(Self::Person.to_string()),
            Self::QuantitativeValue => Some(Self::QuantitativeValue.to_string()),
            Self::Rating => Some(Self::Rating.to_string()),
            Self::Recipe => Some(Self::Recipe.to_string()),
            Self::Review => Some(Self::Review.to_string()),
            Self::VideoObject => Some(Self::VideoObject.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at_type_display() {
        assert_eq!(AtType::Comment.to_string(), "Comment");
        assert_eq!(AtType::CreativeWork.to_string(), "CreativeWork");
        assert_eq!(AtType::Duration.to_string(), "Duration");
        assert_eq!(AtType::Energy.to_string(), "Energy");
        assert_eq!(AtType::Recipe.to_string(), "Recipe");
        assert_eq!(AtType::AggregateRating.to_string(), "AggregateRating");
        assert_eq!(AtType::HowToStep.to_string(), "HowToStep");
        assert_eq!(AtType::HowToTool.to_string(), "HowToTool");
        assert_eq!(AtType::ImageObject.to_string(), "ImageObject");
        assert_eq!(AtType::ItemList.to_string(), "ItemList");
        assert_eq!(AtType::ListItem.to_string(), "ListItem");
        assert_eq!(AtType::MusicAlbum.to_string(), "MusicAlbum");
        assert_eq!(
            AtType::NutritionInformation.to_string(),
            "NutritionInformation"
        );
        assert_eq!(AtType::Organization.to_string(), "Organization");
        assert_eq!(AtType::QuantitativeValue.to_string(), "QuantitativeValue");
        assert_eq!(AtType::Rating.to_string(), "Rating");
        assert_eq!(AtType::Review.to_string(), "Review");
        assert_eq!(AtType::VideoObject.to_string(), "VideoObject");
    }

    #[test]
    fn test_to_opt() {
        assert_eq!(AtType::Comment.to_opt(), Some("Comment".to_string()));
        assert_eq!(
            AtType::CreativeWork.to_opt(),
            Some("CreativeWork".to_string())
        );
        assert_eq!(AtType::Duration.to_opt(), Some("Duration".to_string()));
        assert_eq!(AtType::Energy.to_opt(), Some("Energy".to_string()));
        assert_eq!(AtType::Recipe.to_opt(), Some("Recipe".to_string()));
        assert_eq!(
            AtType::AggregateRating.to_opt(),
            Some("AggregateRating".to_string())
        );
        assert_eq!(AtType::HowToStep.to_opt(), Some("HowToStep".to_string()));
        assert_eq!(AtType::HowToTool.to_opt(), Some("HowToTool".to_string()));
        assert_eq!(
            AtType::ImageObject.to_opt(),
            Some("ImageObject".to_string())
        );
        assert_eq!(AtType::ItemList.to_opt(), Some("ItemList".to_string()));
        assert_eq!(AtType::ListItem.to_opt(), Some("ListItem".to_string()));
        assert_eq!(AtType::MusicAlbum.to_opt(), Some("MusicAlbum".to_string()));
        assert_eq!(
            AtType::NutritionInformation.to_opt(),
            Some("NutritionInformation".to_string())
        );
        assert_eq!(
            AtType::Organization.to_opt(),
            Some("Organization".to_string())
        );
        assert_eq!(
            AtType::QuantitativeValue.to_opt(),
            Some("QuantitativeValue".to_string())
        );
        assert_eq!(AtType::Review.to_opt(), Some("Review".to_string()));
        assert_eq!(
            AtType::VideoObject.to_opt(),
            Some("VideoObject".to_string())
        );
    }
}
