use schemars::JsonSchema;
use serde::Deserialize;

use crate::thing::Action;
use crate::thing::place::AdministrativeArea;

/// Intended audience for an item, i.e. the group for whom the item was created.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Audience {
    /// The target group associated with a given audience
    /// (e.g. veterans, car owners, musicians,
    pub audience_type: String,
    /// The geographic area associated with the audience.
    pub geographic_area: AdministrativeArea,
    /// An additional type for the item, typically used for adding more specific types from external
    /// vocabularies in microdata syntax. This is a relationship between something and a class that
    /// the thing is in. Typically the value is a URI-identified RDF class, and in this case
    /// corresponds to the use of rdf:type in RDF. Text values can be used sparingly, for cases
    /// where useful information can be added without their being an appropriate schema to
    /// reference. In the case of text values, the class label should follow the schema.org style
    /// guide.
    pub additional_type: TextOrUrl,
    /// An alias for the item.
    pub alternate_name: String,
    /// A description of the item.
    pub description: TextOrTextObject,
    /// A sub property of description. A short description of the item used to disambiguate from
    /// other, similar items. Information from other properties (in particular, name) may be
    /// necessary for the description to be useful for disambiguation.
    pub disambiguating_description: String,
    /// The identifier property represents any kind of identifier for any kind of Thing, such as
    /// ISBNs, GTIN codes, UUIDs etc. Schema.org provides dedicated properties for representing many
    /// of these, either as textual strings or as URL (URI) links. See background notes for more
    /// details.
    pub identifier: PropertyValueOrTextOrURL,
    /// An image of the item. This can be a URL or a fully described ImageObject.
    pub image: ImageObjectOrUrl,
    /// Indicates a page (or other CreativeWork) for which this thing is the main entity being
    /// described. See background notes for details.
    /// Inverse property: mainEntity
    pub main_entity_of_page: CreativeWorkOrURL,
    /// The name of the item.
    pub name: String,
    /// Indicates a potential Action, which describes an idealized action in which this thing would
    /// play an 'object' role.
    pub potential_action: Action,
    /// URL of a reference Web page that unambiguously indicates the item's identity. E.g. the URL
    /// of the item's Wikipedia page, Wikidata entry, or official website.
    pub same_as: String,
    /// A CreativeWork or Event about this Thing.
    /// Inverse property: about
    pub subject_of: CreativeWorkOrEvent,
    /// URL of the item.
    pub url: String,
}
