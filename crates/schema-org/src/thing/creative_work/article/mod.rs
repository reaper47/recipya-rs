mod news_article;

pub use news_article::*;

use schemars::JsonSchema;
use serde::Deserialize;

use crate::permutations::creative_work::CreativeWorkOrText;
use crate::permutations::text::IntegerOrText;
use crate::permutations::url::SpeakableSpecificationOrURL;
use crate::thing::CreativeWork;

/// An article, such as a news article or piece of investigative report. Newspapers and magazines
/// have articles of many different types and this is intended to cover them all.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct NewsArticle {
    /// The actual body of the article.
    pub article_body: String,
    /// Articles may belong to one or more 'sections' in a magazine or newspaper, such as Sports,
    /// Lifestyle, etc.
    pub article_section: String,
    /// For an Article, typically a NewsArticle, the backstory property provides a textual summary
    /// giving a brief explanation of why and how an article was created. In a journalistic setting
    /// this could include information about reporting process, methods, interviews, data sources,
    /// etc.
    pub backstory: CreativeWorkOrText,
    /// The page on which the work ends; for example "138" or "xvi".
    pub page_end: IntegerOrText,
    /// The page on which the work starts; for example "135" or "xiii".
    pub page_start: IntegerOrText,
    /// Any description of pages that is not separated into pageStart and pageEnd; for example,
    /// "1-6, 9, 55" or "10-12, 46-49".
    pub pagination: String,
    /// Indicates sections of a Web page that are particularly 'speakable' in the sense of being
    /// highlighted as being especially appropriate for text-to-speech conversion. Other sections
    /// of a page may also be usefully spoken in particular circumstances; the 'speakable' property
    /// serves to indicate the parts most likely to be generally useful for speech.
    ///
    /// The speakable property can be repeated an arbitrary number of times, with three kinds of
    /// possible 'content-locator' values:
    ///
    /// 1.) id-value URL references - uses id-value of an element in the page being annotated. The
    /// simplest use of speakable has (potentially relative) URL values, referencing identified
    /// sections of the document concerned.
    ///
    /// 2.) CSS Selectors - addresses content in the annotated page, e.g. via class attribute. Use
    /// the cssSelector property.
    ///
    /// 3.) XPaths - addresses content via XPaths (assuming an XML view of the content). Use the
    /// xpath property.
    ///
    /// For more sophisticated markup of speakable sections beyond simple ID references, either CSS
    /// selectors or XPath expressions to pick out document section(s) as speakable. For this we
    /// define a supporting type, SpeakableSpecification which is defined to be a possible value of
    /// the speakable property.
    pub speakable: SpeakableSpecificationOrURL,
    /// The number of words in the text of the CreativeWork such as an Article, Book, etc.
    pub word_count: i32,
    #[serde(flatten)]
    pub creative_work: CreativeWork,
}
