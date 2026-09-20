use models::paper::PaperSize;
use test_db::default_config;
use test_harness::create_app_state;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::test]
async fn test_get_all_paper_sizes_ok() -> Result<()> {
    let state = create_app_state(default_config()).await;

    let got = PaperSize::get_all(&state.mm).await?;

    let keys: Vec<&String> = got.keys().collect();
    pretty_assertions::assert_eq!(
        keys,
        vec![
            &"Standard US Sizes".to_string(),
            &"US Envelope Sizes".to_string(),
            &"ISO Envelopes".to_string(),
            &"ISO A Sizes".to_string(),
            &"ISO B Sizes".to_string(),
            &"ISO C Sizes".to_string(),
            &"North American ANSI Sizes".to_string(),
            &"North American ARCH Sizes".to_string(),
        ]
    );
    Ok(())
}

#[tokio::test]
async fn test_get_paper_size_ok() -> Result<()> {
    let state = create_app_state(default_config()).await;

    let got = PaperSize::get(&state.mm, 1).await?;

    pretty_assertions::assert_eq!(
        got,
        PaperSize {
            id: 1,
            paper_category_id: 1,
            name: "US Government Legal".into(),
            height_mm: 330.0,
            width_mm: 216.0,
            height_in: 12.9921,
            width_in: 8.5039,
        }
    );
    Ok(())
}
