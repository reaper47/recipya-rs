mod administrative_area;
mod civic_structure;

pub use administrative_area::*;
pub use civic_structure::*;

use schemars::JsonSchema;
use serde::Deserialize;

use crate::Thing;
use crate::data_type::text::URL;
use crate::permutations::postal_address::PostalAddressOrText;
use crate::permutations::text::NumberOrText;
use crate::thing::Event;
use crate::thing::intangible::structured_value::{OpeningHoursSpecification, PropertyValue};

/// Entities that have a somewhat fixed, physical extension.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Place {
    /// A property-value pair representing an additional characteristic of the entity, e.g. a product feature or another characteristic for which there is no matching property in schema.org.
    ///
    ///     Note: Publishers should be aware that applications designed to use specific schema.org properties (e.g. https://schema.org/width, https://schema.org/color, https://schema.org/gtin13, ...) will typically expect such data to be provided using those properties, rather than using the generic property/value mechanism.
    pub additional_property: PropertyValue,
    /// Physical address of the item.
    pub address: PostalAddressOrText,
    /// The overall rating, based on a collection of reviews or ratings, of the item.
    pub aggregate_rating: AggregateRating,
    /// An amenity feature (e.g. a characteristic or service) of the Accommodation. This generic property does not make a statement about whether the feature is included in an offer for the main accommodation or available at extra costs.
    pub amenity_feature: LocationFeatureSpecification,
    /// A short textual code (also called "store code") that uniquely identifies a place of business. The code is typically assigned by the parentOrganization and used in structured URLs.
    ///
    /// For example, in the URL http://www.starbucks.co.uk/store-locator/etc/detail/3047 the code "3047" is a branchCode for a particular branch.
    pub branch_code: String,
    /// The basic containment relation between a place and one that contains it. Supersedes containedIn.
    ///
    /// Inverse property: containsPlace
    pub contained_in_place: Box<Place>,
    /// The basic containment relation between a place and another that it contains.
    ///
    /// Inverse property: containedInPlace
    pub contains_place: Box<Place>,
    /// Upcoming or past event associated with this place, organization, or action. Supersedes events.
    pub event: Event,
    /// The fax number.
    pub fax_number: String,
    /// The geo coordinates of the place.
    pub geo: GeoCoordinatesOrGeoShape,
    /// Represents a relationship between two geometries (or the places they represent), relating a containing geometry to a contained geometry. "a contains b iff no points of b lie in the exterior of a, and at least one point of the interior of b lies in the interior of a". As defined in DE-9IM.
    pub geo_contains: GeospatialGeometryOrPlace,
    /// Represents a relationship between two geometries (or the places they represent), relating a geometry to another that covers it. As defined in DE-9IM.
    pub geo_covered_by: GeospatialGeometryOrPlace,
    /// Represents a relationship between two geometries (or the places they represent), relating a covering geometry to a covered geometry. "Every point of b is a point of (the interior or boundary of) a". As defined in DE-9IM.
    pub geo_covers: GeospatialGeometryOrPlace,
    /// Represents a relationship between two geometries (or the places they represent), relating a geometry to another that crosses it: "a crosses b: they have some but not all interior points in common, and the dimension of the intersection is less than that of at least one of them". As defined in DE-9IM.
    pub geo_crosses: GeospatialGeometryOrPlace,
    /// Represents spatial relations in which two geometries (or the places they represent) are topologically disjoint: "they have no point in common. They form a set of disconnected geometries." (A symmetric relationship, as defined in DE-9IM.)
    pub geo_disjoint: GeospatialGeometryOrPlace,
    /// Represents spatial relations in which two geometries (or the places they represent) are topologically equal, as defined in DE-9IM. "Two geometries are topologically equal if their interiors intersect and no part of the interior or boundary of one geometry intersects the exterior of the other" (a symmetric relationship).
    pub geo_equals: GeospatialGeometryOrPlace,
    /// Represents spatial relations in which two geometries (or the places they represent) have at least one point in common. As defined in DE-9IM.
    pub geo_intersects: GeospatialGeometryOrPlace,
    /// Represents a relationship between two geometries (or the places they represent), relating a geometry to another that geospatially overlaps it, i.e. they have some but not all points in common. As defined in DE-9IM.
    pub geo_overlaps: GeospatialGeometryOrPlace,
    /// Represents spatial relations in which two geometries (or the places they represent) touch: "they have at least one boundary point in common, but no interior points." (A symmetric relationship, as defined in DE-9IM.)
    pub geo_touches: GeospatialGeometryOrPlace,
    /// Represents a relationship between two geometries (or the places they represent), relating a geometry to one that contains it, i.e. it is inside (i.e. within) its interior. As defined in DE-9IM.
    pub geo_within: GeospatialGeometryOrPlace,
    /// The Global Location Number (GLN, sometimes also referred to as International Location Number or ILN) of the respective organization, person, or place. The GLN is a 13-digit number used to identify parties and physical locations.
    pub global_location_number: String,
    /// Certification information about a product, organization, service, place, or person.
    pub has_certification: Certification,
    /// Indicates whether some facility (e.g. FoodEstablishment, CovidTestingFacility) offers a service that can be used by driving through in a car. In the case of CovidTestingFacility such facilities could potentially help with social distancing from other potentially-infected users.
    pub has_drive_through_service: bool,
    /// The GS1 digital link associated with the object. This URL should conform to the particular requirements of digital links. The link should only contain the Application Identifiers (AIs) that are relevant for the entity being annotated, for instance a Product or an Organization, and for the correct granularity. In particular, for products:
    /// - A Digital Link that contains a serial number (AI 21) should only be present on instances of IndividualProduct
    /// - A Digital Link that contains a lot number (AI 10) should be annotated as SomeProduct if only products from that lot are sold, or IndividualProduct if there is only a specific product.
    /// - A Digital Link that contains a global model number (AI 8013) should be attached to a Product or a ProductModel.
    /// - Other item types should be adapted similarly.
    #[serde(rename = "hasGS1DigitalLink")]
    pub has_gs1_digital_link: URL,
    /// A URL to a map of the place. Supersedes maps, map.
    pub has_map: MapOrURL,
    /// A flag to signal that the item, event, or place is accessible for free. Supersedes free.
    pub is_accessible_for_free: bool,
    /// The International Standard of Industrial Classification of All Economic Activities (ISIC), Revision 4 code for a particular organization, business person, or place.
    pub isic_v4: String,
    /// Keywords or tags used to describe some item. Multiple textual entries in a keywords list are typically delimited by commas, or by repeating the property.
    pub keywords: DefinedTermOrTextOrURL,
    /// The latitude of a location. For example 37.42242 (WGS 84).
    pub latitude: NumberOrText,
    /// An associated logo.
    pub logo: ImageObjectOrURL,
    /// The longitude of a location. For example -122.08585 (WGS 84).
    pub longitude: NumberOrText,
    /// The total number of individuals that may attend an event or venue.
    pub maximum_attendee_capacity: i32,
    /// The opening hours of a certain place.
    pub opening_hours_specification: OpeningHoursSpecification,
    /// A photograph of this place. Supersedes photos.
    pub photo: ImageObjectOrPhotograph,
    /// A flag to signal that the Place is open to public visitors. If this property is omitted there is no assumed default boolean value.
    pub public_access: bool,
    /// A review of the item. Supersedes reviews.
    pub review: Review,
    /// A slogan or motto associated with the item.
    pub slogan: String,
    /// Indicates whether it is allowed to smoke in the place, e.g. in the restaurant, hotel or hotel room.
    pub smoking_allowed: bool,
    /// The special opening hours of a certain place.
    ///
    /// Use this to explicitly override general opening hours brought in scope by openingHoursSpecification or openingHours.
    pub special_opening_hours_specification: OpeningHoursSpecification,
    /// The telephone number.
    pub telephone: String,
    /// A page providing information on how to book a tour of some Place, such as an Accommodation or ApartmentComplex in a real estate setting, as well as other kinds of tours as appropriate.
    pub tour_booking_page: URL,
    #[serde(flatten)]
    pub thing: Thing,
}
