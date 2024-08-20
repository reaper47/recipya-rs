use axum_test::TestResponse;

pub fn assert_html(got: TestResponse, want: Vec<&str>) {
    for s in want {
        got.assert_text_contains(s);
    }
}

pub fn assert_not_in_html(got: TestResponse, want: Vec<&str>) {
    let text = got.text();
    for s in want {
        assert!(!text.contains(s), "expected `{s}` not to be in html");
    }
}
