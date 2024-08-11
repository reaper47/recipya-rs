use iso8601::{
    Date::YMD,
    Duration::YMDHMS,
    {DateTime, Time},
};
use lib_scraper::{
    schema::{common::AggregateRating, recipe::RecipeSchema, AtType},
    websites::Website,
};
use url::Url;

mod helpers;

#[test]
fn test_claudia_abril_dot_com_dot_br() {
    let got = helpers::scrape(Website::ClaudiaAbrilComBr);

    let want = RecipeSchema {
        aggregate_rating: Some(AggregateRating { at_type: AtType::AggregateRating, rating_value: 4, best_rating: 5, rating_count: 38 }),
        at_context: Some(SchemaDotOrg),
        at_type: Some(Container::Vec(Vec::from([AtType::NewsArticle, AtType::Recipe]))),
        author: Some(Organization{at_type: AtType::Organization, name: "CLAUDIA".to_string(), url: Some(Url::parse("https://claudia.abril.com.br").unwrap())}),
        content_rating: Some("Fácil".to_string()),
        cooking_method: Some("Refogado".to_string()),
        cook_time: Some(YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 30, second: 0, millisecond: 0 }),
        date_modified: Some(DateTime { date: YMD { year: 2020, month: 2, day: 5 }, time: Time { hour: 7, minute: 51, second: 35, millisecond: 0, tz_offset_hours: -3, tz_offset_minutes: 0 } }),
        date_published: Some(DateTime { date: YMD { year: 2008, month: 10, day: 24 }, time: Time { hour: 20, minute: 47, second: 0, millisecond: 0, tz_offset_hours: -2, tz_offset_minutes: 0 } }),
        description: Some("Derreta a manteiga e refogue a cebola até ficar transparente. Junte a carne e tempere com o sal. Mexa até a carne dourar de todos os lados. Acrescente a mostarda, o catchup, a pimenta-do-reino e o tomate picado. Cozinhe até formar um molho espesso. Se necessário, adicione água quente aos poucos. Quando o molho estiver [&hellip;]".to_string()),
        headline: Some("Estrogonofe de carne".to_string()),
        image: Some(Container::Item(Url::parse("https://claudia.abril.com.br/wp-content/uploads/2020/02/receita-estrogonofe-de-carne.jpg?quality=85&strip=info&w=620&h=372&crop=1?crop=1&resize=1212,909").unwrap())),
        keywords: Some(Container::Item("Estrogonofe de carne, Refogado, Dia a Dia, Carne, Brasileira, creme de leite, ketchup (ou catchup), pimenta-do-reino".to_string())),
        name: Some("Estrogonofe de carne".to_string()),
        prep_time: Some(YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 30, second: 0, millisecond: 0 }),
        recipe_category: Some(Container::Item("Carne".to_string())),
        recipe_cuisine: Some(Container::Item("Brasileira".to_string())),
        recipe_ingredients: Some(vec![
            "500 gramas de alcatra cortada em tirinhas".to_string(),
            "1/4 xícara (chá) de manteiga ".to_string(),
            "1 unidade de cebola picada".to_string(),
            "1 colher (sobremesa) de mostarda ".to_string(),
            "1 colher (sopa) de ketchup (ou catchup) ".to_string(),
            "1 pitada de pimenta-do-reino ".to_string(),
            "1 unidade de tomate sem pele picado".to_string(),
            "1 xícara (chá) de cogumelo variado | variados escorridos".to_string(),
            "1 lata de creme de leite ".to_string(),
            " sal a gosto".to_string(),
        ]),
        recipe_instructions: Some(Instructions::Text("Derreta a manteiga e refogue a cebola até ficar transparente.Junte a carne e tempere com o sal.Mexa até a carne dourar de todos os lados.Acrescente a mostarda, o catchup, a pimenta-do-reino e o tomate picado.Cozinhe até formar um molho espesso.Se necessário, adicione água quente aos poucos.Quando o molho estiver encorpado e a carne macia, adicione os cogumelos e o creme de leite.Mexa por 1 minuto e retire do fogo.Sirva imediatamente, acompanhado de arroz e batata palha.Dica:\u{a0}Se juntar água ao refogar a carne, frite-a até todo o líquido evaporar.".to_string())),
        recipe_yield: Some(Yield::Num(4)),
        total_time: Some(YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 30, second: 0, millisecond: 0 }),
        ..Default::default()
    };
    assert_eq!(got, want);
}
