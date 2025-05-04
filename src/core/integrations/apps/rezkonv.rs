//! Parses the REZCONV file format.
//!
//! The following versions are supported:
//!     - 'Kalorio V4.03' nach REZKONV

use std::io::Read;

use nom::combinator::map;
use nom::multi::many1;
use nom::{IResult, Parser};

use crate::core::integrations::{Error, Result};
use crate::core::scraper::schema::{AtType, RecipeCategory, RecipeSchema};

struct RecipeComponents<'a> {
    software_version: &'a str,
    title: &'a str,
    category: Vec<&'a str>,
    r#yield: &'a str,
    ingredients: Vec<Ingredient<'a>>,
    instructions: Vec<Instruction<'a>>,
    keywords: Vec<&'a str>,
    author: &'a str,
    erfasst: &'a str,
}

impl From<RecipeComponents<_>> for RecipeSchema {
    fn from(r: RecipeComponents<'_>) -> Self {
        Self {
            
        }
    }
}

/// Parses the recipes in a REZKONV file.
pub fn parse<R>(mut r: R) -> Result<Vec<RecipeSchema>>
where
    R: Read,
{
    let mut content = String::new();
    r.read_to_string(&mut content)?;
    Ok(parse_recipes(&content)?
        .into_iter()
        .map(RecipeSchema::from)
        .collect())
}

fn parse_recipes(input: &str) -> Result<Vec<RecipeComponents>> {
    many1(recipe)
        .parse(input)
        .map(|(_, recipes)| recipes.into_iter().flatten().collect())
        .map_err(|err| Error::Parse(err.to_string()))
}

fn recipe(input: &str) -> IResul<&str, RecipeComponents> {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    mod tests {
        use super::*;

        use std::io::Cursor;

        type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

        #[test]
        fn test_kalorio_v4_03() -> Result<()> {
            let file = files::kalorio_v4_03();
            let buf = Cursor::new(file);

            let got = parse(buf)?;

            pretty_assertions::assert_eq!(got, results::kalorio_v4_03());
            Ok(())
        }
    }

    mod files {
        pub fn kalorio_v4_03<'a>() -> &'a str {
            r##"========== 'Kalorio V4.03' (unreg.) nach REZKONV

     Titel: Ananas-Käsekuchen
Kategorien: , , 
     Menge: 1 Kuchen

    150     Gramm  Kokoszwieback
     60     Gramm  Butter
    600     Gramm  Doppelrahmfrischkäse
    150     Gramm  Saure Sahne
    120     Gramm  Zucker
      4            Eier
      1            Limette
      1      Essl  Mehl
      1     klein. Ananas
                   - a 1 kg
      2      Essl. Rum
    200        ml  Ananassaft
      3      Essl. Vanille-Puddingpulver
      1  Spritzer  Zitrone
     20     Gramm  Kokosraspel

============================= QUELLE ================================
           essen & trinken 1/2002
           Gepostet von: Petra Holzapfel
           Überarbeitet für Kalorio
           - Erfasst am 9.2.2 von
           - Petra Holzapfel

Zwieback in der Küchenmaschine gron zerkleinern. Butter schmelzen und
unter die Brösel mischen. Den Boden einer Springform (24 cm
Durchmesser) mit Backpapier auslegen. Die Brösel als Boden darauf
verteilen und gut andrücken. Kalt stellen.

Frischkäse, saure Sahne und 100 g Zucker mit den Quirlen des
Handrührers glattrühren. Nacheinander die Eier dazugeben. Die
Limettenschale fein abreiben, die Limette auspressen. Limettensaft
und Mehl unter die Käsemasse heben. Die Masse auf dem Boden in der
Springform verteilen. Im heissen Backofen auf der 2. Einschubleiste
von
unten bei 160GradC 15 Minuten backen. Dann die Temperatur auf 150GradC
reduzieren und weitere 35 Minuten backen. Den Kuchen in der Form auf
einem Gitter auskühlen lassen und dann mindestens 2 Stunden kalt
stellen.

3. Die Ananas grosszügig schälen, vierteln und den Strunk entfernen.
Jedes Viertel längs halbieren und quer in 4 mm dicke Scheiben
schneiden. Restlichen Zucker in einem Topf mit dem Rum und Ananassaft
zum Kochen bringen. Ananasscheiben dazugeben und aufkochen.
Puddingpulver mit etwas Rum, Wasser und einem Zitronenspritzer
glattrühren
und das Kompott damitbinden. Das Kompott auskühlen lassen und die
Hälfte auf dem
Käsekuchen verteilen. Den Kuchen mit Kokosraspeln garnieren.

Das restliche Ananaskompott extra zum Kuchen servieren.

:Stichworte: Ananas
:Stichworte: Käse
:Stichworte: Kuchen
:Erfasser/Name: Petra Holzapfel
:Erfasst am: 9.2.2


=====
========== 'Kalorio V4.03' (unreg.) nach REZKONV

     Titel: Flammenkuchen von Marc
Kategorien: 
     Menge: 2 Backbleche


========================== Für den Brotteig: ========================
      6      Essl. Mehl
    1/2      Teel. ; Salz
      1     Prise  ; Pfeffer
    1/2            Würfel Hefe
    1/4      Litr. ; Wasser
                   - (lauwarm)

============================ Für den Belag ==========================
    250     Gramm  Quark
                   - 40% Fett
      1        Be  Saure Sahne
    1/2      Teel. ; Salz
      1     Prise  ; Pfeffer
      1            Zitrone

========================== Für die Garnitur =========================
      5      groß. Zwiebel
    125     Gramm  Durchwachsener Speck
    125     Gramm  Reibkäse
                   - (optional)

============================= QUELLE ================================
           Von meinem Kollegen Marc Hurstel
           - Erfasst am 8.3.1999 von
           - Jochen Herz

Zuerst den Teig vorbereiten: Mehl,Salz und Pfeffer in eine Schüssel
geben. Die Hefe im Wasser auflösen und anschließend über das Mehl
gießen, dabei mit einem Handmixer oder gleich mit der Hand kneten. Den
Teig rühren bis er nicht mehr klebt (eventuell noch etwas mehr Mehl
beimischen). 
Den Teig aus der Schüssel herrausnehmen und mit den Händen auf 
der Arbeitsplatte weiter verarbeiten, bis er schön geschmeidig wird.
Dabei immer ein bißchen Mehl auf die Platte streuen damit der Teig
nicht kleben bleibt ! Den Teig während der Zubereitung
der Sauce und Garnitur ruhen lassen.

Für die Sauce (Belag), den Quark, die saure Sahne, Salz und Pfeffer in
einer Schüssel mischen. Damit diese Sauce einen noch sauerlicheren
Geschmack bekommt, wird der Saft einer Zitrone untergerührt.

Die Zwiebeln schälen und halbieren, dann in dünne Scheiben (1 bis 2 mm
dick) schneiden. Den Speck in kleine Würfel schneiden.
Den Teigballen in 2 gleichgroße Hälften teilen. Jede Hälfte so dünn
wie möglich ausrollen (je dünner, umso schmackhafter !). Die
Teigplatten auf Backbleche mit Backpapier legen. Die Sauce dünn
auftragen. Die Zwiebeln gleichmäßig darauf verteilen. Ebenso die
Speckwürfel. Eventuell auch Käse dazu.
(Traditioneller Flammenkuchen ist ohne Käse !)

Backzeit: 20 Minuten im vorgeheizten Backofen bei 200°C (oder bis der
Teig an den Rändern goldbraun wird)
Dazu ein Kerner Spätlese halbtrocken aus der Pfalz oder ganz einfach
ein schönes elsäßisches Bier (Fischer Ambré).
Und jetzt: "A güata !" wie man bei uns sagt

:Stichworte: Frankreich
:Stichworte: Elsass
:Stichworte: Hefe
:Erfasser/Name: Jochen Herz
:Erfasst am: 8.3.1999


=====
========== 'Kalorio V4.03' (unreg.) nach REZKONV

     Titel: Möhren-Mandel-Muffins
Kategorien: , , , 
     Menge: 12 Muffins

      4            Eiweiß
      4            Eigelb
     50     Gramm  Zucker
    200     Gramm  geraspelte Möhren
    200     Gramm  gemahlene Mandeln
      1      Teel. Kirschwasser
    1/2      Teel. Zimt
    1/2            Zitrone (Schale davon)
     25     Gramm  Stärkemehl
    150     Gramm  Puderzucker
                   Zitronensaft

============================= QUELLE ================================
           - Erfasst am 18.8.2006 von
           - Jochen 'Nunz' Herz

Die Eiweiß zu festem Schnee schlagen, Eigelb und Zucker schaumig
rühren. Dann die geraspelten Möhren in die Eiermasse rühren, die
Mandeln nach und nach dazugeben. Kirschwasser, Zimt, Zitronenschale
und Stärkemehl unter die Teigmasse rühren, zum Schluss den Eischnee
vorsichtig unter die Teigmasse heben.

Muffinform mit Papierförmchen auslegen (evtl. leicht fetten) und die
Teigmasse gleichmäßig auf die zwölf Förmchen verteilen. 20 Minuten auf
mittlerer Schiene im Ofen bei 175°C abbacken (Umluft 160°C).
Puderzucker und Zitronensaft zu einem glattem Guss verrühren und die
fertigen Muffins damit bestreichen.

:Stichworte: Muffins
:Stichworte: Karotten
:Erfasser/Name: Jochen 'Nunz' Herz
:Erfasst am: 18.8.2006


=====
========== 'Kalorio V4.03' (unreg.) nach REZKONV

     Titel: Zwiebelkuchen
Kategorien: , 
     Menge: 4 Portionen


================================ Teig ===============================
    250     Gramm  Mehl
    125     Gramm  Margarine
      1     Prise  Salz
      1            Eier

=============================== Füllung =============================
      6      groß. Zwiebeln
     75     Gramm  Butter
      1        Be  Joghurt
      1        Be  Saure Sahne
                   - oder Schmand
      3            Eier
                   Kümmel
                   - gemahlen
                   Pfeffer
                   Paprika
                   - edelsüss
      1      Teel. Speisestärke

Mehl, Margarine und Salz verrühren, 1 Ei schnell unterkneten und Teig
1-2 Stunden kalt stellen.
Teig in 28er Kuchenform auswellen und hohen Rand kneten.

Zwiebel schälen und in Streifen schneiden und in 75 g Butter und
eventuell etwas Fondor eine halbe Stunde dünsten.
Kalt stellen.

Joghurt, Sahne, Eier, Speisestärke, Paprika, Salz und Pfeffer
verrühren. und die Zwiebeln untermischen.
Die Füllung auf den Teigboden und den rand herunterdrücken damit er
nicht übersteht.

Oben mit Kümmel oder Käse nach Belieben verfeindern und im
vorgeheizten Backofen bei 220 Grad 30 Minuten backen.

:Stichworte: Backen
:Stichworte: Zwiebeln
:Erfasser/Name: Jochen 'Nunz' Herz


=====
"##
        }
    }

    mod results {
        use super::*;

        pub fn kalorio_v4_03() -> Vec<RecipeSchema> {
            vec![RecipeSchema {
                at_context: Default::default(),
                at_type: ome(AtType::Recipe),
                author: to_organization_type("Petra Holzapfel".into()), 
                is_based_on: to_is_based_on("'Kalorio V4.03' nach REZKONV".into()), 
                keywords: to_defined_text(["Käse", "Kuchen"].join(",")), 
                name: Some("Ananas-Käsekuchen".into()), 
                recipe_category: RecipeCategory::Text("Ananas".into()), 
                recipe_ingredient: sections_to_vec(Sections::from([
                    ("".into(), vec![
                        "150 Gramm Kokoszwieback".into(),
                        "60 Gramm Butter".into(),
                        "600 Gramm Doppelrahmfrischkäse".into(),
                        "150 Gramm Saure Sahne".into(),
                        "120 Gramm Zucker".into(),
                        "4 Eier".into(),
                        "1 Limette".into(),
                        "1 Essl  Mehl".into(),
                        "1 klein. Ananas [- a 1 kg]".into(),
                        "2 Essl. Rum".into(),
                        "200 ml  Ananassaft".into(),
                        "3 Essl. Vanille-Puddingpulver".into(),
                        "1 Spritzer Zitrone".into(),
                        "20 Gramm Kokosraspel".into(),
                    ])
                ])),
                recipe_instructions: sections_to_itemlist(Sections::from([
                    ("".into(), vec![
                        "Zwieback in der Küchenmaschine gron zerkleinern. Butter schmelzen und unter die Brösel mischen. Den Boden einer Springform (24 cm Durchmesser) mit Backpapier auslegen. Die Brösel als Boden darauf verteilen und gut andrücken. Kalt stellen.".into(),
                        "Frischkäse, saure Sahne und 100 g Zucker mit den Quirlen des Handrührers glattrühren. Nacheinander die Eier dazugeben. Die Limettenschale fein abreiben, die Limette auspressen. Limettensaft und Mehl unter die Käsemasse heben. Die Masse auf dem Boden in der Springform verteilen. Im heissen Backofen auf der 2. Einschubleiste von unten bei 160GradC 15 Minuten backen. Dann die Temperatur auf 150GradC reduzieren und weitere 35 Minuten backen. Den Kuchen in der Form auf einem Gitter auskühlen lassen und dann mindestens 2 Stunden kalt stellen.".into(),
                        "3. Die Ananas grosszügig schälen, vierteln und den Strunk entfernen. Jedes Viertel längs halbieren und quer in 4 mm dicke Scheiben schneiden. Restlichen Zucker in einem Topf mit dem Rum und Ananassaft zum Kochen bringen. Ananasscheiben dazugeben und aufkochen. Puddingpulver mit etwas Rum, Wasser und einem Zitronenspritzer glattrühren und das Kompott damitbinden. Das Kompott auskühlen lassen und die Hälfte auf dem Käsekuchen verteilen. Den Kuchen mit Kokosraspeln garnieren.".into(),
                        "Das restliche Ananaskompott extra zum Kuchen servieren.".into(),
                    ])
                ])),
                recipe_yield: to_yield(1),
                ..Default::default()
            }, RecipeSchema {
                at_context: Default::default(),
                at_type: ome(AtType::Recipe),
                author: to_organization_type("Jochen Herz".into()), 
                is_based_on: to_is_based_on("'Kalorio V4.03' nach REZKONV".into()), 
                keywords: to_defined_text(["Elsass", "Hefe"].join(",")), 
                name: Some("Flammenkuchen von Marc".into()), 
                recipe_category: RecipeCategory::Text("Frankreich".into()), 
                recipe_ingredient: sections_to_vec(Sections::from([
                    ("Für den Brotteig:".into(), vec![
                        "6 Essl. Mehl".into(),
                        "1/2 Teel. ; Salz".into(),
                        "1 Prise  ; Pfeffer".into(),
                        "1/2 Würfel Hefe".into(),
                        "1/4 Litr. ; Wasser [(lauwarm)]".into()
                    ]),
                    ("Für den Belag".into(), vec![
                        "250 Gramm  Quark [40% Fett]".into(),
                        "1 Be  Saure Sahne".into(),
                        "1/2 Teel. ; Salz".into(),
                        "1 Prise  ; Pfeffer".into(),
                        "1 Zitrone".into(),
                    ]),
                    ("Für die Garnitur".into(), vec![
                        "5 groß. Zwiebel".into(),
                        "125 Gramm Durchwachsener Speck".into(),
                        "125 Gramm Reibkäse [(optional)]".into(),
                    ])
                ])),
                recipe_instructions: sections_to_itemlist(Sections::from([
                    ("".into(), vec![
                        "Zuerst den Teig vorbereiten: Mehl,Salz und Pfeffer in eine Schüssel geben. Die Hefe im Wasser auflösen und anschließend über das Mehl gießen, dabei mit einem Handmixer oder gleich mit der Hand kneten. Den Teig rühren bis er nicht mehr klebt (eventuell noch etwas mehr Mehl beimischen). Teig aus der Schüssel herrausnehmen und mit den Händen auf der Arbeitsplatte weiter verarbeiten, bis er schön geschmeidig wird. Dabei immer ein bißchen Mehl auf die Platte streuen damit der Teig nicht kleben bleibt ! Den Teig während der Zubereitung der Sauce und Garnitur ruhen lassen.".into(),
                        "Für die Sauce (Belag), den Quark, die saure Sahne, Salz und Pfeffer in einer Schüssel mischen. Damit diese Sauce einen noch sauerlicheren Geschmack bekommt, wird der Saft einer Zitrone untergerührt.".into(), 
                        "Die Zwiebeln schälen und halbieren, dann in dünne Scheiben (1 bis 2 mm dick) schneiden. Den Speck in kleine Würfel schneiden. Den Teigballen in 2 gleichgroße Hälften teilen. Jede Hälfte so dünn wie möglich ausrollen (je dünner, umso schmackhafter !). Die Teigplatten auf Backbleche mit Backpapier legen. Die Sauce dünn auftragen. Die Zwiebeln gleichmäßig darauf verteilen. Ebenso die Speckwürfel. Eventuell auch Käse dazu. (Traditioneller Flammenkuchen ist ohne Käse !)".into(), 
                        r#"Backzeit: 20 Minuten im vorgeheizten Backofen bei 200°C (oder bis der Teig an den Rändern goldbraun wird) Dazu ein Kerner Spätlese halbtrocken aus der Pfalz oder ganz einfach ein schönes elsäßisches Bier (Fischer Ambré). Und jetzt: "A güata !" wie man bei uns sagt"#.into(),
                    ])
                ])),
                recipe_yield: to_yield(2),
                ..Default::default()
            }, RecipeSchema {
                at_context: Default::default(),
                at_type: ome(AtType::Recipe),
                author: to_organization_type("Jochen 'Nunz' Herz".into()), 
                is_based_on: to_is_based_on("'Kalorio V4.03' nach REZKONV".into()), 
                keywords: to_defined_text("Karotten".into()), 
                name: Some("Möhren-Mandel-Muffins".into()), 
                recipe_category: RecipeCategory::Text("Muffins".into()), 
                recipe_ingredient: sections_to_vec(Sections::from([
                    ("".into(), vec![
                        "4 Eiweiß".into(),
                        "4 Eigelb".into(),
                        "50 Gramm Zucker".into(),
                        "200 Gramm geraspelte Möhren".into(),
                        "200 Gramm gemahlene Mandeln".into(),
                        "1 Teel. Kirschwasser".into(),
                        "1/2 Teel. Zimt".into(),
                        "1/2 Zitrone (Schale davon)".into(),
                        "25 Gramm Stärkemehl".into(),
                        "150 Gramm Puderzucker ".into(),
                        "Zitronensaft".into(),
                    ]),
                ])),
                recipe_instructions: sections_to_itemlist(Sections::from([
                    ("".into(), vec![
                        "Die Eiweiß zu festem Schnee schlagen, Eigelb und Zucker schaumig rühren. Dann die geraspelten Möhren in die Eiermasse rühren, die Mandeln nach und nach dazugeben. Kirschwasser, Zimt, Zitronenschale und Stärkemehl unter die Teigmasse rühren, zum Schluss den Eischnee vorsichtig unter die Teigmasse heben.".into(),
                        "Muffinform mit Papierförmchen auslegen (evtl. leicht fetten) und die Teigmasse gleichmäßig auf die zwölf Förmchen verteilen. 20 Minuten auf mittlerer Schiene im Ofen bei 175°C abbacken (Umluft 160°C). Puderzucker und Zitronensaft zu einem glattem Guss verrühren und die fertigen Muffins damit bestreichen.".into(),
                    ])
                ])),
                recipe_yield: to_yield(12),
                ..Default::default()
            }, RecipeSchema {
                at_context: Default::default(),
                at_type: ome(AtType::Recipe),
                author: to_organization_type("Jochen 'Nunz' Herz".into()), 
                is_based_on: to_is_based_on("'Kalorio V4.03' nach REZKONV".into()), 
                keywords: to_defined_text("Zwiebeln".into()), 
                name: Some("Zwiebelkuchen".into()), 
                recipe_category: RecipeCategory::Text("Backen".into()), 
                recipe_ingredient: sections_to_vec(Sections::from([
                    ("Teig".into(), vec![
                        "250 Gramm Mehl".into(),
                        "125 Gramm Margarine".into(),
                        "1 Prise Salz".into(),
                        "1 Eier".into(),
                    ]),
                    ("Füllung".into(), vec![
                        "6 groß. Zwiebeln".into(),
                        "75 Gramm Butter".into(),
                        "1 Be Joghurt".into(),
                        "1 Be Saure Sahne [oder Schmand]".into(),
                        "3 Eier".into(),
                        "Kümmel [gemahlen]".into(),
                        "Pfeffer".into(),
                        "Paprika [edelsüss]".into(),
                        "1 Teel. Speisestärke".into(),
                    ]),
                ])),
                recipe_instructions: sections_to_itemlist(Sections::from([
                    ("".into(), vec![
                        "Mehl, Margarine und Salz verrühren, 1 Ei schnell unterkneten und Teig 1-2 Stunden kalt stellen. Teig in 28er Kuchenform auswellen und hohen Rand kneten.".into(),
                        "Zwiebel schälen und in Streifen schneiden und in 75 g Butter und eventuell etwas Fondor eine halbe Stunde dünsten. Kalt stellen.".into(), 
                        "Joghurt, Sahne, Eier, Speisestärke, Paprika, Salz und Pfeffer verrühren. und die Zwiebeln untermischen. Die Füllung auf den Teigboden und den rand herunterdrücken damit er nicht übersteht.".into(), 
                        "Oben mit Kümmel oder Käse nach Belieben verfeindern und im vorgeheizten Backofen bei 220 Grad 30 Minuten backen.".into(),
                    ])
                ])),
                recipe_yield: to_yield(4),
                ..Default::default()
            }]
        }
    }
}
