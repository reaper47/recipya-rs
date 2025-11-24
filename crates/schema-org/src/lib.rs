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

/// Enumeration of all possible @type values.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub enum AtType {
    AggregateRating,
    Comment,
    Energy,
    HowToTool,
    ImageObject,
    ItemList,
    ListItem,
    MusicAlbum,
    NutritionInformation,
    QuantitativeValue,
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
                AtType::Comment => "Comment".to_string(),
                AtType::Energy => "Energy".to_string(),
                AtType::Recipe => "Recipe".to_string(),
                AtType::AggregateRating => "AggregateRating".to_string(),
                AtType::HowToTool => "HowToTool".to_string(),
                AtType::ImageObject => "ImageObject".to_string(),
                AtType::ItemList => "ItemList".to_string(),
                AtType::ListItem => "ListItem".to_string(),
                AtType::MusicAlbum => "MusicAlbum".to_string(),
                AtType::NutritionInformation => "NutritionInformation".to_string(),
                AtType::QuantitativeValue => "QuantitativeValue".to_string(),
                AtType::Review => "Review".to_string(),
                AtType::VideoObject => "VideoObject".to_string(),
            }
        )
    }
}
