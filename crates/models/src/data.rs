use crate::RecipeDetails;
use crate::params::SearchParams;
use crate::time::FormattedTimes;

/// Data holds data to pass on to the templates.
#[derive(Default)]
pub struct Data {
    pub is_admin: bool,
    pub is_authenticated: bool,
    pub is_autologin: bool,
    pub is_hx_request: bool,

    pub about: AboutData,
    pub pagination: Option<PaginationData>,
    pub searchbar: Option<SearchbarData>,
    pub share: Option<ShareData>,
    pub recipes: Vec<ViewRecipe>,
}

/// NewAboutData creates a new instance of AboutData.
#[derive(Default)]
pub struct AboutData {
    pub is_update_available: bool,
}

/// PaginationData holds data related to pagination.
#[derive(Debug, PartialEq)]
pub struct PaginationData {
    pub left: Vec<u64>,
    pub middle: Vec<u64>,
    pub right: Vec<u64>,

    pub prev: u64,
    pub selected: u64,
    pub next: u64,

    pub htmx: PaginationHtmxData,
    pub search: PaginationSearchData,

    pub is_hidden: bool,
    pub num_pages: u64,
    pub num_results: u64,
    pub results_per_page: u64,
    pub url: String,
    pub url_queries: String,
}

/// PaginationHtmxData holds data related to htmx for pagination.
#[derive(Debug, PartialEq)]
pub struct PaginationHtmxData {
    pub is_swap: bool,
    pub target: String,
}

/// PaginationSearchData holds search data for the pagination.
#[derive(Debug, PartialEq)]
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

        let queries = match &params.sort {
            Some(sort) => format!("sort={sort}"),
            None => String::new(),
        };

        let page = params.page.unwrap_or(1);

        Self::new("/recipes", queries, page, num_recipes as u64, htmx)
    }

    fn new(
        base_url: &str,
        url_queries: String,
        current_page: u64,
        num_results: u64,
        htmx: PaginationHtmxData,
    ) -> Self {
        let num_pages = std::cmp::max(1, (num_results + (15 - 1)) / 15);

        let mut left = Vec::<u64>::new();
        let mut middle = Vec::<u64>::new();
        let mut right = Vec::<u64>::new();

        if num_pages <= 10 {
            left.extend(1..=num_pages);
        } else if current_page <= num_pages / 2 {
            if current_page > 4 {
                middle.extend_from_slice(&[
                    current_page - 2,
                    current_page - 1,
                    current_page,
                    current_page + 1,
                    current_page + 2,
                ]);
                left.push(1);
            } else {
                left.extend(1..=current_page + 3);
            }

            right.push(num_pages);
        } else {
            if current_page < num_pages - 3 {
                middle.extend_from_slice(&[
                    current_page - 2,
                    current_page - 1,
                    current_page,
                    current_page + 1,
                    current_page + 2,
                ]);
                right.push(num_pages);
            } else {
                right.extend(current_page - 3..=num_pages);
            }

            left.push(1);
        }

        let selected = if num_results < current_page * 15 {
            1
        } else {
            current_page
        };

        Self {
            left,
            middle,
            right,
            prev: std::cmp::max(1, selected - 1),
            selected,
            next: selected + 1,
            htmx,
            search: PaginationSearchData { current_page: 1 },
            is_hidden: num_pages == 0,
            num_pages,
            num_results,
            results_per_page: 15,
            url: base_url.to_string(),
            url_queries,
        }
    }
}

/// SearchbarData holds data related to the searchbar.
#[derive(Debug, PartialEq)]
pub struct SearchbarData {
    pub sort: String,
    pub term: String,
}

impl SearchbarData {
    /// Creates a SearchbarData from the query parameters.
    pub fn from_params(params: SearchParams) -> Self {
        Self {
            term: params.q.unwrap_or_default(),
            sort: params.sort.unwrap_or_default(),
        }
    }
}

/// ShareData holds information on the entity being shared.
pub struct ShareData {
    pub is_from_host: bool,
    pub is_shared: bool,
}

/// ViewRecipeData holds template data related to viewing a recipe.
#[derive(Clone)]
pub struct ViewRecipe {
    pub recipe_details: RecipeDetails,
    pub formatted_times: FormattedTimes,
}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_formatted_times {
        use super::*;
        use crate::recipe::Times;
        use crate::time::FormattedTimes;

        #[test]
        fn test_with_hours_and_minutes_ok() -> Result<()> {
            let times = Times {
                cook_seconds: 10800,
                prep_seconds: 900,
                total_seconds: 10800 + 900,
                id: 0,
                recipe_id: 0,
            };

            let got = FormattedTimes::from_times(&times)?;

            pretty_assertions::assert_eq!(
                got,
                FormattedTimes {
                    cook: "3h".into(),
                    cook_datetime: "PT3H".into(),
                    cook_edit: "03:00:00".into(),
                    prep: "15m".into(),
                    prep_datetime: "PT15M".into(),
                    prep_edit: "00:15:00".into(),
                    total: "3h 15m".into(),
                    total_datetime: "PT3H15M".into(),
                }
            );
            Ok(())
        }

        #[test]
        fn test_with_minutes_ok() -> Result<()> {
            let times = Times {
                cook_seconds: 960,
                prep_seconds: 900,
                total_seconds: 960 + 900,
                id: 0,
                recipe_id: 0,
            };

            let got = FormattedTimes::from_times(&times)?;

            pretty_assertions::assert_eq!(
                got,
                FormattedTimes {
                    cook: "16m".into(),
                    cook_datetime: "PT16M".into(),
                    cook_edit: "00:16:00".into(),
                    prep: "15m".into(),
                    prep_datetime: "PT15M".into(),
                    prep_edit: "00:15:00".into(),
                    total: "31m".into(),
                    total_datetime: "PT31M".into(),
                }
            );
            Ok(())
        }
    }

    mod tests_searchbar {
        use super::SearchbarData;
        use super::*;

        #[test]
        fn test_searchbar_data_from_query_none_ok() {
            let query = SearchParams {
                q: None,
                sort: None,
                page: None,
            };

            let got = SearchbarData::from_params(query);

            let expected = SearchbarData {
                sort: "".into(),
                term: "".into(),
            };
            pretty_assertions::assert_eq!(expected, got);
        }

        #[test]
        fn test_searchbar_data_from_query_ok() {
            let query = SearchParams {
                q: Some("hamburger".into()),
                sort: Some("z-a".into()),
                page: Some(4),
            };

            let got = SearchbarData::from_params(query);

            let expected = SearchbarData {
                sort: "z-a".into(),
                term: "hamburger".into(),
            };
            pretty_assertions::assert_eq!(expected, got);
        }
    }

    mod tests_pagination {
        use crate::data::{PaginationData, PaginationHtmxData, PaginationSearchData};

        #[test]
        fn test_pagination_new_some_results_ok() {
            let got = PaginationData::new(
                "/recipes",
                "".into(),
                1,
                20,
                PaginationHtmxData {
                    is_swap: false,
                    target: "#content".into(),
                },
            );

            pretty_assertions::assert_eq!(
                got,
                PaginationData {
                    left: vec![1, 2],
                    middle: vec![],
                    right: vec![],
                    prev: 1,
                    selected: 1,
                    next: 2,
                    htmx: PaginationHtmxData {
                        is_swap: false,
                        target: "#content".into()
                    },
                    search: PaginationSearchData { current_page: 1 },
                    is_hidden: false,
                    num_pages: 2,
                    num_results: 20,
                    results_per_page: 15,
                    url: "/recipes".into(),
                    url_queries: String::new()
                }
            );
        }

        #[test]
        fn test_pagination_new_no_results_ok() {
            let got = PaginationData::new(
                "/recipes",
                "".into(),
                2,
                12,
                PaginationHtmxData {
                    is_swap: false,
                    target: "#content".into(),
                },
            );

            pretty_assertions::assert_eq!(
                got,
                PaginationData {
                    left: vec![1],
                    middle: vec![],
                    right: vec![],
                    prev: 1,
                    selected: 1,
                    next: 2,
                    htmx: PaginationHtmxData {
                        is_swap: false,
                        target: "#content".into()
                    },
                    search: PaginationSearchData { current_page: 1 },
                    is_hidden: false,
                    num_pages: 1,
                    num_results: 12,
                    results_per_page: 15,
                    url: "/recipes".into(),
                    url_queries: String::new()
                }
            );
        }

        #[test]
        fn test_pagination_new_hundreds_results_left_ok() {
            let got = PaginationData::new(
                "/recipes",
                "".into(),
                4,
                258,
                PaginationHtmxData {
                    is_swap: false,
                    target: "#content".into(),
                },
            );

            pretty_assertions::assert_eq!(
                got,
                PaginationData {
                    left: vec![1, 2, 3, 4, 5, 6, 7],
                    middle: vec![],
                    right: vec![18],
                    prev: 3,
                    selected: 4,
                    next: 5,
                    htmx: PaginationHtmxData {
                        is_swap: false,
                        target: "#content".into()
                    },
                    search: PaginationSearchData { current_page: 1 },
                    is_hidden: false,
                    num_pages: 18,
                    num_results: 258,
                    results_per_page: 15,
                    url: "/recipes".into(),
                    url_queries: String::new()
                }
            );
        }

        #[test]
        fn test_pagination_new_hundreds_results_middle_ok() {
            let got = PaginationData::new(
                "/recipes",
                "".into(),
                11,
                258,
                PaginationHtmxData {
                    is_swap: false,
                    target: "#content".into(),
                },
            );

            pretty_assertions::assert_eq!(
                got,
                PaginationData {
                    left: vec![1],
                    middle: vec![9, 10, 11, 12, 13],
                    right: vec![18],
                    prev: 10,
                    selected: 11,
                    next: 12,
                    htmx: PaginationHtmxData {
                        is_swap: false,
                        target: "#content".into()
                    },
                    search: PaginationSearchData { current_page: 1 },
                    is_hidden: false,
                    num_pages: 18,
                    num_results: 258,
                    results_per_page: 15,
                    url: "/recipes".into(),
                    url_queries: String::new()
                }
            );
        }

        #[test]
        fn test_pagination_new_hundreds_results_right_ok() {
            let got = PaginationData::new(
                "/recipes",
                "".into(),
                16,
                258,
                PaginationHtmxData {
                    is_swap: false,
                    target: "#content".into(),
                },
            );

            pretty_assertions::assert_eq!(
                got,
                PaginationData {
                    left: vec![1],
                    middle: vec![],
                    right: vec![13, 14, 15, 16, 17, 18],
                    prev: 15,
                    selected: 16,
                    next: 17,
                    htmx: PaginationHtmxData {
                        is_swap: false,
                        target: "#content".into()
                    },
                    search: PaginationSearchData { current_page: 1 },
                    is_hidden: false,
                    num_pages: 18,
                    num_results: 258,
                    results_per_page: 15,
                    url: "/recipes".into(),
                    url_queries: String::new()
                }
            );
        }
    }
}
