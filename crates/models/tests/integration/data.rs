use models::{
    data::{PageSlot, PaginationData, PaginationHtmxData, PaginationSearchData, SearchbarData},
    params::SearchParams,
    recipe::structs::time::Times,
    time::FormattedTimes,
};

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

mod tests_formatted_times {
    use super::*;

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
    use super::*;

    #[test]
    fn test_searchbar_data_from_query_none_ok() {
        let query = SearchParams {
            is_favourites: None,
            q: None,
            sort: None,
            page: None,
        };

        let got = SearchbarData::from_params(query);

        let expected = SearchbarData {
            is_favourites: false,
            sort: String::new(),
            term: String::new(),
        };
        pretty_assertions::assert_eq!(expected, got);
    }

    #[test]
    fn test_searchbar_data_from_query_ok() {
        let query = SearchParams {
            is_favourites: None,
            q: Some("hamburger".into()),
            sort: Some("z-a".into()),
            page: Some(4),
        };

        let got = SearchbarData::from_params(query);

        let expected = SearchbarData {
            is_favourites: false,
            sort: "z-a".into(),
            term: "hamburger".into(),
        };
        pretty_assertions::assert_eq!(expected, got);
    }
}

mod tests_pagination {
    use super::*;

    const RESULTS_PER_PAGE: u64 = 15;
    const ID: &str = "pagination-recipes";

    #[test]
    fn test_pagination_new_some_results_ok() {
        let got = PaginationData::new(
            ID,
            "/recipes",
            String::new(),
            1,
            20,
            RESULTS_PER_PAGE,
            PaginationHtmxData {
                is_swap: false,
                target: "#content".into(),
            },
            None,
        );

        pretty_assertions::assert_eq!(
            got,
            PaginationData {
                prev: 1,
                selected: 1,
                next: 2,
                htmx: PaginationHtmxData {
                    is_swap: false,
                    target: "#content".into()
                },
                search: PaginationSearchData { current_page: 1 },
                slots: vec![PageSlot::Page(1), PageSlot::Page(2),],
                is_hidden: false,
                num_pages: 2,
                num_results: 20,
                results_per_page: 15,
                url: "/recipes".into(),
                url_queries: String::new(),
                id: ID.to_string(),
                additional_css: None,
            }
        );
    }

    #[test]
    fn test_pagination_new_no_results_ok() {
        let got = PaginationData::new(
            ID,
            "/recipes",
            String::new(),
            2,
            12,
            RESULTS_PER_PAGE,
            PaginationHtmxData {
                is_swap: false,
                target: "#content".into(),
            },
            None,
        );

        pretty_assertions::assert_eq!(
            got,
            PaginationData {
                prev: 1,
                selected: 1,
                next: 1,
                htmx: PaginationHtmxData {
                    is_swap: false,
                    target: "#content".into()
                },
                search: PaginationSearchData { current_page: 1 },
                slots: vec![PageSlot::Page(1)],
                is_hidden: false,
                num_pages: 1,
                num_results: 12,
                results_per_page: 15,
                url: "/recipes".into(),
                url_queries: String::new(),
                id: ID.to_string(),
                additional_css: None,
            }
        );
    }

    #[test]
    fn test_pagination_new_hundreds_results_left_ok() {
        let got = PaginationData::new(
            ID,
            "/recipes",
            String::new(),
            4,
            258,
            RESULTS_PER_PAGE,
            PaginationHtmxData {
                is_swap: false,
                target: "#content".into(),
            },
            None,
        );

        pretty_assertions::assert_eq!(
            got,
            PaginationData {
                prev: 3,
                selected: 4,
                next: 5,
                htmx: PaginationHtmxData {
                    is_swap: false,
                    target: "#content".into()
                },
                search: PaginationSearchData { current_page: 1 },
                slots: vec![
                    PageSlot::Page(1),
                    PageSlot::Page(2),
                    PageSlot::Page(3),
                    PageSlot::Page(4),
                    PageSlot::Page(5),
                    PageSlot::Ellipsis,
                    PageSlot::Page(18),
                ],
                is_hidden: false,
                num_pages: 18,
                num_results: 258,
                results_per_page: 15,
                url: "/recipes".into(),
                url_queries: String::new(),
                id: ID.to_string(),
                additional_css: None,
            }
        );
    }

    #[test]
    fn test_pagination_new_hundreds_results_middle_ok() {
        let got = PaginationData::new(
            ID,
            "/recipes",
            String::new(),
            11,
            258,
            RESULTS_PER_PAGE,
            PaginationHtmxData {
                is_swap: false,
                target: "#content".into(),
            },
            None,
        );

        pretty_assertions::assert_eq!(
            got,
            PaginationData {
                prev: 10,
                selected: 11,
                next: 12,
                htmx: PaginationHtmxData {
                    is_swap: false,
                    target: "#content".into()
                },
                search: PaginationSearchData { current_page: 1 },
                slots: vec![
                    PageSlot::Page(1),
                    PageSlot::Ellipsis,
                    PageSlot::Page(10),
                    PageSlot::Page(11),
                    PageSlot::Page(12),
                    PageSlot::Ellipsis,
                    PageSlot::Page(18),
                ],
                is_hidden: false,
                num_pages: 18,
                num_results: 258,
                results_per_page: 15,
                url: "/recipes".into(),
                url_queries: String::new(),
                id: ID.to_string(),
                additional_css: None,
            }
        );
    }

    #[test]
    fn test_pagination_new_hundreds_results_right_ok() {
        let got = PaginationData::new(
            ID,
            "/recipes",
            String::new(),
            16,
            258,
            RESULTS_PER_PAGE,
            PaginationHtmxData {
                is_swap: false,
                target: "#content".into(),
            },
            None,
        );

        pretty_assertions::assert_eq!(
            got,
            PaginationData {
                prev: 15,
                selected: 16,
                next: 17,
                htmx: PaginationHtmxData {
                    is_swap: false,
                    target: "#content".into()
                },
                search: PaginationSearchData { current_page: 1 },
                slots: vec![
                    PageSlot::Page(1),
                    PageSlot::Ellipsis,
                    PageSlot::Page(14),
                    PageSlot::Page(15),
                    PageSlot::Page(16),
                    PageSlot::Page(17),
                    PageSlot::Page(18),
                ],
                is_hidden: false,
                num_pages: 18,
                num_results: 258,
                results_per_page: 15,
                url: "/recipes".into(),
                url_queries: String::new(),
                id: ID.to_string(),
                additional_css: None,
            }
        );
    }

    #[test]
    fn test_pagination_new_hundreds_results_last_page_ok() {
        let got = PaginationData::new(
            ID,
            "/recipes",
            String::new(),
            18,
            258,
            RESULTS_PER_PAGE,
            PaginationHtmxData {
                is_swap: false,
                target: "#content".into(),
            },
            None,
        );

        pretty_assertions::assert_eq!(
            got,
            PaginationData {
                prev: 17,
                selected: 18,
                next: 18,
                htmx: PaginationHtmxData {
                    is_swap: false,
                    target: "#content".into()
                },
                search: PaginationSearchData { current_page: 1 },
                slots: vec![
                    PageSlot::Page(1),
                    PageSlot::Ellipsis,
                    PageSlot::Page(14),
                    PageSlot::Page(15),
                    PageSlot::Page(16),
                    PageSlot::Page(17),
                    PageSlot::Page(18),
                ],
                is_hidden: false,
                num_pages: 18,
                num_results: 258,
                results_per_page: 15,
                url: "/recipes".into(),
                url_queries: String::new(),
                id: ID.to_string(),
                additional_css: None,
            }
        );
    }

    #[test]
    fn test_pagination_thousands_results_last_page_ok() {
        let got = PaginationData::new(
            ID,
            "/recipes",
            String::new(),
            193,
            2888,
            RESULTS_PER_PAGE,
            PaginationHtmxData {
                is_swap: false,
                target: "#content".into(),
            },
            None,
        );

        pretty_assertions::assert_eq!(
            got,
            PaginationData {
                prev: 192,
                selected: 193,
                next: 193,
                htmx: PaginationHtmxData {
                    is_swap: false,
                    target: "#content".into()
                },
                search: PaginationSearchData { current_page: 1 },
                slots: vec![
                    PageSlot::Page(1),
                    PageSlot::Ellipsis,
                    PageSlot::Page(189),
                    PageSlot::Page(190),
                    PageSlot::Page(191),
                    PageSlot::Page(192),
                    PageSlot::Page(193),
                ],
                is_hidden: false,
                num_pages: 193,
                num_results: 2888,
                results_per_page: 15,
                url: "/recipes".into(),
                url_queries: String::new(),
                id: ID.to_string(),
                additional_css: None,
            }
        );
    }
}
