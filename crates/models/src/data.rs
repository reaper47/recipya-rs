use iso8601::DateTime;

use config::States;

use crate::RecipeDetails;
use crate::params::SearchParams;
use crate::reports::ViewReport;
use crate::shopping::{ShoppingList, ShoppingListDetails};
use crate::time::FormattedTimes;
use crate::view::ViewMode;

/// Data holds data to pass on to the templates.
#[derive(Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct Data {
    pub states: States,
    pub is_admin: bool,
    pub is_authenticated: bool,
    pub is_hx_request: bool,
    pub is_preview: bool,

    pub about: AboutData,
    pub pagination: Option<PaginationData>,
    pub searchbar: Option<SearchbarData>,
    pub share: Option<ShareData>,
    pub recipes: Vec<ViewRecipe>,
    pub reports: Option<ReportsData>,
    pub shopping: Option<ShoppingData>,
}

/// Creates a new instance of `AboutData`.
#[derive(Default)]
pub struct AboutData {
    pub is_update_available: bool,
    pub is_check_update: bool,
    pub last_checked_update_at: DateTime,
    pub last_updated_at: DateTime,
}

impl AboutData {
    /// Creates a new instance of `AboutData`.
    pub const fn new(
        is_update_available: bool,
        is_check_update: bool,
        last_checked_update_at: DateTime,
        last_updated_at: DateTime,
    ) -> Self {
        Self {
            is_update_available,
            is_check_update,
            last_checked_update_at,
            last_updated_at,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum PageSlot {
    Page(u64),
    Ellipsis,
}

/// Holds pagination data.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct PaginationData {
    pub prev: u64,
    pub selected: u64,
    pub next: u64,

    pub htmx: PaginationHtmxData,
    pub search: PaginationSearchData,
    pub slots: Vec<PageSlot>,

    pub is_hidden: bool,
    pub num_pages: u64,
    pub num_results: u64,
    pub results_per_page: u64,
    pub url: String,
    pub url_queries: String,

    pub id: String,
    pub additional_css: Option<String>,
}

impl PaginationData {
    /// Sets the visibility of the pagination footer to hidden.
    pub fn hidden() -> Self {
        Self {
            htmx: PaginationHtmxData {
                is_swap: true,
                ..Default::default()
            },
            is_hidden: true,
            ..Default::default()
        }
    }
}

/// Holds data related to htmx for pagination.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct PaginationHtmxData {
    pub is_swap: bool,
    pub target: String,
}

/// olds search data for the pagination.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct PaginationSearchData {
    pub current_page: u64,
}

impl PaginationData {
    /// Creates a struct with calculated pagination data for recipes.
    pub fn new_for_recipes(params: &SearchParams, num_recipes: i64, is_htmx_swap: bool) -> Self {
        let htmx = PaginationHtmxData {
            is_swap: is_htmx_swap,
            target: "#content".into(),
        };

        let queries = params
            .sort
            .as_ref()
            .map_or_else(String::new, |sort| format!("sort={sort}"));

        let mut page = params.page.unwrap_or(1);
        if page < 1 {
            page = 1;
        }

        Self::new(
            "pagination-recipes",
            "/recipes",
            queries,
            page,
            num_recipes.cast_unsigned(),
            15,
            htmx,
            None,
        )
    }

    /// Creates a struct with calculated pagination data.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: &str,
        base_url: &str,
        url_queries: String,
        current_page: u64,
        num_results: u64,
        results_per_page: u64,
        htmx: PaginationHtmxData,
        additional_css: Option<&str>,
    ) -> Self {
        let num_pages = std::cmp::max(1, num_results.div_ceil(results_per_page));

        let selected = if num_results < (current_page - 1) * 15 {
            1
        } else {
            current_page
        };

        Self {
            prev: std::cmp::max(1, selected - 1),
            selected,
            next: if selected == num_pages {
                selected
            } else {
                selected + 1
            },
            htmx,
            search: PaginationSearchData { current_page: 1 },
            slots: page_slots(selected, num_pages),
            is_hidden: num_pages == 0,
            num_pages,
            num_results,
            results_per_page: 15,
            url: base_url.to_string(),
            url_queries,
            id: id.to_string(),
            additional_css: additional_css.map(ToString::to_string),
        }
    }
}

fn page_slots(curr: u64, total: u64) -> Vec<PageSlot> {
    if total <= 7 {
        return (1..=total).map(PageSlot::Page).collect();
    }

    let mut slots = Vec::with_capacity(7);

    if curr <= 4 {
        for i in 1..=5 {
            slots.push(PageSlot::Page(i));
        }
        slots.push(PageSlot::Ellipsis);
        slots.push(PageSlot::Page(total));
    } else if curr >= total - 3 {
        slots.push(PageSlot::Page(1));
        slots.push(PageSlot::Ellipsis);
        for i in (total - 4)..=total {
            slots.push(PageSlot::Page(i));
        }
    } else {
        slots.push(PageSlot::Page(1));
        slots.push(PageSlot::Ellipsis);
        slots.push(PageSlot::Page(curr - 1));
        slots.push(PageSlot::Page(curr));
        slots.push(PageSlot::Page(curr + 1));
        slots.push(PageSlot::Ellipsis);
        slots.push(PageSlot::Page(total));
    }

    slots
}

/// Holds data related to the reports.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ReportsData {
    pub reports: Vec<ViewReport>,
    pub selected: Option<ViewReport>,
    pub page: i64,
}

/// Holds data related to the searchbar.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SearchbarData {
    pub is_favourites: bool,
    pub sort: String,
    pub term: String,
}

impl SearchbarData {
    /// Creates a `SearchbarData` from the query parameters.
    pub fn from_params(params: SearchParams) -> Self {
        Self {
            is_favourites: params.is_favourites.unwrap_or_default(),
            term: params.q.unwrap_or_default(),
            sort: params.sort.unwrap_or_default(),
        }
    }
}

/// Holds information on the entity being shared.
#[derive(Debug)]
pub struct ShareData {
    pub is_from_host: bool,
    pub is_shared: bool,
}

/// Holds data related to the shopping module.
pub struct ShoppingData {
    pub labels: Option<Vec<String>>,
    pub shopping_lists: Vec<ShoppingList>,
    pub selected_shopping_list: Option<ShoppingListDetails>,
    pub selected_view_mode: ViewMode,
}

/// Holds template data related to viewing a recipe.
#[derive(Clone)]
pub struct ViewRecipe {
    pub recipe_details: RecipeDetails,
    pub formatted_times: FormattedTimes,
}
