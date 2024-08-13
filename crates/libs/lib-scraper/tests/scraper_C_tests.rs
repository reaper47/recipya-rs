use iso8601::{
    Date::YMD,
    Duration::YMDHMS,
    {DateTime, Time},
};

use lib_scraper::{
    schema::{
        common::{
            AggregateRating, CreativeWorkOrItemListOrText, CreativeWorkOrUrl, CreativeWorkType,
            DateOrDateTime, DefinedTermOrTextOrUrl, DistanceOrQuantitativeValue, DistanceType,
            ImageObjectOrUrl, ImageObjectType, NumberOrText, OrganizationOrPerson,
            OrganizationType, QuantitativeValueOrText, QuantitativeValueType, RatingOrText,
            ReviewRating, ReviewType, TextOrTextObject,
        },
        recipe::{RecipeCategory, RecipeCuisine, RecipeSchema},
        AtContext, AtType,
    },
    websites::Website,
};
use url::Url;

mod support;

#[test]
fn test_claudia_abril_dot_com_dot_br() {
    let got = support::scrape(Website::ClaudiaAbrilComBr, 0);

    let want = RecipeSchema {
        aggregate_rating: Some(AggregateRating {
            at_type: AtType::AggregateRating,
            rating_value: Some(NumberOrText::Number(4)),
            best_rating: Some(5),
            rating_count: Some(38),
            ..Default::default()
        }),
        article_body: Some("Derreta a manteiga e refogue a cebola até ficar transparente.Junte a carne e tempere com o sal.Mexa até a carne dourar de todos os lados.Acrescente a mostarda, o catchup, a pimenta-do-reino e o tomate picado.Cozinhe até formar um molho espesso.Se necessário, adicione água quente aos poucos.Quando o molho estiver encorpado e a carne macia, adicione os cogumelos e o creme de leite.Mexa por 1 minuto e retire do fogo.Sirva imediatamente, acompanhado de arroz e batata palha.Dica:&nbsp;Se juntar água ao refogar a carne, frite-a até todo o líquido evaporar.".to_string()),
        at_context: AtContext::SchemaDotOrg,
        at_type: Some(AtType::Recipe),
        author: Some(OrganizationType {
            at_type: AtType::Organization,
            name: Some("CLAUDIA".to_string()),
            url: Some(Url::parse("https://claudia.abril.com.br").unwrap()),
            ..Default::default()
        }),
        content_rating: Some(RatingOrText::Text("Fácil".to_string())),
        cooking_method: Some("Refogado".to_string()),
        cook_time: Some(YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 30, second: 0, millisecond: 0 }),
        date_modified: Some(DateOrDateTime::DateTime(DateTime { date: YMD { year: 2020, month: 2, day: 5 }, time: Time { hour: 7, minute: 51, second: 35, millisecond: 0, tz_offset_hours: -3, tz_offset_minutes: 0 } })),
        date_published: Some(DateOrDateTime::DateTime(DateTime { date: YMD { year: 2008, month: 10, day: 24 }, time: Time { hour: 20, minute: 47, second: 0, millisecond: 0, tz_offset_hours: -2, tz_offset_minutes: 0 } })),
        description: Some(TextOrTextObject::Text("Derreta a manteiga e refogue a cebola até ficar transparente. Junte a carne e tempere com o sal. Mexa até a carne dourar de todos os lados. Acrescente a mostarda, o catchup, a pimenta-do-reino e o tomate picado. Cozinhe até formar um molho espesso. Se necessário, adicione água quente aos poucos. Quando o molho estiver [&amp;hellip;]".to_string())),
        headline: Some("Estrogonofe de carne".to_string()),
        image: Some(ImageObjectOrUrl::Url(Url::parse("https://claudia.abril.com.br/wp-content/uploads/2020/02/receita-estrogonofe-de-carne.jpg?quality=85&amp;strip=info&amp;w=620&amp;h=372&amp;crop=1?crop=1&amp;resize=1212,909").unwrap())),
        is_part_of: Some(CreativeWorkOrUrl::CreativeWork(CreativeWorkType {
            description: Some(TextOrTextObject::Text("Domine o fato. Confie na fonte.".to_string())),
            image: Some(ImageObjectOrUrl::Url(Url::parse("https://claudia.abril.com.br/wp-content/uploads/2016/09/claudia-schema.png?w=150").unwrap())),
            name: Some("CLAUDIA".to_string()),
            ..Default::default()
        })),
        keywords: Some(DefinedTermOrTextOrUrl::Text("Estrogonofe de carne, Refogado, Dia a Dia, Carne, Brasileira, creme de leite, ketchup (ou catchup), pimenta-do-reino".to_string())),
        main_entity_of_page: Some(CreativeWorkOrUrl::CreativeWork(CreativeWorkType {
            at_type: AtType::WebPage,
            at_id: Some(Url::parse("https://claudia.abril.com.br/receitas/estrogonofe-de-carne").unwrap()),
            ..Default::default()
        })),
        name: Some("Estrogonofe de carne".to_string()),
        prep_time: Some(YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 30, second: 0, millisecond: 0 }),
        publisher: Some(OrganizationOrPerson::Organization(OrganizationType {
            at_type: AtType::Organization,
            name: Some("CLAUDIA".to_string()),
            logo: Some(ImageObjectOrUrl::ImageObject(ImageObjectType {
                at_type: AtType::ImageObject,
                height: Some(DistanceOrQuantitativeValue::Distance(DistanceType { value: "60".to_string() })),
                url: Some(Url::parse("https://claudia.abril.com.br/wp-content/uploads/2016/09/claudia-schema.png?w=240&amp;resize=90,60").unwrap()),
                width: Some(DistanceOrQuantitativeValue::Distance(DistanceType { value: "90".to_string() })),
                ..Default::default()
            })),
            ..Default::default()
        })),
        recipe_category: RecipeCategory::Text("Carne".to_string()),
        recipe_cuisine: Some(RecipeCuisine::Text("Brasileira".to_string())),
        recipe_ingredient: Some(vec![
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
        recipe_instructions: Some(CreativeWorkOrItemListOrText::Text("Derreta a manteiga e refogue a cebola até ficar transparente.Junte a carne e tempere com o sal.Mexa até a carne dourar de todos os lados.Acrescente a mostarda, o catchup, a pimenta-do-reino e o tomate picado.Cozinhe até formar um molho espesso.Se necessário, adicione água quente aos poucos.Quando o molho estiver encorpado e a carne macia, adicione os cogumelos e o creme de leite.Mexa por 1 minuto e retire do fogo.Sirva imediatamente, acompanhado de arroz e batata palha.Dica:&nbsp;Se juntar água ao refogar a carne, frite-a até todo o líquido evaporar.".to_string())),
        recipe_yield: QuantitativeValueOrText::QuantitativeValue(QuantitativeValueType { value: 4 }),
        review: Some(vec![
                      ReviewType {
                          at_type: AtType::Review,
                          review_rating: ReviewRating {
                              at_type: AtType::Rating,
                              rating_value: "5".to_string(),
                          },
                          author: OrganizationOrPerson::Organization(
                              OrganizationType {
                                  at_type: AtType::Organization,
                                  name: Some("CLAUDIA".to_string()),
                                  ..Default::default()
                              },
                          ),
                          date_published: DateOrDateTime::DateTime(DateTime { date: YMD { year: 2008, month: 10, day: 24 }, time: Time { hour: 20, minute: 47, second: 0, millisecond: 0, tz_offset_hours: -2, tz_offset_minutes: 0 } }),
                          review_body: None,
                      },
                  ]),
        total_time: Some(YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 30, second: 0, millisecond: 0 }),
        ..Default::default()
    };
    pretty_assertions::assert_eq!(got, want);
}

#[test]
fn test_cafedelites_dot_com() {
    todo!();
}

#[test]
fn test_canada_dot_ca() {
    todo!();
}

#[test]
fn test_castironketo_dot_com() {
    todo!();
}

#[test]
fn test_cdkitchen_dot_com() {
    todo!();
}

#[test]
fn test_cestmafournee_dot_com() {
    todo!();
}

#[test]
fn test_chatelaine_dot_com() {
    todo!();
}

#[test]
fn test_chefkoch_dot_de() {
    todo!();
}

#[test]
fn test_chefnini_dot_com() {
    todo!();
}

#[test]
fn test_chefsavvy_dot_com() {
    todo!();
}

#[test]
fn test_chejorge_dot_com() {
    todo!();
}

#[test]
fn test_chetnamakan_dot_co_dot_uk() {
    todo!();
}

#[test]
fn test_chinesecookingdemystified_substack_dot_com() {
    todo!();
}

#[test]
fn test_closetcooking_dot_com() {
    todo!();
}

#[test]
fn test_colruyt_dot_be() {
    todo!();
}

#[test]
fn test_comidinhasdochef_dot_com() {
    todo!();
}

#[test]
fn test_cook_talk_dot_com() {
    todo!();
}

#[test]
fn test_cookeatshare_dot_com() {
    todo!();
}

#[test]
fn test_cookieandkate_dot_com() {
    todo!();
}

#[test]
fn test_cookpad_dot_com() {
    todo!();
}

#[test]
fn test_coop_dot_se() {
    todo!();
}

#[test]
fn test_copykat_dot_com() {
    todo!();
}

#[test]
fn test_costco_dot_com() {
    todo!();
}

#[test]
fn test_countryliving_dot_com() {
    todo!();
}

#[test]
fn test_creativecanning_dot_com() {
    todo!();
}

#[test]
fn test_cucchiaio_dot_it() {
    todo!();
}

#[test]
fn test_cuisineandtravel_dot_com() {
    todo!();
}

#[test]
fn test_cuisineaz_dot_com() {
    todo!();
}

#[test]
fn test_culy_dot_nl() {
    todo!();
}

#[test]
fn test_cybercook_dot_com_dot_br() {
    todo!();
}
