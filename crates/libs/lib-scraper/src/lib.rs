use std::sync::Arc;

use scraper::{Html, Selector};

pub use self::error::{Error, Result};
use crate::schema::recipe::GraphObject;
use crate::schema::AtType;
use crate::{schema::recipe::RecipeSchema, websites::Website};

mod custom;
mod error;
pub mod schema;
pub mod websites;

#[async_trait::async_trait]
pub trait HttpClient {
    async fn get_async<'a>(&'a self, host: Website, url: &str) -> Result<String>;
    fn get(&self, host: Website, url: &str) -> Result<String>;
}

struct AppHttpClient {
    client: reqwest::Client,
}

impl AppHttpClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl HttpClient for AppHttpClient {
    async fn get_async<'a>(&'a self, _host: Website, url: &str) -> Result<String> {
        let res = self.client.get(url).send().await?;
        let body = res.text().await?;
        Ok(body)
    }

    fn get(&self, _host: Website, url: &str) -> Result<String> {
        let client = reqwest::blocking::Client::new();
        let res = client.get(url).send()?;
        let body = res.text()?;
        Ok(body)
    }
}

pub struct Scraper {
    pub client: Arc<dyn HttpClient + Sync + Send>,
}

impl Scraper {
    pub fn scrape(&self, url: &str) -> Result<RecipeSchema> {
        let host = match Website::from(url) {
            Ok(host) => host,
            Err(err) => return Err(err),
        };
        let content = self.client.get(host, url)?;
        let doc = Html::parse_document(&content);

        /*let recipe: RecipeSchema = match serde_json::from_str(
            r#"{"@context":"http://schema.org","@type":["NewsArticle","Recipe"],"name":"Estrogonofe de carne","image":"https://claudia.abril.com.br/wp-content/uploads/2020/02/receita-estrogonofe-de-carne.jpg?quality=85&amp;strip=info&amp;w=620&amp;h=372&amp;crop=1?crop=1&amp;resize=1212,909","author":{"@type":"Organization","name":"CLAUDIA","url":"https://claudia.abril.com.br"},"publisher":{"@type":"Organization","name":"CLAUDIA","logo":{"@type":"ImageObject","url":"https://claudia.abril.com.br/wp-content/uploads/2016/09/claudia-schema.png?w=240&amp;resize=90,60","height":"60","width":"90"}},"mainEntityOfPage":{"@type":"WebPage","@id":"https://claudia.abril.com.br/receitas/estrogonofe-de-carne"},"headline":"Estrogonofe de carne","datePublished":"2008-10-24T20:47:00-0200","description":"Derreta a manteiga e refogue a cebola até ficar transparente. Junte a carne e tempere com o sal. Mexa até a carne dourar de todos os lados. Acrescente a mostarda, o catchup, a pimenta-do-reino e o tomate picado. Cozinhe até formar um molho espesso. Se necessário, adicione água quente aos poucos. Quando o molho estiver [&amp;hellip;]","isAccessibleForFree":"False","isPartOf":{"Name":"CLAUDIA","@type":["CreativeWork","Product"],"productID":"claudia.abril.com.br:digital","description":"Domine o fato. Confie na fonte.","image":"https://claudia.abril.com.br/wp-content/uploads/2016/09/claudia-schema.png?w=150","brand":{"@type":"Brand","name":"claudia"},"sku":"claudia.abril.com.br:digital","mpn":"claudia.abril.com.br:digital","offers":[]},"dateModified":"2020-02-05T07:51:35-0300","articleBody":"Derreta a manteiga e refogue a cebola até ficar transparente.Junte a carne e tempere com o sal.Mexa até a carne dourar de todos os lados.Acrescente a mostarda, o catchup, a pimenta-do-reino e o tomate picado.Cozinhe até formar um molho espesso.Se necessário, adicione água quente aos poucos.Quando o molho estiver encorpado e a carne macia, adicione os cogumelos e o creme de leite.Mexa por 1 minuto e retire do fogo.Sirva imediatamente, acompanhado de arroz e batata palha.Dica:&nbsp;Se juntar água ao refogar a carne, frite-a até todo o líquido evaporar.","recipeYield":4,"totalTime":"PT30M","CookTime":"PT30M","PrepTime":"PT30M","Keywords":"Estrogonofe de carne, Refogado, Dia a Dia, Carne, Brasileira, creme de leite, ketchup (ou catchup), pimenta-do-reino","CookingMethod":"Refogado","recipeCuisine":"Brasileira","recipeCategory":"Carne","ContentRating":"Fácil","recipeIngredient":["500 gramas de alcatra cortada em tirinhas","1/4 xícara (chá) de manteiga ","1 unidade de cebola picada","1 colher (sobremesa) de mostarda ","1 colher (sopa) de ketchup (ou catchup) ","1 pitada de pimenta-do-reino ","1 unidade de tomate sem pele picado","1 xícara (chá) de cogumelo variado | variados escorridos","1 lata de creme de leite "," sal a gosto"],"recipeInstructions":"Derreta a manteiga e refogue a cebola até ficar transparente.Junte a carne e tempere com o sal.Mexa até a carne dourar de todos os lados.Acrescente a mostarda, o catchup, a pimenta-do-reino e o tomate picado.Cozinhe até formar um molho espesso.Se necessário, adicione água quente aos poucos.Quando o molho estiver encorpado e a carne macia, adicione os cogumelos e o creme de leite.Mexa por 1 minuto e retire do fogo.Sirva imediatamente, acompanhado de arroz e batata palha.Dica:&nbsp;Se juntar água ao refogar a carne, frite-a até todo o líquido evaporar.","Review":[{"@type":"Review","author":{"@type":"Organization","name":"CLAUDIA"},"datePublished":"2008-10-24T20:47:00-0200","name":"Estrogonofe de carne","reviewRating":{"@type":"Rating","bestRating":"5","ratingValue":"5","worstRating":"1"}}],"aggregateRating":{"@type":"AggregateRating","ratingValue":4,"bestRating":5,"ratingCount":38}}"#,
        ) {
            Ok(value) => value,
            Err(error) => {
                println!("Error while parsing schema of {url}: {error}");
                panic!("fuck")
            }
        };*/

        let sel = Selector::parse(r#"script[type='application/ld+json']"#)?;
        for el in doc.select(&sel).into_iter() {
            let json = &el.inner_html();
            let recipe: RecipeSchema = match serde_json::from_str(json) {
                Ok(value) => value,
                Err(error) => {
                    println!(
                        "Error while parsing schema: {error}\nURL: {url}\nJSON: {json}\n-----"
                    );
                    continue;
                }
            };

            match recipe.at_graph {
                None => return Ok(recipe),
                Some(graph) => {
                    for temp in graph.into_iter() {
                        match temp {
                            GraphObject::Recipe(mut recipe) => {
                                recipe.at_type = Some(AtType::Recipe);
                                return Ok(recipe);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        Err(Error::DomainNotImplemented)
    }
}
