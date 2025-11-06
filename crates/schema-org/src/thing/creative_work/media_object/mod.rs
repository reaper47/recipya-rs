mod data_download;

pub use data_download::*;

use schemars::JsonSchema;
use serde::Deserialize;

use crate::data_type::text::URL;
use crate::thing::CreativeWork;
use crate::thing::creative_work::article::NewsArticle;

/// A media object, such as an image, video, audio, or text object embedded in a web page or a
/// downloadable dataset i.e. DataDownload. Note that a creative work may have many media objects
/// associated with it on the same web page. For example, a page about a single song (MusicRecording)
/// may have a music video (VideoObject), and a high and low bandwidth audio stream (2 AudioObject's).
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MediaObject {
    /// A NewsArticle associated with the Media Object.
    pub associated_article: NewsArticle,
    /// The bitrate of the media object.
    pub bitrate: String,
    /// File size in (mega/kilo)bytes.
    pub content_size: String,
    /// Actual bytes of the media object, for example the image file or video file.
    pub content_url: URL,
    /// The duration of the item (movie, audio recording, event, etc.) in ISO 8601 duration format.
    pub duration: DurationOrQuantitativeValue,
    /// A URL pointing to a player for a specific video. In general, this is the information in the
    /// src element of an embed tag and should not be the same as the content of the loc tag.
    pub embed_url: URL,
    /// The CreativeWork encoded by this media object.
    ///
    /// Inverse property: encoding
    pub encodes_creative_work: CreativeWork,
    /// Media type typically expressed using a MIME format (see IANA site and MDN reference), e.g.
    /// application/zip for a SoftwareApplication binary, audio/mpeg for .mp3 etc.
    ///
    ///     In cases where a CreativeWork has several media type representations, encoding can be
    /// used to indicate each MediaObject alongside particular encodingFormat information.
    ///
    ///     Unregistered or niche encoding and file formats can be indicated instead via the most
    /// appropriate URL, e.g. defining Web page or a Wikipedia/Wikidata entry. Supersedes
    /// fileFormat.
    pub encoding_format: TextOrUrl,
    /// The endTime of something. For a reserved event or service
    /// (e.g. FoodEstablishmentReservation), the time that it is expected to end. For actions that
    /// span a period of time, when the action was performed. E.g. John wrote a book from January
    /// to December. For media, including audio and video, it's the time offset of the end of a clip
    /// within a larger file.
    ///
    /// Note that Event uses startDate/endDate instead of startTime/endTime, even when describing
    /// dates with times. This situation may be clarified in future revisions.
    pub end_time: DateTimeOrDate,
    /// The height of the item.
    pub height: DistanceOrQuantitativeValue,
    /// The ISO 3166-1 (ISO 3166-1 alpha-2) or ISO 3166-2 code, the place, or the GeoShape for the
    /// geo-political region(s) for which the offer or delivery charge specification is not valid,
    /// e.g. a region where the transaction is not allowed.
    ///
    /// See also eligibleRegion.
    pub ineligible_region: GeoShapeOrPlaceOrText,
    /// Used to indicate a specific claim contained, implied, translated or refined from the content
    /// of a MediaObject or other CreativeWork. The interpreting party can be indicated using
    /// claimInterpreter.
    pub interpreted_as_claim: Claim,
    /// Player type required—for example, Flash or Silverlight.
    pub player_type: String,
    /// The production company or studio responsible for the item, e.g. series, video game,
    /// episode etc.
    pub production_company: Organization,
    /// The regions where the media is allowed. If not specified, then it's assumed to be allowed
    /// everywhere. Specify the countries in ISO 3166 format.
    pub regions_allowed: Place,
    /// Indicates if use of the media require a subscription (either paid or free). Allowed values
    /// are true or false (note that an earlier version had 'yes', 'no').
    pub requires_subscription: BooleanOrMediaSubscription,
    /// The SHA-2 SHA256 hash of the content of the item. For example, a zero-length input has
    /// value 'e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855'.
    pub sha256: String,
    /// The startTime of something. For a reserved event or service
    /// (e.g. FoodEstablishmentReservation), the time that it is expected to start. For actions that
    /// span a period of time, when the action was performed. E.g. John wrote a book from January
    /// to December. For media, including audio and video, it's the time offset of the start of a
    /// clip within a larger file.
    ///
    /// Note that Event uses startDate/endDate instead of startTime/endTime, even when describing
    /// dates with times. This situation may be clarified in future revisions.
    pub start_time: DateTimeOrDate,
    /// Date (including time if available) when this media object was uploaded to this site.
    pub upload_date: DateTimeOrDate,
    /// The width of the item.
    pub width: DistanceOrQuantitativeValue,
    #[serde(flatten)]
    pub creative_work: CreativeWork,
}
