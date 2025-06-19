#[macro_export]
macro_rules! impl_display_as_debug {
    ($type:ty) => {
        impl core::fmt::Display for $type {
            fn fmt(
                &self,
                fmt: &mut core::fmt::Formatter,
            ) -> core::result::Result<(), core::fmt::Error> {
                write!(fmt, "{self:?}")
            }
        }
    };
}

/// Defines an entity with a name and establishes a many-to-many relationship with `Recipe`.
///
/// This macro generates:
/// 1. A struct representing the entity with an `id` and `name`.
/// 2. An insertable struct for inserting new entities.
/// 3. A relational struct mapping the entity to a `Recipe` in a many-to-many relationship.
///
#[macro_export]
macro_rules! name_entity_with_relations {
    ($struct_name:ident, $table_name:ident, $relation_table:ident) => {
        /// Represents an entity with an `id` and a `name`, mapped to a database table.
        #[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
        #[diesel(table_name = schema::$table_name)]
        #[diesel(check_for_backend(diesel::pg::Pg))]
        pub struct $struct_name {
            pub id: i64,
            pub name: String,
        }

        paste::paste! {
            /// Represents an insertable version of the entity for database insertion.
            #[derive(Insertable)]
            #[diesel(table_name = schema::$table_name)]
            pub(super) struct [<$struct_name ForInsert>] {
                pub name: Option<String>,
            }
        }

        paste::paste! {
            /// Represents the many-to-many relationship between the entity and `Recipe`.
            #[derive(Identifiable, Selectable, Queryable, Insertable, Associations)]
            #[diesel(belongs_to($struct_name), belongs_to(Recipe))]
            #[diesel(table_name = schema::$relation_table)]
            #[diesel(primary_key([<$struct_name:lower _id>], recipe_id))]
            pub struct [<$struct_name Recipe>] {
                pub [<$struct_name:lower _id>]: i64,
                pub recipe_id: i64,
            }
        }
    };
}
