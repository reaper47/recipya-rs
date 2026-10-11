#[allow(
    unused_variables,
    clippy::unused_self,
    clippy::missing_panics_doc,
    clippy::used_underscore_items,
    clippy::needless_pass_by_value,
    clippy::non_std_lazy_statics,
    clippy::needless_pass_by_ref_mut,
    clippy::elidable_lifetime_names,
    clippy::unnecessary_wraps,
    clippy::missing_const_for_fn,
    clippy::unnecessary_semicolon,
    clippy::string_lit_as_bytes,
    clippy::all
)]
mod generated {
    include!(concat!(env!("OUT_DIR"), "/generated/fluent/messages.rs"));
}
pub use generated::*;
