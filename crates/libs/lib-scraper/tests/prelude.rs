#![cfg_attr(test, allow(unused_imports))]
pub use lib_scraper::{
    schema::{
        common::{
            AggregateRating, ClipOrVideoObject, CreativeWorkOrItemListOrText, CreativeWorkOrUrl,
            CreativeWorkType, DateOrDateTime, DefinedTermOrTextOrUrl, DistanceOrQuantitativeValue,
            DistanceType, HowTo, ImageObjectOrUrl, ImageObjectType, NumberOrText,
            OrganizationOrPerson, OrganizationType, QuantitativeValueOrText, QuantitativeValueType,
            RatingOrText, ReviewRating, ReviewType, TextOrTextObject, VideoObjectType,
        },
        nutrition::{Energy, Mass, NutritionInformationSchema, RestrictedDiet},
        recipe::{RecipeCategory, RecipeCuisine, RecipeSchema},
        AtContext, AtType,
    },
    websites::Website,
};
