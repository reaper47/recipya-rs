use std::{collections::HashMap, sync::OnceLock};

use lib_scraper::websites::Website;

pub fn websites_for_tests() -> &'static HashMap<Website, Vec<String>> {
    static INSTANCE: OnceLock<HashMap<Website, Vec<String>>> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        HashMap::from([
            (
                Website::O101cookbooksDotCom,
                vec!["https://www.101cookbooks.com/simple-bruschetta".to_string()],
            ),
            (Website::O15gramsDotCom, vec!["https://15gram.be/recepten/mac-n-cheese-met-gehakt-en-pompoen".to_string()]),
            (Website::O24kitchenDotNl, vec!["https://www.24kitchen.nl/recepten/kalkoensandwich-met-waldorfsalade".to_string()]),
            (Website::O750gDotCom, vec!["https://www.750g.com/carbonara-vegetarienne-r202631.htm".to_string()]),
            (
                Website::AbuelasCounterDotCom,
                vec!["https://abuelascounter.com/roasted-carrot-soup".to_string()],
            ),
            (
                Website::ACoupleCooksDotCom,
                vec!["https://www.acouplecooks.com/shaved-brussels-sprouts".to_string()],
            ),
            (Website::AddapinchDotCom, vec!["https://addapinch.com/easy-grape-jelly-meatballs-recipe".to_string()]),
            (
                Website::AfghanKitchenRecipesDotCom,
                vec!["http://www.afghankitchenrecipes.com/recipe/norinj-palau-rice-with-orange".to_string()],
            ),
            (
                Website::AFlavorJournalDotCom,
                vec!["https://aflavorjournal.com/charred-lemon-orzo".to_string()],
            ),
            (Website::AhDotNl, vec!["https://www.ah.nl/allerhande/recept/R-R1197438/boeuf-bourguignon-uit-de-oven-met-geroosterde-spruiten".to_string()]),
            (
                Website::AkispetretzikisDotCom,
                vec!["https://akispetretzikis.com/recipe/6867/eukolos-mpaklavas".to_string()],
            ),
            (
                Website::AlexandraCooksDotCom,
                vec!["https://alexandracooks.com/2018/08/16/very-good-bagels-easy-ish-too".to_string()],
            ),
            (
                Website::ALittleBitYummyDotCom,
                vec!["https://alittlebityummy.com/recipe/en-us/low-fodmap-blueberry-crumble-slice".to_string()],
            ),
            (Website::AllCladDotCom, vec!["https://www.all-clad.com/blog/post/kellyann-citrus-chicken".to_string()]),
            (Website::AldiDotComDotAu, vec!["https://www.aldi.com.au/recipes/breakfast-recipes/spinach-crepes-with-smoked-salmon-recipe".to_string()]),
            (
                Website::AllTheHealthyThingsDotCom,
                vec!["https://allthehealthythings.com/healthy-slow-cooker-chili".to_string()],
            ),
            (Website::AllRecipesDotCom, vec!["https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies".to_string()]),
            (Website::AltonBrownDotCom, vec!["https://altonbrown.com/recipes/the-apple-pie".to_string()]),
            (Website::AmazingRibsDotCom, vec!["https://amazingribs.com/tested-recipes-chicken-recipes-crispy-grilled-buffalo-wings-recipe".to_string()]),
            (
                Website::AmbitiousKitchenDotCom,
                vec!["https://www.ambitiouskitchen.com/lemon-garlic-salmon".to_string()],
            ),
            (
                Website::AmericasTestKitchenDotCom,
                vec!["https://www.americastestkitchen.com/recipes/7390-marbled-blueberry-bundt-cake".to_string()],
            ),
            (
                Website::AngielaEatsDotCom,
                vec!["https://www.angielaeats.com/recipes/maple-soy-brussels-sprouts".to_string()],
            ),
            (Website::AniagotujeDotPl, vec!["https://aniagotuje.pl/przepis/zupa-pomidorowa".to_string()]),
            (
                Website::AntilliaansEtenDotNl,
                vec!["https://www.antilliaans-eten.nl/recepten/antilliaanse-pikaballetjes-gehaktballetjes".to_string()],
            ),
            (
                Website::ArchanasKitchenDotCom,
                vec!["https://www.archanaskitchen.com/karnataka-style-orange-peels-curry-recipe".to_string()],
            ),
            (Website::ArgiroDotGr, vec!["https://www.argiro.gr/recipe/kotopoulo-ala-krem-me-ruzi".to_string()]),
            (Website::ArlaDotSe, vec!["https://www.arla.se/recept/kycklingpasta-med-spenat-och-grillade-gronsaker".to_string()]),
            (
                Website::AtelierDesChefsDotFr,
                vec!["https://www.atelierdeschefs.fr/fr/recette/17741-boeuf-bourguignon-traditionnel.php".to_string()],
            ),
            (
                Website::AverieCooksDotCom,
                vec!["https://www.averiecooks.com/slow-cooker-beef-stroganoff".to_string()],
            ),
            (
                Website::AvocadoSkilletDotCom,
                vec!["https://avocadoskillet.com/soba-noodles-with-crispy-tofu-and-cilantro-avocado-sauce".to_string()],
            ),
            (
                Website::BakingMischiefDotCom,
                vec!["https://bakingmischief.com/italian-roasted-potatoes".to_string()],
            ),
            (Website::BakelsDotComDotAu, vec!["https://www.bakels.com.au/recipes/damper-using-pettina-scone-mix".to_string()]),
            (
                Website::BakingSenseDotCom,
                vec!["https://www.baking-sense.com/2022/02/23/irish-potato-farls".to_string()],
            ),
            (
                Website::BarefootContessaDotCom,
                vec!["https://barefootcontessa.com/recipes/brussels-sprouts-lardons".to_string()],
            ),
            (
                Website::BareFeetInTheKitchenDotCom,
                vec!["https://barefeetinthekitchen.com/root-beer-ice-cream".to_string()],
            ),
            (Website::BbcDotCoDotUk, vec!["https://www.bbc.co.uk/food/recipes/healthy_sausage_16132".to_string()]),
            (
                Website::BbcGoodFoodDotCom,
                vec![
                    "https://www.bbcgoodfood.com/recipes/three-cheese-risotto".to_string(),
                    "https://www.bbcgoodfood.com/recipes/pan-fried-salmon".to_string(),
                ],
            ),
            (Website::BettybossiDotCh, vec!["https://www.bettybossi.ch/de/Rezept/ShowRezept/BB_KUCA130802_0054A-160-de".to_string()]),
            (
                Website::BettyCrockerDotCom,
                vec!["https://www.bettycrocker.com/recipes/spinach-mushroom-quiche/ed3014db-7810-41d6-8e1c-cd4eed7b1db3".to_string()],
            ),
            (
                Website::BeyondKimcheeDotCom,
                vec!["https://www.beyondkimchee.com/vegetables-in-coconut-milk-sayur-lemak".to_string()],
            ),
            (
                Website::BiancazaPatkaDotCom,
                vec!["https://biancazapatka.com/en/vegan-stuffed-peppers/#recipe".to_string()],
            ),
            (Website::BigOvenDotCom, vec!["https://www.bigoven.com/recipe/vegetable-tempura-japanese/19344".to_string()]),
            (
                Website::BlogDotGiallozafferanoDotIt,
                vec!["https://blog.giallozafferano.it/gingernigella/confettura-di-fragole-peperoni-e-peperoncini".to_string()],
            ),
            (Website::BlueApronDotCom, vec!["https://www.blueapron.com/recipes/sweet-spicy-pork-belly-fried-rice-with-kimchi-fried-eggs-71501763-72c6-4cc2-86d5-de2bc14d6b7b".to_string()]),
            (Website::BlueJeanchefDotCom, vec!["https://bluejeanchef.com/recipes/apple-dutch-baby-pancake".to_string()]),
            (Website::BreadtopiaDotCom, vec!["https://breadtopia.com/kubaneh-jewish-yemeni-bread".to_string()]),
            (
                Website::BrianLagerstromDotCom,
                vec![
                    "https://www.brianlagerstrom.com/recipes/faster-better-spaghetti-amp-meat-sauce".to_string(),
                    "https://www.brianlagerstrom.com/recipes/grilled-pizza".to_string(),
                ]
            ),
            (
                Website::BriceletBaklavaDotCh,
                vec!["https://briceletbaklava.ch/2023/10/taille-au-sel-de-granges-marnand.html".to_string()],
            ),
            (
                Website::BritishBakelsDotCoDotUk,
                vec!["https://www.britishbakels.co.uk/recipes/multiseed-rolls".to_string()],
            ),
            (
                Website::BodybuildingDotCom,
                vec!["https://www.bodybuilding.com/recipes/beef-teriyaki-rice-and-stir-fry".to_string()],
            ),
            (Website::BonAppetitDotCom, vec!["https://www.bonappetit.com/recipe/crispy-chicken-with-zaatar-olive-rice".to_string()]),
            (Website::BongEatsDotCom, vec!["https://www.bongeats.com/recipe/chicken-lollipop".to_string()]),
            (
                Website::BottomlessGreensDotCom,
                vec!["https://bottomlessgreens.com/complete-dinners/gluten-free-chicken-marsala".to_string()],
            ),
            (
                Website::BowlOfDeliciousDotCom,
                vec!["https://www.bowlofdelicious.com/mini-meatloaves".to_string()],
            ),
            (
                Website::BudgetBytesDotCom,
                vec!["https://www.budgetbytes.com/easy-vegetable-stir-fry".to_string()],
            ),
            (Website::CafeDelitesDotCom, vec!["https://cafedelites.com/butter-chicken".to_string()]),
            (Website::CanadaDotCa, vec!["https://food-guide.canada.ca/en/recipes/muhammara-dip-red-bell-pepper-walnut".to_string()]),
            (
                Website::CastironketoDotCom,
                vec!["https://www.castironketo.net/blog/balsamic-mushrooms-with-herbed-veggie-mash".to_string()],
            ),
            (Website::CdKitchenDotCom, vec!["https://www.cdkitchen.com/recipes/recs/285/MerleHaggardsRainbowStew65112.shtml".to_string()]),
            (
                Website::CestMaFourneeDotCom,
                vec!["https://www.cestmafournee.com/2024/02/crumble-banane-citron-vert-sauce.html".to_string()],
            ),
            (Website::ChatelaineDotCom, vec!["https://chatelaine.com/recipe/salads/mozzarella-clementine-panzanella".to_string()]),
            (Website::ChefkochDotDe, vec!["https://www.chefkoch.de/rezepte/1064631211795001/Knusprige-Ofenkartoffeln.html".to_string()]),
            (Website::ChefniniDotCom, vec!["https://www.chefnini.com/ramen-vegetarien".to_string()]),
            (Website::ChefSavvyDotCom, vec!["https://chefsavvy.com/crispy-baked-chicken-wings".to_string()]),
            (Website::ChejorgeDotCom, vec!["https://chejorge.com/2020/08/15/vegan-nasi-lemak".to_string()]),
            (
                Website::ChetnamakanDotCoDotUk,
                vec!["https://chetnamakan.co.uk/coronation-chicken-sandwich".to_string()],
            ),
            (
                Website::ChineseCookingDemystifiedDotSubstackDotCom,
                vec!["https://chinesecookingdemystified.substack.com/p/lao-gan-ma-fried-rice".to_string()],
            ),
            (
                Website::ClaudiaAbrilComBr,
                vec!["https://claudia.abril.com.br/receitas/estrogonofe-de-carne".to_string()],
            ),
            (
                Website::ClosetCookingDotCom,
                vec!["https://www.closetcooking.com/chipotle-roast-sweet-potatoes".to_string()],
            ),
            (Website::ColruytDotBe, vec!["https://www.colruyt.be/nl/lekker-koken/recept/scampi-torpedo-met-chimichurri".to_string()]),
            (
                Website::ComidinhasdoChefDotCom,
                vec!["https://comidinhasdochef.com/pudim-no-copinho-para-festa".to_string()],
            ),
            (Website::CookEatShareDotCom, vec!["https://cookeatshare.com/recipes/balinese-bbq-pork-roast-babi-guling-81003".to_string()]),
            (
                Website::CookieAndKateDotCom,
                vec!["https://cookieandkate.com/honey-butter-cornbread-recipe".to_string()],
            ),
            (Website::CookpadDotCom, vec!["https://cookpad.com/recipe/4610651".to_string()]),
            (Website::CookTalkDotCom, vec!["https://cook-talk.com/?p=5476".to_string()]),
            (Website::CoopDotSe, vec!["https://www.coop.se/recept/appelkaka-med-havregryn".to_string()]),
            (Website::CopykatDotCom, vec!["https://copykat.com/mcdonalds-egg-mcmuffin".to_string()]),
            (Website::CostcoDotCom, vec!["https://www.costco.com/connection-recipe-chicken-salad-grapes-walnuts-blue-cheese-march-2023.html".to_string()]),
            (
                Website::CountryLivingDotCom,
                vec!["https://www.countryliving.com/food-drinks/a39298988/braised-turkey-wings-recipe".to_string()],
            ),
            (
                Website::CreativeCanningDotCom,
                vec!["https://creativecanning.com/caramelized-onion-jam".to_string()],
            ),
            (Website::CucchiaioDotIt, vec!["https://www.cucchiaio.it/ricetta/baccala-in-crosta-senza-glutine".to_string()]),
            (Website::CulyDotNl, vec!["https://www.culy.nl/recepten/spinaziesalade-met-feta".to_string()]),
            (
                Website::CuisineAndTravelDotCom,
                vec!["https://www.cuisineandtravel.com/pumpkin-vegan-muffins".to_string()],
            ),
            (Website::CuisineazDotCom, vec!["https://www.cuisineaz.com/recettes/champignons-farcis-au-fromage-brie-87449.aspx".to_string()]),
            (
                Website::CyberCookDotComDotBr,
                vec!["https://cybercook.com.br/receitas/peixes-e-frutos-do-mar/receita-de-file-de-tilapia-com-batatas-82273?receita-do-dia".to_string()],
            ),
            (
                Website::DamnDeliciousDotNet,
                vec!["https://damndelicious.net/2023/04/28/crispy-baked-chicken-tacos".to_string()],
            ),
            (
                Website::DaringGourmetDotCom,
                vec!["https://www.daringgourmet.com/homemade-giardiniera".to_string()],
            ),
            (
                Website::DavidleBovitzDotCom,
                vec!["https://www.davidlebovitz.com/marcella-hazans-bolognese-sauce-recipe-italian-beef-tomato".to_string()],
            ),
            (Website::DelishDotCom, vec!["https://www.delish.com/cooking/recipe-ideas/a24489879/beef-and-broccoli-recipe".to_string()]),
            (Website::DherbsDotCom, vec!["https://www.dherbs.com/recipes/recipe/ethiopian-cabbage-dish-tikel-gomen".to_string()]),
            (
                Website::DinnerAtTheZooDotCom,
                vec!["https://www.dinneratthezoo.com/hawaiian-chicken-kabobs".to_string()],
            ),
            (
                Website::DinnerThenDessertDotCom,
                vec!["https://dinnerthendessert.com/lemon-pepper-chicken".to_string()],
            ),
            (Website::DishDotCoDotNz, vec!["https://dish.co.nz/recipes/baked-spiced-basmati-rice-and-lentil-pilaf-with-kasundi".to_string()]),
            (
                Website::DitchTheCarbsDotCom,
                vec!["https://www.ditchthecarbs.com/how-to-make-keto-samosa-air-fryer-oven".to_string()],
            ),
            (
                Website::DomesticateMeDotCom,
                vec!["https://domesticate-me.com/10-summer-cocktail-recipes".to_string()],
            ),
            (
                Website::DonnaHayDotComDotAu,
                vec!["https://www.donnahay.com.au/recipes/desserts-and-baking/apple-and-vanilla-tarte-tatin".to_string()],
            ),
            (
                Website::DownshiftologyDotCom,
                vec!["https://downshiftology.com/recipes/baked-chicken-breasts".to_string()],
            ),
            (Website::DrDotdk, vec!["https://www.dr.dk/mad/opskrift/nytarskage-med-champagne-kransekagebund-solbaer-og-chokoladepynt".to_string()]),
            (Website::DreenaburtonDotCom, vec!["https://dreenaburton.com/2-ingredient-watermelon-gelato".to_string()]),
            (Website::DrinkoteketDotSe, vec!["https://drinkoteket.se/recept/limoncello-spritz".to_string()]),
            (
                Website::EatingBirdFoodDotCom,
                vec!["https://www.eatingbirdfood.com/cinnamon-rolls".to_string()],
            ),
            (Website::EatingWellDotCom, vec!["https://www.eatingwell.com/recipe/7887715/lemon-chicken-piccata".to_string()]),
            (Website::EatLiveRunDotCom, vec!["https://www.eatliverun.com/dinner-cheap-garlickly-greens-beans".to_string()]),
            (Website::EatSmarterDotCom, vec!["https://eatsmarter.com/recipes/vietnamese-chicken-cabbage-salad".to_string()]),
            (Website::EatWell101DotCom, vec!["https://www.eatwell101.com/garlic-parmesan-marinated-mushrooms-recipe".to_string()]),
            (
                Website::EatWhatTonightDotCom,
                vec!["https://eatwhattonight.com/2021/11/diced-chicken-with-spicy-chilies-%e8%be%a3%e5%ad%90%e9%b8%a1%e4%b8%81/#wpzoom-recipe-card".to_string()],
            ),
            (Website::ElaveganDotCom, vec!["https://elavegan.com/vegan-moussaka-lentils-gluten-free".to_string()]),
            (
                Website::ElephantasticVeganDotCom,
                vec!["https://www.elephantasticvegan.com/banana-blossom-vegan-fish".to_string()],
            ),
            (
                Website::EmmikochteinfachDotDe,
                vec!["https://emmikochteinfach.de/kartoffelgratin-rezept-klassisch-und-einfach".to_string()],
            ),
            (
                Website::EntertainingWithBethDotCom,
                vec!["https://entertainingwithbeth.com/orange-pecan-crumb-muffins".to_string()],
            ),
            (Website::EpicuriousDotCom, vec!["https://www.epicurious.com/recipes/food/views/olive-oil-cake".to_string()]),
            (
                Website::ErrensKitchenDotCom,
                vec!["https://www.errenskitchen.com/baked-or-barbecued-sticky-glazed-ribs".to_string()],
            ),
            (
                Website::EtenvaneefkeDotNl,
                vec!["http://www.etenvaneefke.nl/pastasalade".to_string()],
            ),
            (
                Website::ExpressenDotSe,
                vec!["https://alltommat.expressen.se/recept/saftiga-choklad--och-apelsinbullar".to_string()],
            ),
            (
                Website::EvolvingTableDotCom,
                vec!["https://www.evolvingtable.com/lemon-orzo-pasta-salad-with-feta".to_string()],
            ),
            (
                Website::FamilyFoodOnTheTableDotCom,
                vec!["https://www.familyfoodonthetable.com/slow-cooker-honey-garlic-chicken".to_string()],
            ),
            (
                Website::FarmhouseDeliveryDotCom,
                vec!["https://recipes.farmhousedelivery.com/green-shakshuka".to_string()],
            ),
            (
                Website::FarmhouseOnBooneDotCom,
                vec!["https://www.farmhouseonboone.com/sourdough-pretzel-buns".to_string()],
            ),
            (
                Website::FattoincasadabenedettaDotIt,
                vec!["https://www.fattoincasadabenedetta.it/ricetta/lasagne-al-pistacchio".to_string()],
            ),
            (
                Website::FeastingAtHomeDotCom,
                vec!["https://www.feastingathome.com/orecchiette-pasta-with-broccoli-sauce".to_string()],
            ),
            (
                Website::FelixDotKitchen,
                vec![
                    "https://felix.kitchen/2019/12/12/haehnchenkeulen-pouletschenkel-mit-whisky-im-ofen".to_string(),
                    "https://felix.kitchen/2024/01/31/gruene-kloesse-rohe-kartoffeln-knoedel".to_string(),
                ]
            ),
            (
                Website::FifteenSpatulasDotCom,
                vec!["https://www.fifteenspatulas.com/guacamole".to_string()],
            ),
            (
                Website::FineDiningLoversDotCom,
                vec!["https://www.finedininglovers.com/recipes/main-course/szechuan-chicken".to_string()],
            ),
            (
                Website::FindingTimeForCookingDotCom,
                vec!["https://findingtimeforcooking.com/drink-recipes/orange-amaretto-white-russian-cocktail".to_string()],
            ),
            (Website::FitmenCookDotCom, vec!["https://fitmencook.com/rosemary-blue-cheese-turkey-sliders".to_string()]),
            (
                Website::FitsLowCookerQueenDotCom,
                vec!["https://fitslowcookerqueen.com/easy-homemade-breakfast-sausage".to_string()],
            ),
            (Website::FoodDotCom, vec!["https://www.food.com/recipe/jim-lahey-s-no-knead-pizza-margherita-382696".to_string()]),
            (Website::Food52DotCom, vec!["https://food52.com/recipes/7930-orecchiette-with-roasted-butternut-squash-kale-and-caramelized-red-onion".to_string()]),
            (Website::FoodalDotCom, vec!["https://foodal.com/recipes/candy/chili-chocolate-bark".to_string()]),
            (
                Website::FoodAndWineDotCom,
                vec!["https://www.foodandwine.com/recipes/garlic-salmon-with-sheet-pan-potatoes".to_string()],
            ),
            (
                Website::FoodByMariaDotCom,
                vec!["https://www.foodbymaria.com/plant-based-summer-moussaka".to_string()],
            ),
            (
                Website::FoodieCrushDotCom,
                vec!["https://www.foodiecrush.com/gnocchi-with-pomodoro-sauce".to_string()],
            ),
            (
                Website::FoodNetworkDotCoDotUk,
                vec!["https://foodnetwork.co.uk/recipes/waldorf-chicken-boats".to_string()],
            ),
            (
                Website::FoodRepublicDotCom,
                vec!["https://www.foodrepublic.com/recipes/hand-cut-burger".to_string()],
            ),
            (
                Website::FoolProofLivingDotCom,
                vec!["https://foolproofliving.com/chocolate-chili".to_string()],
            ),
            (
                Website::ForksOverKnivesDotCom,
                vec!["https://www.forksoverknives.com/recipes/vegan-snacks-appetizers/crispy-buffalo-cauliflower-bites".to_string()],
            ),
            (Website::ForkToSpoonDotCom, vec!["https://forktospoon.com/air-fryer-blooming-onion-bites".to_string()]),
            (
                Website::FrancescakooktDotNl,
                vec!["https://www.francescakookt.nl/vegetarische-rode-kool-stamppot-jus".to_string()],
            ),
            (
                Website::FranzoesischKochenDotDe,
                vec!["https://www.franzoesischkochen.de/navettes-aus-marseille".to_string()],
            ),
            (
                Website::FredriksfikaDotAllasDotSe,
                vec!["https://fredriksfika.allas.se/fredriks-pajer/knackig-appelpaj-med-rakram".to_string()],
            ),
            (Website::GastroPlantDotCom, vec!["https://gastroplant.com/vegan-kimchi-jjigae".to_string()]),
            (
                Website::GazoakleyChefDotCom,
                vec!["https://www.gazoakleychef.com/recipes/vegan-brisket".to_string()],
            ),
            (
                Website::GesundAktivDotCom,
                vec!["https://www.gesund-aktiv.com/rezepte/suppe/quitten-pastinaken-suppe".to_string()],
            ),
            (
                Website::GiallozafferanoDotCom,
                vec!["https://www.giallozafferano.com/recipes/Christmas-spice-cookies.html".to_string()],
            ),
            (
                Website::GimmeSomeOvenDotCom,
                vec!["https://www.gimmesomeoven.com/miso-chocolate-peanut-butter-cornflake-bars-gimme-some-oven".to_string()],
            ),
            (Website::GloboDotCom, vec!["https://receitas.globo.com/cheesecake-com-geleia-de-frutas-vermelhas-do-bbb-22.ghtml".to_string()]),
            (
                Website::GlutenFreeTablesDotCom,
                vec!["https://glutenfreetables.com/recipe/gluten-free-bread-loaf-bread-buns-with-schar-mix-b-flour".to_string()],
            ),
            (Website::GodtDotNo, vec!["https://www.godt.no/oppskrifter/kjoett/svin/10849/koteletter-med-paerer-i-langpanne".to_string()]),
            (
                Website::GonnaWantSecondsDotCom,
                vec!["https://www.gonnawantseconds.com/beef-tomato-macaroni-soup/#wprm-recipe-container-15941".to_string()],
            ),
            (
                Website::GoodEatingsDotCom,
                vec![
                    "https://goodeatings.com/recipes/mains/hot-pink-creamy-beetroot-pasta".to_string(),
                    "https://goodeatings.com/recipes/salads/herbed-potato-salad-w-chickpeas-and-sundried-tomato".to_string(),
                ]
            ),
            (
                Website::GoodFoodDiscoveriesDotCom,
                vec!["https://goodfooddiscoveries.com/hunters-chicken/#wpzoom-premium-recipe-card".to_string()],
            ),
            (
                Website::GoodHouseKeepingDotCom,
                vec!["https://www.goodhousekeeping.com/food-recipes/a44652479/balsamic-chicken-caprese-recipe".to_string()],
            ),
            (Website::GoodtoDotCom, vec!["https://www.goodto.com/recipes/gousto-joe-wicks-chicken-grain-bowl".to_string()]),
            (
                Website::GourmetTravellerDotComDotAu,
                vec!["https://www.gourmettraveller.com.au/recipe/chefs-recipes/fried-spaghetti-allassassina".to_string()],
            ),
            (Website::GurkiDotNo, vec!["https://gurki.no/tomatsuppe".to_string()]),
            (Website::GrandFraisDotCom, vec!["https://www.grandfrais.com/recettes/saute-de-lapin-sauce-chasseur".to_string()]),
            (
                Website::GreatBritishChefsDotCom,
                vec!["https://www.greatbritishchefs.com/recipes/babecued-miso-poussin-recipe".to_string()],
            ),
            (Website::GreeneviDotCom, vec!["https://greenevi.com/vegan-onigiri-japanese-stuffed-rice-balls".to_string()]),
            (Website::GrimGrainsDotCom, vec!["https://grimgrains.com/site/red_lentil_stew.html".to_string()]),
            (
                Website::GroupRecipesDotCom,
                vec!["http://www.grouprecipes.com/145313/sandys-green-bean-casserole.html".to_string()],
            ),
            (
                Website::HalfBakedHarvestDotCom,
                vec!["https://www.halfbakedharvest.com/louisiana-style-chicken-and-rice".to_string()],
            ),
            (
                Website::HandleTheHeatDotCom,
                vec!["https://handletheheat.com/peanut-butter-pie".to_string()],
            ),
            (Website::HassanChefDotCom, vec!["https://www.hassanchef.com/2022/10/dragon-chicken.html".to_string()]),
            (
                Website::HeadbangersKitchenDotCom,
                vec!["https://headbangerskitchen.com/recipe/keto-chicken-adobo".to_string()],
            ),
            (
                Website::HealthyLittleFoodiesDotCom,
                vec!["https://www.healthylittlefoodies.com/broccoli-tots".to_string()],
            ),
            (
                Website::HeatherChristoDotCom,
                vec!["https://heatherchristo.com/2020/05/03/13229".to_string()],
            ),
            (Website::HelloFreshDotCom, vec!["https://www.hellofresh.com/recipes/creamy-shrimp-tagliatelle-5a8f0fcbae08b52f161b5832".to_string()]),
            (
                Website::HomebrewAnswersDotCom,
                vec![
                    "https://homebrewanswers.com/banana-wine-recipe".to_string(),
                    "https://homebrewanswers.com/pomegranate-wine-recipe".to_string(),
                    "https://homebrewanswers.com/watermelon-wine-recipe".to_string(),
                ]
            ),
            (Website::HomeChefDotCom, vec!["https://www.homechef.com/meals/farmhouse-fried-chicken".to_string()]),
            (Website::HostTheToastDotCom, vec!["https://hostthetoast.com/guinness-beef-stew-with-cheddar-herb-dumplings".to_string()]),
            (Website::IcaDotSe, vec!["https://www.ica.se/recept/chicken-a-la-king-729980".to_string()]),
            (Website::ImWorthyDotCom, vec!["https://im-worthy.com/cranberry-walnut-oatmeal-energy-balls".to_string()]),
            (
                Website::InBloomBakeryDotCom,
                vec!["https://inbloombakery.com/the-best-cinnamon-rolls-ever".to_string()],
            ),
            (
                Website::IndianHealthyRecipesDotCom,
                vec!["https://www.indianhealthyrecipes.com/mango-rice-mamidikaya-pulihora".to_string()],
            ),
            (Website::InnitDotCom, vec!["https://www.innit.com/meal/504/8008/Salad%3A%20Coconut-Pineapple-Salad".to_string()]),
            (
                Website::InsanelyGoodRecipesDotCom,
                vec!["https://insanelygoodrecipes.com/chicken-cordon-bleu-casserole".to_string()],
            ),
            (Website::InspiralizedDotCom, vec!["https://inspiralized.com/vegetarian-zucchini-noodle-pad-thai".to_string()]),
            (Website::InstantPotDotCom, vec!["https://instantpot.com/blogs/recipes/cilantro-lime-rice".to_string()]),
            (
                Website::JamieOliverDotCom,
                vec![
                    "https://www.jamieoliver.com/recipes/chicken-recipes/thai-green-chicken-curry".to_string(),
                    "https://www.jamieoliver.com/recipes/pork-recipes/outrageous-pulled-pork".to_string(),
                ],
            ),
            (Website::JaimysKitchenDotNl, vec!["https://jaimyskitchen.nl/recepten/sajoer-lodeh-indonesisch-groente-recept-met-tofu".to_string()]),
            (
                Website::JarofLemonsDotCom,
                vec!["https://www.jaroflemons.com/vegetarian-hot-honey-pizza".to_string()],
            ),
            (
                Website::JimCooksFoodGoodDotCom,
                vec!["https://jimcooksfoodgood.com/recipe-weeknight-pad-thai".to_string()],
            ),
            (Website::JoCooksDotCom, vec!["https://www.jocooks.com/recipes/korean-fried-chicken".to_string()]),
            (
                Website::JoyFoodSunshineDotCom,
                vec!["https://joyfoodsunshine.com/peanut-butter-frosting".to_string()],
            ),
            (Website::JoyTheBakerDotCom, vec!["https://joythebaker.com/2023/01/jambalaya-biscuits".to_string()]),
            (
                Website::JulieGoodwinDotComDotAu,
                vec!["https://juliegoodwin.com.au/white-chocolate-and-raspberry-muffins".to_string()],
            ),
            (Website::JumboDotCom, vec!["https://www.jumbo.com/recepten/perzikenmousse-een-snel-gemakkelijk-en-heerlijk-nagerecht-1491323-7".to_string()]),
            (Website::JustATasteDotCom, vec!["https://www.justataste.com/mini-sour-cream-doughnut-muffins-recipe".to_string()]),
            (Website::JustBentoDotCom, vec!["https://justbento.com/handbook/recipe-collection-mains/sushi-roll-bento-make-sushi-rolls-without-sushi-mat".to_string()]),
            (
                Website::JustOneCookBookDotCom,
                vec!["https://www.justonecookbook.com/teriyaki-tofu-bowl".to_string()],
            ),
            (Website::KeepinItKindDotCom, vec!["https://keepinitkind.com/jackfruit-vietnamese-summer-rolls-with-hoisin-peanut-sauce".to_string()]),
            (
                Website::KennyMcGovernDotCom,
                vec!["https://kennymcgovern.com/chicken-noodle-soup".to_string()],
            ),
            (
                Website::KingArthurBakingDotCom,
                vec![
                    "https://www.kingarthurbaking.com/recipes/sourdough-zucchini-bread-recipe".to_string(),
                    "https://www.kingarthurbaking.com/recipes/fudge-brownies-recipe".to_string(),
                ],
            ),
            (
                Website::KitchenAidDotComDotAu,
                vec!["https://kitchenaid.com.au/blogs/kitchenthusiast/hot-cross-buns".to_string()],
            ),
            (
                Website::KitchenSanctuaryDotCom,
                vec!["https://www.kitchensanctuary.com/air-fryer-crispy-chicken-wings".to_string()],
            ),
            (
                Website::KitchenStoriesDotCom,
                vec!["https://www.kitchenstories.com/de/rezepte/valencianische-paella".to_string()],
            ),
            (Website::KochbarDotDe, vec!["https://www.kochbar.de/rezept/465773/Spargelsalat-Fruchtig.html".to_string()]),
            (Website::KochbucherDotCom, vec!["https://kochbucher.com/eierlikor-pralinen".to_string()]),
            (Website::KoketDotSe, vec!["https://www.koket.se/mitt-kok/tommy-myllymaki/myllymakis-toast-skagen".to_string()]),
            (Website::KookjijDotNl, vec!["https://www.kookjij.nl/recepten/daging-smoor-heerlijk-gestoofd-indisch-rundvlees-ook-voor-slowcooker".to_string()]),
            (Website::KptnCookDotCom, vec!["https://mobile.kptncook.com/recipe/pinterest/empanadas-mit-wuerziger-tomaten-salsa/3f1e5736".to_string()]),
            (
                Website::KristinesKitchenBlogDotCom,
                vec!["https://kristineskitchenblog.com/blackberry-pie".to_string()],
            ),
            (
                Website::KuchniaDomowaDotPl,
                vec!["https://www.kuchnia-domowa.pl/przepisy/dodatki-do-dan/548-mizeria".to_string()],
            ),
            (Website::KuchynalidlaDotSk, vec!["https://kuchynalidla.sk/recepty/bravcova-rolada-so-syrom-a-sunkou".to_string()]),
            (
                Website::KwestiasmakuDotCom,
                vec!["https://www.kwestiasmaku.com/przepis/muffiny-czekoladowe-z-maslem-orzechowym".to_string()],
            ),
            (Website::LahbcoDotCom, vec!["https://www.lahbco.com/greens-n-things/berryburratasalad".to_string()]),
            (
                Website::LatelierDeRoxaneDotCom,
                vec!["https://www.latelierderoxane.com/blog/recette-cake-marbre".to_string()],
            ),
            (
                Website::LeanandGreenRecipesDotNet,
                vec!["https://leanandgreenrecipes.net/recipes/italian/spaghetti-squash-lasagna".to_string()],
            ),
            (Website::LeckerDotDe, vec!["https://www.lecker.de/gemuesepfanne-mit-haehnchen-zuckerschoten-und-brokkoli-79685.html".to_string()]),
            (
                Website::LecremedelacrumbDotCom,
                vec!["https://www.lecremedelacrumb.com/instant-pot-pot-roast-potatoes".to_string()],
            ),
            (
                Website::LekkerenSimpelDotCom,
                vec!["https://www.lekkerensimpel.com/gougeres".to_string()],
            ),
            (
                Website::LeukeReceptenDotNl,
                vec!["https://www.leukerecepten.nl/recepten/pita-tandoori".to_string()],
            ),
            (
                Website::LifestyleOfAFoodieDotCom,
                vec!["https://lifestyleofafoodie.com/chick-fil-a-peppermint-milkshake".to_string()],
            ),
            (Website::LidlDotNl, vec!["https://recepten.lidl.nl/recept/prikker-met-watermeloen-rode-ui-olijven-en-feta".to_string()]),
            (Website::LidlKochenDotDe, vec!["https://www.lidl-kochen.de/rezeptwelt/schweinemedaillons-mit-ofenkartoffeln-butterbohnen-und-rosmarinbroeseln-147914".to_string()]),
            (
                Website::LithuanianInTheUsaDotCom,
                vec!["https://lithuanianintheusa.com/2017/05/14/curd-cheese-cookies-with-applesvarskes-skareles-su-obuoliais".to_string()],
            ),
            (
                Website::LittleSpiceJarDotCom,
                vec!["https://littlespicejar.com/starbucks-pumpkin-loaf".to_string()],
            ),
            (Website::LivelyTableDotCom, vec!["https://livelytable.com/bbq-ribs-on-the-charcoal-grill".to_string()]),
            (
                Website::LivingTheGreenLifeDotCom,
                vec!["https://livingthegreenlife.com/recepten/vegan-tikka-masala-met-rijst".to_string()],
            ),
            (
                Website::LoveAndLemonsDotCom,
                vec!["https://www.loveandlemons.com/cucumber-mango-miso-noodle-bowls".to_string()],
            ),
            (
                Website::LovingItVeganDotCom,
                vec!["https://lovingitvegan.com/vegan-buffalo-chicken-dip".to_string()],
            ),
            (
                Website::MaangchiDotCom,
                vec![
                    "https://www.maangchi.com/recipe/tteokbokki".to_string(),
                    "https://www.maangchi.com/recipe/dwaejigogi-bokkeum".to_string(),
                    "https://www.maangchi.com/recipe/tongbaechu-kimchi".to_string(),
                ]
            ),
            (Website::MadensVerdenDotdk, vec!["https://madensverden.dk/durumboller-nemme-italienske-boller-med-durum-mel".to_string()]),
            (Website::MadsvinDotCom, vec!["https://madsvin.com/droemmekage".to_string()]),
            (Website::MarmitonDotOrg, vec!["https://www.marmiton.org/recettes/recette_cake-sale-chevre-pesto_35665.aspx".to_string()]),
            (
                Website::MarthaStewartDotCom,
                vec!["https://www.marthastewart.com/1539828/lemon-glazed-sheet-cake".to_string()],
            ),
            (Website::MatpratDotNo, vec!["https://www.matprat.no/oppskrifter/tradisjon/vafler".to_string()]),
            (Website::McCormickDotCom, vec!["https://www.mccormick.com/recipes/main-dishes/mexican-chicken-and-rice".to_string()]),
            (Website::MeljoulwanDotCom, vec!["https://meljoulwan.com/2019/06/10/thai-chicken-meatballs".to_string()]),
            (
                Website::MelsKitchenCafeDotCom,
                vec!["https://www.melskitchencafe.com/grilled-rosemary-ranch-chicken".to_string()],
            ),
            (
                Website::MexicanMadeMeatlessDotCom,
                vec!["https://mexicanmademeatless.com/verdolagas-purslane-in-salsa-verde/#recipe".to_string()],
            ),
            (Website::MindmegetteDotHu, vec!["https://www.mindmegette.hu/karamellas-lavasuti.recept".to_string()]),
            (
                Website::MinimalistBakerDotCom,
                vec!["https://minimalistbaker.com/adaptogenic-hot-chocolate-mix".to_string()],
            ),
            (
                Website::MinistryOfCurryDotCom,
                vec!["https://ministryofcurry.com/cranberry-sauce-recipe".to_string()],
            ),
            (Website::MisyaDotInfo, vec!["https://www.misya.info/ricetta/grigliata-di-carne.htm".to_string()]),
            (
                Website::ModernHoneyDotCom,
                vec!["https://www.modernhoney.com/fettuccine-alfredo".to_string()],
            ),
            (
                Website::MomonTimeoutDotCom,
                vec!["https://www.momontimeout.com/bacon-wrapped-chicken-breast-recipe".to_string()],
            ),
            (Website::MomsDishDotCom, vec!["https://momsdish.com/khinkali".to_string()]),
            (
                Website::MomsWithCrockpotsDotCom,
                vec!["https://momswithcrockpots.com/crockpot-cornbread".to_string()],
            ),
            (
                Website::MotherThymeDotCom,
                vec!["https://www.motherthyme.com/2018/06/blt-pasta-salad.html".to_string()],
            ),
            (Website::MoulinexDotFr, vec!["https://www.moulinex.fr/recette/detail/PRO/B%C5%93uf%20carottes%20fondant/249426".to_string()]),
            (
                Website::MundoDeReceitasBimbyDotComDotPt,
                vec!["https://www.mundodereceitasbimby.com.pt/entradas-receitas/batatas-com-chourico/i1ggy3yl-6f492-260374-cfcd2-gzaqtn4i".to_string()],
            ),
            (
                Website::MyBakingAddictionDotCom,
                vec!["https://www.mybakingaddiction.com/pistachio-pudding-cake".to_string()],
            ),
            (
                Website::MyJewishLearningDotCom,
                vec!["https://www.myjewishlearning.com/recipe/challah-recipe".to_string()],
            ),
            (Website::MyKitchen101DotCom, vec!["https://mykitchen101.com/%e5%8e%9f%e5%91%b3%e7%89%9b%e6%b2%b9%e8%9b%8b%e7%b3%95".to_string()]),
            (
                Website::MyKoreanKitchenDotCom,
                vec!["https://mykoreankitchen.com/mushroom-rice-bowl".to_string()],
            ),
            (
                Website::MyGingerGarlicKitchenDotCom,
                vec!["https://www.mygingergarlickitchen.com/tawa-paneer".to_string()],
            ),
            (
                Website::MyKitchen101enDotCom,
                vec!["https://mykitchen101en.com/plain-butter-cake".to_string()],
            ),
            (Website::MyPlateDotGov, vec!["https://www.myplate.gov/recipes/supplemental-nutrition-assistance-program-snap/20-minute-chicken-creole".to_string()]),
            (Website::MyRecipesDotCom, vec!["https://www.myrecipes.com/recipe/quick-easy-nachos".to_string()]),
            (
                Website::NatashasKitchenDotCom,
                vec!["https://natashaskitchen.com/taco-seasoning-recipe".to_string()],
            ),
            (Website::NigellaDotCom, vec!["https://www.nigella.com/recipes/moonblush-tomatoes".to_string(), "https://www.nigella.com/recipes/guests/coffee-baklava-with-dried-figs".to_string()]),
            (
                Website::NinjaTestKitchenDotEu,
                vec!["https://ninjatestkitchen.eu/recipe/dirt-worm-brownies".to_string()],
            ),
            (
                Website::NourishedByNutritionDotCom,
                vec!["https://nourishedbynutrition.com/fudgy-gluten-free-tahini-brownies".to_string()],
            ),
            (Website::NosaltyDotHu, vec!["https://www.nosalty.hu/recept/kelt-pizzateszta".to_string()]),
            (
                Website::NotEnoughCinnamonDotCom,
                vec!["https://www.notenoughcinnamon.com/easy-chicken-shawarma-bowl".to_string()],
            ),
            (Website::NrkDotNo, vec!["https://www.nrk.no/mat/japansk-omelett-_tamagoyaki_-1.16435297".to_string()]),
            (
                Website::Number2PencilDotCom,
                vec!["https://www.number-2-pencil.com/creamy-one-pot-pumpkin-alfredo".to_string()],
            ),
            (
                Website::NutritionFactsDotOrg,
                vec!["https://nutritionfacts.org/recipe/cinnamon-roll-oatmeal".to_string()],
            ),
            (Website::NyTimesDotCom, vec!["https://cooking.nytimes.com/recipes/8357-spaghetti-with-fried-eggs".to_string()]),
            (Website::OhSheGlowsDotCom, vec!["https://ohsheglows.com/2017/11/23/bread-free-stuffing-balls".to_string()]),
            (
                Website::OkokoReceptenDotNl,
                vec!["https://www.okokorecepten.nl/recept/vlees/chili-con-carne/chili-con-carne-amuse".to_string()],
            ),
            (
                Website::OmnivoresCookbookDotCom,
                vec!["https://omnivorescookbook.com/chinese-scallion-pancakes".to_string()],
            ),
            (
                Website::OnceUponaChefDotCom,
                vec!["https://www.onceuponachef.com/recipes/perfect-basmati-rice.html".to_string()],
            ),
            (Website::OhMyVeggiesDotCom, vec!["https://ohmyveggies.com/korean-barbecue-jackfruit-sandwiches".to_string()]),
            (
                Website::OneSweetAppetiteDotCom,
                vec!["https://onesweetappetite.com/ranch-chicken".to_string()],
            ),
            (Website::OwenHanDotCom, vec!["https://www.owen-han.com/recipes/cobb-chicken-sandwich".to_string()]),
            (
                Website::PaleoRunningMommaDotCom,
                vec!["https://www.paleorunningmomma.com/grain-free-peanut-butter-granola-bars-vegan-paleo-option".to_string()],
            ),
            (
                Website::PanelinhaDotComDotBr,
                vec!["https://www.panelinha.com.br/receita/Frango-ao-curry".to_string()],
            ),
            (Website::PaniniHappyDotCom, vec!["https://paninihappy.com/why-you-need-this-pumpkin-muffin-recipe".to_string()]),
            (
                Website::ParsleyAndParmDotCom,
                vec!["https://parsleyandparm.com/zaatar-sheet-pan-vegetables-and-eggs".to_string()],
            ),
            (
                Website::PersnicketyPlatesDotCom,
                vec!["https://www.persnicketyplates.com/easy-slow-cooker-french-toast-casserole".to_string()],
            ),
            (
                Website::PickupLimesDotCom,
                vec!["https://www.pickuplimes.com/recipe/the-best-vegan-chow-mein-800".to_string()],
            ),
            (Website::PinchOfYumDotCom, vec!["https://pinchofyum.com/the-best-soft-chocolate-chip-cookies".to_string()]),
            (
                Website::PinkOwlKitchenDotCom,
                vec!["https://pinkowlkitchen.com/cajun-dirty-rice-with-smoked-sausage".to_string()],
            ),
            (Website::PingoDoceDotPt, vec!["https://www.pingodoce.pt/receitas/tarte-de-alho-frances-caramelizado".to_string()]),
            (
                Website::PlatingPixelsDotCom,
                vec!["https://www.platingpixels.com/mushroom-tart-recipe".to_string()],
            ),
            (Website::PlentyVeganDotCom, vec!["https://plentyvegan.com/almond-chickpea-chocolate-chip-cookies".to_string()]),
            (Website::PloetzblogDotDe, vec!["https://www.ploetzblog.de/rezepte/hartweizenbrot/id=61fbd728a672573b7a4f6e3e".to_string()]),
            (
                Website::PlowingThroughLifeDotCom,
                vec!["https://plowingthroughlife.com/the-best-rich-and-moist-chocolate-cake".to_string()],
            ),
            (
                Website::PopSugarDotCoDotUk,
                vec!["https://www.popsugar.co.uk/food/cinnamon-butter-baked-carrot-recipe-46882533".to_string()],
            ),
            (Website::PotatoRollsDotGom, vec!["https://potatorolls.com/recipes/crab-mac-and-cheese-hot-dog".to_string()]),
            (
                Website::PracticalSelfRelianceDotCom,
                vec!["https://practicalselfreliance.com/zucchini-relish".to_string()],
            ),
            (
                Website::PressureLuckCookingDotCom,
                vec!["https://pressureluckcooking.com/spanish-omelette-scramble".to_string()],
            ),
            (
                Website::PrimalEdgeHealthDotCom,
                vec!["https://www.primaledgehealth.com/slow-cooker-crack-chicken".to_string()],
            ),
            (
                Website::ProjectGezondDotNl,
                vec!["https://www.projectgezond.nl/italiaanse-kiprollade-met-gremolata".to_string()],
            ),
            (Website::PrzepisyDotPl, vec!["https://www.przepisy.pl/przepis/placki-ziemniaczane".to_string()]),
            (Website::PurelyPopeDotCom, vec!["https://purelypope.com/sweet-chili-brussel-sprouts".to_string()]),
            (Website::PureWowDotCom, vec!["https://www.purewow.com/recipes/corn-fritter-caprese-peaches-tomatoes".to_string()]),
            (
                Website::PurpleCarrotDotCom,
                vec!["https://www.purplecarrot.com/recipe/gnocchi-al-pesto-with-charred-green-beans-lemon-zucchini-bc225f0b-1985-4d94-b05b-a78de295b2da?plan=chefs_choice".to_string()],
            ),
            (Website::PuurgezondDotNl, vec!["https://www.puurgezond.nl/eten/sperziebonen/recepten/zalm-met-sperziebonen-en-cashew-dillesaus".to_string()]),
            (Website::QuitoqueDotFr, vec!["https://www.quitoque.fr/recette/12976/saumon-teriyaki-et-riz-rouge".to_string()]),
            (
                Website::RachlmansFieldDotCom,
                vec!["https://rachlmansfield.com/delicious-crispy-rice-salad-gluten-free".to_string()],
            ),
            (Website::RadioFranceDotFr, vec!["https://www.radiofrance.fr/franceinter/recette-sauce-cacahuete-tomates-et-piments-3598484".to_string()]),
            (
                Website::RainbowPlantLifeDotCom,
                vec!["https://rainbowplantlife.com/livornese-stewed-beans".to_string()],
            ),
            (Website::RealSimpleDotCom, vec!["https://www.realsimple.com/food-recipes/browse-all-recipes/sheet-pan-chicken-and-sweet-potatoes".to_string()]),
            (
                Website::ReceitasNestleDotComDotBr,
                vec!["https://www.receitasnestle.com.br/receitas/pave-de-pessego".to_string()],
            ),
            (
                Website::RecettesDotQcDotCa,
                vec!["https://www.recettes.qc.ca/recettes/recette/yakisoba-nouille-sautees-a-la-japonaise".to_string()],
            ),
            (Website::RecipeGirlDotCom, vec!["https://www.recipegirl.com/chili-rubbed-pork-chops-with-grilled-pineapple-salsa".to_string()]),
            (Website::RecipeRunnerDotCom, vec!["https://reciperunner.com/cranberry-apple-sauce".to_string()]),
            (
                Website::RecipeTinEatsDotCom,
                vec!["https://www.recipetineats.com/chicken-sharwama-middle-eastern".to_string()],
            ),
            (Website::RedditDotCom, vec!["https://old.reddit.com/r/recipes/comments/1bhr8se/spicy_chilli_garlic_prawn_linguine_pasta".to_string()]),
            (
                Website::RedhouseSpiceDotCom,
                vec!["https://redhousespice.com/pork-fried-rice/".to_string()],
            ),
            (Website::ReisHungerDotDe, vec!["https://www.reishunger.de/rezepte/rezept/440/chicken-tikka-masala".to_string()]),
            (Website::RezeptWeltDotDe, vec!["https://www.rezeptwelt.de/vorspeisensalate-rezepte/haehnchen-nuggets/y3duba6e-e2d56-608317-cfcd2-vjez4wd6".to_string()]),
            (Website::RicettaDotIt, vec!["https://ricetta.it/pan-d-arancio".to_string()]),
            (
                Website::RicettePerBimbyDotIt,
                vec!["https://www.ricetteperbimby.it/ricette/dolcetti-mandorle-e-limone-bimby".to_string()],
            ),
            (Website::RobinasBellDotCom, vec!["https://robinasbell.com/2019/08/make-a-pizza-with-edible-flowers-its-like-eating-summer/".to_string()]),
            (
                Website::RosannaPansinoDotCom,
                vec!["https://rosannapansino.com/blogs/recipes/rainbow-treats".to_string()],
            ),
            (Website::RutgerbaktDotNl, vec!["https://rutgerbakt.nl/basisrecepten/oreo-topping-van-roomkaas/".to_string()]),
            (
                Website::RecipeCommunityDotComDotAu,
                vec!["https://www.recipecommunity.com.au/baking-sweet-recipes/flourless-refined-sugar-free-chocolate-cake/1te0mta9-5d0d3-705689-cfcd2-7zd1b4nd".to_string()],
            ),
            (
                Website::SaboresaJinomotoDotComDotBr,
                vec!["https://www.saboresajinomoto.com.br/receita/pizza-de-pao-amanhecido".to_string()],
            ),
            (
                Website::SallysBakingAddictionDotCom,
                vec!["https://sallysbakingaddiction.com/breakfast-pastries/".to_string()],
            ),
            (Website::SallysBlogDotDe, vec!["https://sallys-blog.de/rezepte/zwieback-dessert-etimek-tatlisi-no-bake".to_string()]),
            (
                Website::SaltAndLavenderDotCom,
                vec!["https://www.saltandlavender.com/creamy-garlic-chicken/".to_string()],
            ),
            (
                Website::SaltPepperSkilletDotCom,
                vec!["https://saltpepperskillet.com/creamy-mashed-potatoes/".to_string()],
            ),
            (
                Website::SarahsVeganGuideDotCom,
                vec!["https://sarahsveganguide.com/vegan-sushi-guide".to_string()],
            ),
            (Website::SaveurDotCom, vec!["https://www.saveur.com/recipes/varenyky-pierogi-recipe/".to_string()]),
            (
                Website::SavoryNothingsDotCom,
                vec!["https://www.savorynothings.com/whole-wheat-cinnamon-crunch-banana-bread/".to_string()],
            ),
            (
                Website::SeriousEatsDotCom,
                vec!["https://www.seriouseats.com/miyeok-guk-korean-seaweed-and-brisket-soup".to_string()],
            ),
            (
                Website::SimpleVeganistaDotCom,
                vec!["https://simple-veganista.com/blackberry-cobbler/".to_string()],
            ),
            (
                Website::SimplyCookitDotCom,
                vec!["https://www.simply-cookit.com/de/rezepte/paprikagulasch".to_string()],
            ),
            (
                Website::SimplyQuinoaDotCom,
                vec!["https://www.simplyquinoa.com/spicy-kimchi-quinoa-bowls/".to_string()],
            ),
            (
                Website::SimplyRecipesDotCom,
                vec!["https://www.simplyrecipes.com/recipes/chicken_tikka_masala/".to_string()],
            ),
            (
                Website::SimplyWhiskedDotCom,
                vec!["https://www.simplywhisked.com/dill-pickle-pasta-salad/".to_string()],
            ),
            (
                Website::SkinnyTasteDotCom,
                vec!["https://www.skinnytaste.com/air-fryer-steak/".to_string()],
            ),
            (
                Website::SmittenKitchenDotCom,
                vec![
                    "https://smittenkitchen.com/2024/04/new-york-crumb-cake".to_string(),
                    "https://smittenkitchen.com/2009/05/slaw-tartare".to_string(),
                    "https://smittenkitchen.com/2008/12/sausage-stuffed-potatoes-a-green-salad".to_string()
                ]
            ),
            (Website::SoborsDotHu, vec!["https://sobors.hu/receptek/karamelles-sajttorta-poharkrem-recept/".to_string()]),
            (
                Website::SouthernCastIronDotCom,
                vec!["https://southerncastiron.com/creamy-turkey-and-wild-rice-soup/".to_string()],
            ),
            (
                Website::SouthernLivingDotCom,
                vec!["https://www.southernliving.com/recipes/oven-roasted-corn-on-cob".to_string()],
            ),
            (
                Website::SpendWithPenniesDotCom,
                vec!["https://www.spendwithpennies.com/split-pea-soup/".to_string()],
            ),
            (
                Website::SpiceboxTravelsDotCom,
                vec!["https://spiceboxtravels.com/2013/12/13/ottolenghi-style-eggplant-with-pomegranate/".to_string()],
            ),
            (
                Website::StaySnatchedDotCom,
                vec!["https://www.staysnatched.com/seafood-dressing".to_string()],
            ),
            (
                Website::SteamyKitchenDotCom,
                vec!["https://steamykitchen.com/4474-korean-style-tacos-with-kogi-bbq-sauce.html".to_string()],
            ),
            (Website::StreetKitchenDotCo, vec!["https://streetkitchen.co/recipe/thai-red-duck-curry/".to_string()]),
            (
                Website::StrongrFastrDotCom,
                vec!["https://www.strongrfastr.com/recipes/96-latininspired_creamy_chicken_stew".to_string()],
            ),
            (Website::SunBasketDotCom, vec!["https://sunbasket.com/recipe/chicken-and-dumplings".to_string()]),
            (Website::SundPaabudgetDotdk, vec!["https://sundpaabudget.dk/shawarma-bowl".to_string()]),
            (Website::SunsetDotCom, vec!["https://www.sunset.com/recipe/veggie-chili".to_string()]),
            (
                Website::SweetcsDesignsDotCom,
                vec!["https://sweetcsdesigns.com/roasted-tomato-marinara-sauce/".to_string()],
            ),
            (
                Website::SweetPeasAnSsaffronDotCom,
                vec!["https://sweetpeasandsaffron.com/slow-cooker-cilantro-lime-chicken-tacos-freezer-slow-cooker/".to_string()],
            ),
            (Website::TasteAtlasDotCom, vec!["https://www.tasteatlas.com/pastel-de-nata/recipe".to_string()]),
            (
                Website::TasteOfHomeDotCom,
                vec!["https://www.tasteofhome.com/recipes/cast-iron-skillet-steak/".to_string()],
            ),
            (
                Website::TastesBetterFromScratchDotCom,
                vec!["https://tastesbetterfromscratch.com/apple-crisp".to_string()],
            ),
            (
                Website::TastesOfLizzytDotCom,
                vec!["https://www.tastesoflizzyt.com/easter-ham-pie/".to_string()],
            ),
            (Website::TastyDotCo, vec!["https://tasty.co/recipe/honey-soy-glazed-salmon".to_string()]),
            (Website::TastyKitchenDotCom, vec!["https://tastykitchen.com/recipes/main-courses/garlic-shrimp-scampi-with-angel-hair-pasta/".to_string()]),
            (Website::TescoDotCom, vec!["https://realfood.tesco.com/recipes/salted-honey-and-rosemary-lamb-with-roasties-and-rainbow-carrots.html".to_string()]),
            (
                Website::ThatVeganDadDotNet,
                vec![
                    "https://www.thatvegandad.net/small-plate/vegan-poutine-with-almond-curds".to_string(),
                    "https://www.thatvegandad.net/main-meals/vegan-creamy-roasted-red-capsicum-pasta/#1".to_string(),
                ]
            ),
            (
                Website::TheCleverCarrotDotCom,
                vec!["https://www.thecookierookie.com/blt-guacamole/".to_string()],
            ),
            (
                Website::TheCookieRookieDotCom,
                vec!["https://www.thecookierookie.com/blt-guacamole/".to_string()],
            ),
            (
                Website::TheCookingGuyDotCom,
                vec!["https://www.thecookingguy.com/recipes/funeral-sandwiches".to_string()],
            ),
            (
                Website::TheExpertGuidesDotCom,
                vec!["https://theexpertguides.com/recipes/is-guacamole-vegan/".to_string()],
            ),
            (
                Website::TheFoodFlamingoDotCom,
                vec!["https://thefoodflamingo.com/pistachio-cardamom-tres-leches-cake/".to_string()],
            ),
            (Website::TheGucchaDotCom, vec!["https://www.theguccha.com/4-ingredient-chia-pudding/".to_string()]),
            (
                Website::TheHappyFoodieDotCoDotUk,
                vec!["https://thehappyfoodie.co.uk/recipes/leek-and-lentil-gratin/".to_string()],
            ),
            (
                Website::TheHeartySoulDotCom,
                vec!["https://theheartysoul.com/ginger-sweet-potato-and-coconut-milk-stew/".to_string()],
            ),
            (
                Website::TheKitchenCommunityDotOrg,
                vec!["https://thekitchencommunity.org/turkey-salad-recipe/".to_string()],
            ),
            (
                Website::TheKitchenMagPieDotCom,
                vec!["https://www.thekitchenmagpie.com/blt-pasta-salad/".to_string()],
            ),
            (Website::TheKitchnDotCom, vec!["https://www.thekitchn.com/how-to-reheat-turkey-and-keep-it-moist-251033".to_string()]),
            (
                Website::TheMagicalSlowCookerDotCom,
                vec!["https://www.themagicalslowcooker.com/slow-cooker-apple-cider-pot-roast/".to_string()],
            ),
            (
                Website::TheNutritiousKitchenDotCo,
                vec!["http://thenutritiouskitchen.co/fluffy-paleo-blueberry-pancakes/".to_string()],
            ),
            (
                Website::TheModernProperDotCom,
                vec!["https://themodernproper.com/turkey-pozole-rojo".to_string()],
            ),
            (
                Website::ThePalatableLifeDotCom,
                vec!["https://www.thepalatablelife.com/cinnamon-toast-crunch-cookies-2".to_string()],
            ),
            (
                Website::ThePioneerWomanDotCom,
                vec!["https://www.thepioneerwoman.com/food-cooking/recipes/a8865/eggs-benedict/".to_string()],
            ),
            (
                Website::TheRecipeCriticDotCom,
                vec!["https://therecipecritic.com/avocado-egg-rolls/".to_string()],
            ),
            (
                Website::TheSaltyMarshmallowDotCom,
                vec!["https://thesaltymarshmallow.com/best-banana-bread-recipe/".to_string()],
            ),
            (
                Website::TheSpruceEatsDotCom,
                vec!["https://www.thespruceeats.com/pasta-with-anchovies-and-breadcrumbs-recipe-5215384".to_string()],
            ),
            (
                Website::TheVintageMixerDotCom,
                vec!["https://www.thevintagemixer.com/roasted-asparagus-grilled-cheese/".to_string()],
            ),
            (
                Website::TheWoksOfLifeDotCom,
                vec!["https://thewoksoflife.com/fried-wontons/".to_string()],
            ),
            (Website::ThinliciousDotCom, vec!["https://thinlicious.com/low-carb-greek-yogurt/".to_string()]),
            (Website::TidyMomDotNet, vec!["https://tidymom.net/make-ahead-mashed-potatoes/".to_string()]),
            (
                Website::TimesOfIndiaDotCom,
                vec!["https://recipes.timesofindia.com/recipes/beetroot-cold-soup/rs90713582.cms".to_string()],
            ),
            (Website::TineDotNo, vec!["https://www.tine.no/oppskrifter/middag-og-hovedretter/kylling-og-fjarkre/rask-kylling-tikka-masala".to_string()]),
            (
                Website::TudogostosoDotComDotBr,
                vec!["https://www.tudogostoso.com.br/receita/585-rocambole-de-carne-moida.html".to_string()],
            ),
            (
                Website::TwoPeasAndTheirPodDotCom,
                vec!["https://www.twopeasandtheirpod.com/easy-chickpea-salad/".to_string()],
            ),
            (Website::TwoSleeversDotCom, vec!["https://twosleevers.com/pressure-cooker-aloo-gobi/".to_string()]),
            (
                Website::UitPaulinesKeukenDotNl,
                vec!["https://uitpaulineskeuken.nl/recept/breekbrood-met-kaasfondue".to_string()],
            ),
            (
                Website::UnsophistiCookDotCom,
                vec!["https://unsophisticook.com/oven-roasted-baby-potatoes/".to_string()],
            ),
            (Website::UsaPearsDotOrg, vec!["https://usapears.org/recipe/pear-vinegar/".to_string()]),
            (Website::ValdemarsroDotdk, vec!["https://www.valdemarsro.dk/butter_chicken/".to_string()]),
            (
                Website::VanillaAndBeanDotCom,
                vec!["https://vanillaandbean.com/carrot-cake-bread/".to_string()],
            ),
            (Website::VeganPratiqueDotFr, vec!["https://vegan-pratique.fr/recettes/banana-bread/".to_string()]),
            (
                Website::VegetarBloggenDotNo,
                vec!["https://www.vegetarbloggen.no/2023/07/15/peanottkake/".to_string()],
            ),
            (Website::VegolosiDotIt, vec!["https://www.vegolosi.it/ricette-vegane/pancake-vegani-senza-glutine-alla-quinoa-e-cocco/".to_string()]),
            (
                Website::VegRecipesOfIndiaDotCom,
                vec!["https://www.vegrecipesofindia.com/paneer-butter-masala/".to_string()],
            ),
            (Website::WaitRoseDotCom, vec!["https://www.waitrose.com/ecom/recipe/the-best-macaroni-cheese".to_string()]),
            (
                Website::WatchWhatUEatDotCom,
                vec!["https://www.watchwhatueat.com/healthy-fried-brown-rice/".to_string()],
            ),
            (
                Website::WearenotMarthaDotCom,
                vec!["https://wearenotmartha.com/western-omelet/".to_string()],
            ),
            (
                Website::WeightWatchersDotCom,
                vec!["https://www.weightwatchers.com/us/recipe/pepperoni-flatbread-pizza/646d12d61a843705da13cb7f".to_string()],
            ),
            (Website::WellPlatedDotCom, vec!["https://www.wellplated.com/energy-balls/".to_string()]),
            (
                Website::WhatsGabyCookingDotCom,
                vec!["https://whatsgabycooking.com/pea-prosciutto-spring-pizza/".to_string()],
            ),
            (
                Website::WholeFoodsMarketDotCoDotUk,
                vec!["https://www.wholefoodsmarket.co.uk/recipes/pollo-al-ajillo".to_string()],
            ),
            (Website::WikibooksDotOrg, vec!["https://en.wikibooks.org/wiki/Cookbook:Creamed_Spinach".to_string()]),
            (
                Website::WikibooksDotOrgMobile,
                vec!["https://en.m.wikibooks.org/wiki/Cookbook:Creamed_Spinach".to_string()],
            ),
            (
                Website::WomensWeeklyFoodDotComDotAu,
                vec!["https://www.womensweeklyfood.com.au/recipe/baking/classic-lamingtons-17017/".to_string()],
            ),
            (Website::WoopDotCoDotNz, vec!["https://woop.co.nz/thai-marinated-beef-sirlion-344-2-f.html".to_string()]),
            (Website::YeMekDotNet, vec!["https://ye-mek.net/recipe/walnut-turkish-baklava-recipe".to_string()]),
            (Website::YumeliseDotFr, vec!["https://www.yumelise.fr/crepes-jambon/".to_string()]),
            (Website::ZeitDotDe, vec!["https://www.zeit.de/zeit-magazin/wochenmarkt/2021-08/kohlrabi-fenchel-carpaccio-fior-di-latte-rezept".to_string()]),
            (Website::ZenbellyDotCom, vec!["https://www.zenbelly.com/short-ribs/".to_string()]),
        ])
    })
}
