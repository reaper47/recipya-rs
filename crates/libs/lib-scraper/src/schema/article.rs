use serde::Deserialize;
use url::Url;

use crate::schema::{
    common::{
        Action, CreativeWorkOrUrl, DateOrDateTime, DefinedTermOrTextOrUrl, ImageObjectOrUrl,
        LanguageOrText, OrganizationOrPerson, OrganizationType,
    },
    AtType,
};

/// An article, such as a news article or piece of investigative report. Newspapers and magazines
/// have articles of many different types and this is intended to cover them all.
#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ArticleSchema {
    #[serde(rename = "@type", default = "set_article")]
    pub at_type: AtType,

    #[serde(rename = "@id")]
    pub at_id: Option<Url>,

    /// Articles may belong to one or more 'sections' in a magazine or newspaper, such as
    /// Sports, Lifestyle, etc.
    pub article_section: Vec<String>,

    /// The author of this content or rating. Please note that author is special in that HTML 5
    /// provides a special mechanism for indicating authorship via the rel tag. That is equivalent
    /// to this and may be used interchangeably.
    pub author: Option<OrganizationType>,

    /// The number of comments this CreativeWork (e.g. Article, Question or Answer) has received.
    /// This is most applicable to works published in Web sites with commenting system;
    /// additional comments may exist elsewhere.
    pub comment_count: Option<i32>,

    /// The date on which the CreativeWork was most recently modified or when the item's entry
    /// was modified within a DataFeed.
    pub date_modified: Option<DateOrDateTime>,

    /// Date of first publication or broadcast. For example the date a CreativeWork was broadcast
    /// or a Certification was issued.
    pub date_published: Option<DateOrDateTime>,

    /// Headline of the article.
    pub headline: Option<String>,

    /// An image of the item. This can be a URL or a fully described ImageObject.
    pub image: Option<ImageObjectOrUrl>,

    /// The language of the content or performance or used in an action. Please use one of the
    /// language codes from the IETF BCP 47 standard. See also availableLanguage. Supersedes language.
    pub in_language: Option<LanguageOrText>,

    /// Indicates an item or CreativeWork that this item, or CreativeWork (in some sense), is part of.
    // Inverse property: hasPart
    pub is_part_of: Option<CreativeWorkOrUrl>,

    /// Keywords or tags used to describe some item. Multiple textual entries in a keywords list
    /// are typically delimited by commas, or by repeating the property.
    #[serde(alias = "Keywords")]
    pub keywords: Option<DefinedTermOrTextOrUrl>,

    /// Indicates a page (or other CreativeWork) for which this thing is the main entity being
    /// described. See background notes for details. Inverse property: mainEntity
    pub main_entity_of_page: Option<CreativeWorkOrUrl>,

    /// Indicates a potential Action, which describes an idealized action in which this thing
    /// would play an 'object' role.
    pub potential_action: Option<Action>,

    /// The publisher of the creative work.
    pub publisher: Option<OrganizationOrPerson>,

    /// A thumbnail image relevant to the Thing.
    pub thumbnail_url: Option<Url>,

    /// The number of words in the text of the Article.
    pub word_count: Option<i32>,
}

fn set_article() -> AtType {
    AtType::Article
}
