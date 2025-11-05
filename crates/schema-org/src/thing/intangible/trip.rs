use schemars::JsonSchema;
use serde::Deserialize;

use crate::components::{DateTimeOrDate, OrganizationOrPerson, Place, Thing};

/// A trip or journey. An itinerary of visits to one or more places.
#[derive(Debug, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Trip {
    /// The expected arrival time.
    pub arrival_time: DateTimeOrDate,
    /// The expected departure time.
    pub departure_time: DateTimeOrDate,
    /// Destination(s) ( Place ) that make up a trip. For a trip where destination order is
    /// important use ItemList to specify that order (see examples).
    pub itinerary: ItemListOrPlace,
    /// An offer to provide this item—for example, an offer to sell a product, rent the DVD of a
    /// movie, perform a service, or give away tickets to an event. Use businessFunction to indicate
    /// the kind of transaction offered, i.e. sell, lease, etc. This property can also be used to
    /// describe a Demand. While this property is listed as expected on a number of common types,
    /// it can be used in others. In that case, using a second type, such as Product or a subtype of
    /// Product, can clarify the nature of the offer.
    ///
    /// Inverse property: itemOffered
    pub offers: DemandOrOffer,
    /// Identifies that this Trip is a subTrip of another Trip. For example Day 1, Day 2, etc. of a
    /// multi-day trip.
    ///
    /// Inverse property: subTrip
    pub part_of_trip: Box<Thing>,
    /// The service provider, service operator, or service performer; the goods producer. Another
    /// party (a seller) may offer those services or goods on behalf of the provider. A provider may
    /// also serve as the seller. Supersedes carrier.
    pub provider: OrganizationOrPerson,
    /// Identifies a Trip that is a subTrip of this Trip. For example Day 1, Day 2, etc. of a multi-day trip.
    ///
    /// Inverse property: partOfTrip
    pub sub_trip: Box<Thing>,
    /// The location of origin of the trip, prior to any destination(s).
    pub trip_origin: Place,

    pub thing: Thing,
}
