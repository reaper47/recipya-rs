use models::language::Language;
use test_db::default_config;
use test_harness::create_app_state;

use crate::Result;

#[tokio::test]
async fn test_get_all_languages_ok() -> Result<()> {
    let state = create_app_state(default_config()).await;

    let got = Language::get_all(&state.mm).await?;

    assert_eq!(got.len(), 91);
    Ok(())
}
