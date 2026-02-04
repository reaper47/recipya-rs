use std::collections::HashMap;
use std::sync::LazyLock;

use regex::Regex;
use whatlang::{Lang, detect_lang};

macro_rules! time_regexes {
    ($($lang:ident => $regex:expr),* $(,)?) => {
        $(
            static $lang: LazyLock<Regex> = LazyLock::new(|| {
                    Regex::new($regex).expect(concat!("Failed to compile ", stringify!($lang)))
            });
        )*
    };
}

time_regexes!(
    AFR_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:tot|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>minu(ut|te)|min\.?|sekond(e|es)|sek\.?|s\b|uur|ure|h\b)",
    AKA_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:kosi|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>simma|sɛkɛn|sikɔne|dɔnhwer|nnɔnhwerew)",
    AMH_REGEX => r"(?i)(?:(?<min>[\d]+[.,]?[\d]*)\s*(?:እስከ|-|–)\s*)?(?<max>[\d]+[.,]?[\d]*)\s*(?<unit>ደቂቃ(?:ዎች)?|ደቅ\.?|ሰከንድ(?:ዎች)?|ሰከ\.?|ሰዓት|ሰዓታት|ሰ\.?)",
    ARA_REGEX => r"(?i)(?:(?<min>[\d۰-۹]+[.,]?[\d۰-۹]*)\s*(?:إلى|الى|-|–)\s*)?(?<max>[\d۰-۹]+[.,]?[\d۰-۹]*)\s*(?<unit>دقيقة|دقائق|دق\.?|ثانية|ثواني|ثا\.?|ساعة|ساعات|سا\.?)",
    AZE_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:qədər|kimi|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>dəqiqə|dəq\.?|saniyə|san\.?|saat|sa\.?)",
    BEL_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:да|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>хвілін[ыа]?|хв\.?|секунд[ыа]?|сек\.?|гадзін[ыа]?|гадз\.?|г\b)",
    BUL_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:до|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>минут[аи]?|мин\.?|секунд[аи]?|сек\.?|час(?:а|ове)?|ч\.?)",
    CAT_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:fins a|fins|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>minut(s)?|min\.?|segon(s)?|seg\.?|s\b|hor(a|es)|h\b)",
    CES_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:až|do|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>minut[ay]?|min\.?|sekund[ay]?|sek\.?|sec|s\b|hodin[ay]?|hod\.?|h\b)",
    CMN_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:到|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>分钟|分|秒钟|秒|小时|时)",
    DAN_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:til|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>minut(?:ter)?|min\.?|sekund(?:er)?|sek\.?|s\b|time(?:r)?|t\b)",
    DEU_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:bis|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>minuten?|min\.?|m\b|sekunden?|sek\.?|s\b|stunden?|std\.?|st\.?|h\b)",
    ELL_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:έως|ως|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>λεπτό|λεπτά|λεπ\.?|δευτερόλεπτο|δευτερόλεπτα|δευτ\.?|ώρα|ώρες|ω\.?)",
    ENG_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:to|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>minutes?|mins?|m\b|seconds?|secs?|s\b|hours?|hrs?|h\b)",
    EPO_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:ĝis|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>minutoj?|min|m\b|sekundoj?|sek|s\b|horoj?|hor|h\b)",
    EST_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:kuni|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>minut(it)?|min\.?|sekund(it)?|sek\.?|s\b|tund(i)?|h\b|t\b)",
    FIN_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:asti|jopa|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>minuutti(a)?|min\.?|sekunti(a)?|sek\.?|s\b|tunti(a)?|h\b|t\b)",
    FRA_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:à|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>minutes?|min|m\b|secondes?|sec|s\b|heures?|hr|h\b)",
    GUJ_REGEX => r"(?:(?<min>[\d૦-૯]+[.,]?[\d૦-૯]*)\s*(?:સુધી|-|–)\s*)?(?<max>[\d૦-૯]+[.,]?[\d૦-૯]*)\s*(?<unit>મિનિટ|મિનિટો|સેકન્ડ|સેકંડ|સેકન્ડો|કલાક|કલાકો)",
    HEB_REGEX => r"(?i)(?:(?<min>[\d]+[.,]?[\d]*)\s*(?:עד|-|–)\s*)?(?<max>[\d]+[.,]?[\d]*)\s*(?<unit>דקות|דקה|דק'?|שניות|שנייה|שנ'?|שעות|שעה|שע'?)",
    HIN_REGEX => r"(?i)(?:(?<min>[\d०-९]+[.,]?[\d०-९]*)\s*(?:से|-|–)\s*)?(?<max>[\d०-९]+[.,]?[\d०-९]*)\s*(?<unit>मिनट|मिनटों|मि\.?|सेकंड|सेकण्ड|सेकंडों|से\.?|घंटे|घंटा|घंटों|घं\.?|घ\.?)",
    HRV_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:do|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>minut[ae]?|min\.?|sekund[ae]?|sek\.?|sat[ai]?|h\b)",
    HUN_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:-ig|akár|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>perc|másodperc|mp\.?|sec|óra|h\b)",
    HYE_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:մինչև|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>րոպե|ր\.?|վայրկյան|վ\.?|ժամ|ժ\.?)",
    IND_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:sampai|s/d|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>menit|detik|jam)",
    ITA_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:à|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>minut[o|i]?|min|m\b|second[o|i]|sec|s\b|ora|or|o\b)",
    JAV_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:nganti|ngantos|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>menit|detik|jam)",
    JPN_REGEX => r"(?:(?<min>[\d０-９]+[.,．]?[\d０-９]*)\s*(?:から|〜|～|-|–)\s*)?(?<max>[\d０-９]+[.,．]?[\d０-９]*)\s*(?<unit>分間?|分|秒間?|秒|時間|時)",
    KHM_REGEX => r"(?:(?<min>[\d០-៩]+[.,]?[\d០-៩]*)\s*(?:ដល់|-|–)\s*)?(?<max>[\d០-៩]+[.,]?[\d០-៩]*)\s*(?<unit>នាទី|វិនាទី|ម៉ោង)",
    LAT_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:ad|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>minut(a|um)|secund(a|um)|hor(a|ae))",
    LAV_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:līdz|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>minūt[es]{1,2}|min\.?|sekund[es]{1,2}|sek\.?|stund[as]{1,2}|st\.?)",
    LIT_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:iki|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>minut(ė|ės|ių)|min\.?|sekund(ė|ės|žių)|sek\.?|valand(a|os|ų)|val\.?)",
    KAT_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:-დან|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>წუთ[ი|ის|ში]?|წთ\.?|წამ[ი|ის|ში]?|წმ\.?|საათ[ი|ის|ში]?|სთ\.?)",
    KAN_REGEX => r"(?:(?<min>[\d೦-೯]+[.,]?[\d೦-೯]*)\s*(?:ವರೆಗೆ|-|–)\s*)?(?<max>[\d೦-೯]+[.,]?[\d೦-೯]*)\s*(?<unit>ನಿಮಿಷ(?:ಗಳು)?|ನಿ\.?|ಸೆಕೆಂಡ್(?:ಗಳು)?|ಸೆ\.?|ಗಂಟೆ(?:ಗಳು)?|ಗಂ\.?)",
    KOR_REGEX => r"(?:(?<min>[\d０-９]+[.,．]?[\d０-９]*)\s*(?:에서|부터|~|～|-|–)\s*)?(?<max>[\d０-９]+[.,．]?[\d０-９]*)\s*(?<unit>분|초|시간|시)",
    MAL_REGEX => r"(?:(?<min>[\d൦-൯]+[.,]?[\d൦-൯]*)\s*(?:വരെ|-|–)\s*)?(?<max>[\d൦-൯]+[.,]?[\d൦-൯]*)\s*(?<unit>മിനിറ്റ്|മിനിറ്റുകൾ|സെക്കൻഡ്|സെക്കൻഡുകൾ|മണിക്കൂർ|മണിക്കൂറുകൾ)",
    MAR_REGEX => r"(?:(?<min>[\d०-९]+[.,]?[\d०-९]*)\s*(?:ते|-|–)\s*)?(?<max>[\d०-९]+[.,]?[\d०-९]*)\s*(?<unit>मिनिट(?:े)?|मि\.?|सेकंद|से\.?|तास|ता\.?)",
    MKD_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:до|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>минут[аи]?|мин\.?|секунд[аи]?|сек\.?|час(от|а)?|ч\.?)",
    NEP_REGEX => r"(?:(?<min>[\d०-९]+[.,]?[\d०-९]*)\s*(?:सम्म|-|–)\s*)?(?<max>[\d०-९]+[.,]?[\d०-९]*)\s*(?<unit>मिनेट|सेकेन्ड|घण्टा)",
    NLD_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:tot|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>minu(ut|uten)|min\.?|seconde(n)?|sec\.?|s\b|uur|uren|u\b|h\b)",
    NOB_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:til|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>minutt(?:er)?|min\.?|sekund(?:er)?|sek\.?|s\b|time(?:r)?|t\b)",
    MYA_REGEX => r"(?:(?<min>[\d၀-၉]+[.,]?[\d၀-၉]*)\s*(?:ထိ|-|–)\s*)?(?<max>[\d၀-၉]+[.,]?[\d၀-၉]*)\s*(?<unit>မိနစ်|စက္ကန့်|နာရီ)",
    ORI_REGEX => r"(?:(?<min>[\d୦-୯]+[.,]?[\d୦-୯]*)\s*(?:ପର୍ଯ୍ୟନ୍ତ|-|–)\s*)?(?<max>[\d୦-୯]+[.,]?[\d୦-୯]*)\s*(?<unit>ମିନିଟ୍|ସେକେଣ୍ଡ|ଘଣ୍ଟା)",
    PAN_REGEX => r"(?:(?<min>[\d੦-੯]+[.,]?[\d੦-੯]*)\s*(?:ਤੋਂ|-|–)\s*)?(?<max>[\d੦-੯]+[.,]?[\d੦-੯]*)\s*(?<unit>ਮਿੰਟ|ਮਿੰਟਾਂ|ਮਿੰ\.?|ਸਕਿੰਟ|ਸਕਿੰਟਾਂ|ਸੈਕੰਡ|ਸਕਿੰ\.?|ਘੰਟਾ|ਘੰਟੇ|ਘੰਟਿਆਂ|ਘੰ\.?)",
    PES_REGEX => r"(?:(?<min>[\d۰-۹]+[.,]?[\d۰-۹]*)\s*(?:تا|-|–)\s*)?(?<max>[\d۰-۹]+[.,]?[\d۰-۹]*)\s*(?<unit>دقیقه|دقیقا|ثانیه|ثانیا|ساعت)",
    POL_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:do|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>minut[ay]?|min\.?|sekund[ay]?|sek\.?|godzin[ay]?|godz\.?|h\b)",
    POR_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:a|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>minutos?|min|m\b|segundos?|seg|s\b|horas?|hr|h\b)",
    RON_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:până la|până|la|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>minut(e)?|min\.?|secund(ă|e)|sec\.?|s\b|or(ă|e)|h\b)",
    RUS_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:до|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>минут[ыа]?|мин|секунд[ыа]?|сек|час(?:ов|а)?|ч\b)",
    SIN_REGEX => r"(?:(?<min>[\d෦-෯]+[.,]?[\d෦-෯]*)\s*(?:දක්වා|-|–)\s*)?(?<max>[\d෦-෯]+[.,]?[\d෦-෯]*)\s*(?<unit>විනාඩි|තත්පර|පැය)",
    SLK_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:až|do|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>minút[ay]?|min\.?|sekúnd[ay]?|sek\.?|s\b|hodín[ay]?|hod\.?|h\b)",
    SLV_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:do|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>minut[ae]?|min\.?|sekund[ae]?|sek\.?|s\b|ur[ae]?|h\b)",
    SNA_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:kusvika|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>mineti|maminits?i|sekondi|masekon[z|d]i|awa|maawa)",
    SPA_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:a|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>minutos?|min|m\b|segundos?|seg|s\b|horas?|hr|h\b)",
    SRP_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:do|до|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>minut[a]?|min\.|минут[а]?|мин\.|sekund[ai]?|sek\.|секунд[аи]?|сек\.|sat[ai]?|čas(ov|a)?|сат[аи]?|час(ов|а)?|h\b|ч\b)",
    SWE_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:till|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>minut(?:er)?|min\.?|sekund(?:er)?|sek\.?|s\b|timme|timmar|tim\.?|t\b)",
    TAM_REGEX => r"(?:(?<min>[\d௦-௯]+[.,]?[\d௦-௯]*)\s*(?:வரை|-|–)\s*)?(?<max>[\d௦-௯]+[.,]?[\d௦-௯]*)\s*(?<unit>நிமிடம்|நிமிடங்கள்|நிமி\.?|வினாடி|நொடி|நொடிகள்|வி\.?|மணி|மணிநேரம்|மணி\.?)",
    TEL_REGEX => r"(?:(?<min>[\d౦-౯]+[.,]?[\d౦-౯]*)\s*(?:వరకు|-|–)\s*)?(?<max>[\d౦-౯]+[.,]?[\d౦-౯]*)\s*(?<unit>నిమిషం|నిమిషాలు|సెకను|సెకన్లు|గంట|గంటలు)",
    TGL_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:hanggang|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>minuto(s)?|segundo(s)?|oras)",
    THA_REGEX => r"(?:(?<min>[\d๐-๙]+[.,]?[\d๐-๙]*)\s*(?:ถึง|~|～|-|–)\s*)?(?<max>[\d๐-๙]+[.,]?[\d๐-๙]*)\s*(?<unit>นาที|วินาที|ชั่วโมง)",
    TUK_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:çenli|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>minut|sekunt|sagat)",
    TUR_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:ila|kadar|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>dakika|dk\.?|saniye|sn\.?|saat|sa\.?)",
    URD_REGEX => r"(?:(?<min>[\d۰-۹]+[.,]?[\d۰-۹]*)\s*(?:تک|-|–)\s*)?(?<max>[\d۰-۹]+[.,]?[\d۰-۹]*)\s*(?<unit>منٹ|منٹوں|سیکنڈ|سیکنڈوں|گھنٹہ|گھنٹے|گھنٹوں)",
    UKR_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:до|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>годин[иау]?|год|хвилин[иау]?|хв|секунд[иау]?|сек)",
    UZB_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:gacha|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>daqiqa|minut|soniya|sekund|soat)",
    VIE_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:đến|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>phút|giây|giờ|tiếng)",
    YID_REGEX => r"(?i)(?:(?<min>[\d]+[.,]?[\d]*)\s*(?:ביז|-|–)\s*)?(?<max>[\d]+[.,]?[\d]*)\s*(?<unit>מינוט|מינוטן|מינ'?|סעקונדע|סעקונדן|סעק'?|שעה|שעות|שע'?)",
    ZUL_REGEX => r"(?i)(?:(?<min>\d+[.,]?\d*)\s*(?:kuya ku|kuya|-|–)\s*)?(?<max>\d+[.,]?\d*)\s*(?<unit>umzuzu|imizuz[u|wana]|amaminithi|min\.?|isekhondi|amasekhondi|sek\.?|s\b|ihora|amahora|h\b)",
);

static LANGUAGE_PATTERNS: LazyLock<HashMap<Lang, &'static Regex>> = LazyLock::new(|| {
    HashMap::from([
        (Lang::Afr, &*AFR_REGEX),
        (Lang::Aka, &*AKA_REGEX),
        (Lang::Amh, &*AMH_REGEX),
        (Lang::Ara, &*ARA_REGEX),
        (Lang::Aze, &*AZE_REGEX),
        (Lang::Bel, &*BEL_REGEX),
        (Lang::Bul, &*BUL_REGEX),
        (Lang::Cat, &*CAT_REGEX),
        (Lang::Ces, &*CES_REGEX),
        (Lang::Cmn, &*CMN_REGEX),
        (Lang::Dan, &*DAN_REGEX),
        (Lang::Deu, &*DEU_REGEX),
        (Lang::Ell, &*ELL_REGEX),
        (Lang::Eng, &*ENG_REGEX),
        (Lang::Est, &*EST_REGEX),
        (Lang::Epo, &*EPO_REGEX),
        (Lang::Fin, &*FIN_REGEX),
        (Lang::Fra, &*FRA_REGEX),
        (Lang::Guj, &*GUJ_REGEX),
        (Lang::Heb, &*HEB_REGEX),
        (Lang::Hin, &*HIN_REGEX),
        (Lang::Hrv, &*HRV_REGEX),
        (Lang::Hun, &*HUN_REGEX),
        (Lang::Hye, &*HYE_REGEX),
        (Lang::Ind, &*IND_REGEX),
        (Lang::Ita, &*ITA_REGEX),
        (Lang::Jav, &*JAV_REGEX),
        (Lang::Jpn, &*JPN_REGEX),
        (Lang::Kat, &*KAT_REGEX),
        (Lang::Kan, &*KAN_REGEX),
        (Lang::Khm, &*KHM_REGEX),
        (Lang::Kor, &*KOR_REGEX),
        (Lang::Lat, &*LAT_REGEX),
        (Lang::Lav, &*LAV_REGEX),
        (Lang::Lit, &*LIT_REGEX),
        (Lang::Mal, &*MAL_REGEX),
        (Lang::Mar, &*MAR_REGEX),
        (Lang::Mkd, &*MKD_REGEX),
        (Lang::Mya, &*MYA_REGEX),
        (Lang::Nep, &*NEP_REGEX),
        (Lang::Nld, &*NLD_REGEX),
        (Lang::Nob, &*NOB_REGEX),
        (Lang::Ori, &*ORI_REGEX),
        (Lang::Pan, &*PAN_REGEX),
        (Lang::Pes, &*PES_REGEX),
        (Lang::Pol, &*POL_REGEX),
        (Lang::Por, &*POR_REGEX),
        (Lang::Ron, &*RON_REGEX),
        (Lang::Rus, &*RUS_REGEX),
        (Lang::Sin, &*SIN_REGEX),
        (Lang::Slk, &*SLK_REGEX),
        (Lang::Slv, &*SLV_REGEX),
        (Lang::Sna, &*SNA_REGEX),
        (Lang::Srp, &*SRP_REGEX),
        (Lang::Spa, &*SPA_REGEX),
        (Lang::Swe, &*SWE_REGEX),
        (Lang::Tam, &*TAM_REGEX),
        (Lang::Tel, &*TEL_REGEX),
        (Lang::Tgl, &*TGL_REGEX),
        (Lang::Tha, &*THA_REGEX),
        (Lang::Tuk, &*TUK_REGEX),
        (Lang::Tur, &*TUR_REGEX),
        (Lang::Urd, &*URD_REGEX),
        (Lang::Ukr, &*UKR_REGEX),
        (Lang::Uzb, &*UZB_REGEX),
        (Lang::Vie, &*VIE_REGEX),
        (Lang::Yid, &*YID_REGEX),
        (Lang::Zul, &*ZUL_REGEX),
    ])
});

pub struct TimeParser;

impl Default for TimeParser {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeParser {
    pub const fn new() -> Self {
        Self
    }

    pub fn parse_max_time_seconds(&self, text: &str) -> Option<i32> {
        let lang = detect_lang(text)?;

        let pattern = match LANGUAGE_PATTERNS.get(&lang) {
            None => &*ENG_REGEX,
            Some(&p) => p,
        };

        self.extract_time(text, pattern, lang)
    }

    fn extract_time(&self, text: &str, pattern: &Regex, lang: Lang) -> Option<i32> {
        pattern
            .captures_iter(text)
            .filter_map(|cap| {
                let max_val = cap.name("max").and_then(|m| m.as_str().parse::<f32>().ok());
                let unit = cap.name("unit")?.as_str();
                self.normalize_to_seconds(max_val.unwrap_or_default(), unit, lang)
            })
            .max()
    }
    #[allow(clippy::too_many_lines)]
    fn normalize_to_seconds(&self, value: f32, unit: &str, lang: Lang) -> Option<i32> {
        match lang {
            Lang::Cmn => match unit {
                s if s.starts_with("秒钟") || s.starts_with('秒') => Some(to_seconds(value)),
                s if s.starts_with('分') => Some(to_minutes(value)),
                s if s.starts_with('小') => Some(to_hours(value)),
                _ => None,
            },
            Lang::Eng
            | Lang::Epo
            | Lang::Fra
            | Lang::Por
            | Lang::Spa
            | Lang::Ita
            | Lang::Ces
            | Lang::Ron
            | Lang::Lat
            | Lang::Slk
            | Lang::Cat
            | Lang::Tgl => match unit {
                s if s.starts_with('s') || s.starts_with('d') => Some(to_seconds(value)),
                s if s.starts_with('m') => Some(to_minutes(value)),
                s if s.starts_with('h') || s.starts_with('o') => Some(to_hours(value)),
                _ => None,
            },
            Lang::Deu | Lang::Lav => match unit.to_lowercase().as_str() {
                s if s.starts_with("sek") => Some(to_seconds(value)),
                s if s.starts_with('m') => Some(to_minutes(value)),
                s if s.starts_with("st") || s.starts_with('o') => Some(to_hours(value)),
                _ => None,
            },
            Lang::Kat => match unit {
                s if s.starts_with("წა") => Some(to_seconds(value)),
                s if s.starts_with("წუ") => Some(to_minutes(value)),
                s if s.starts_with('ს') => Some(to_hours(value)),
                _ => None,
            },
            Lang::Ara => match unit {
                s if s.starts_with("ثا") => Some(to_seconds(value)),
                s if s.starts_with("د") => Some(to_minutes(value)),
                s if s.starts_with("س") => Some(to_hours(value)),
                _ => None,
            },
            Lang::Hin => match unit {
                s if s.starts_with("से") => Some(to_seconds(value)),
                s if s.starts_with("मि") => Some(to_minutes(value)),
                s if s.starts_with("घं") => Some(to_hours(value)),
                _ => None,
            },
            Lang::Jpn => match unit {
                s if s.starts_with("秒") => Some(to_seconds(value)),
                s if s.starts_with("分") => Some(to_minutes(value)),
                s if s.starts_with("時") => Some(to_hours(value)),
                _ => None,
            },
            Lang::Heb => match unit {
                s if s.starts_with("שנ") => Some(to_seconds(value)),
                s if s.starts_with("ד") => Some(to_minutes(value)),
                s if s.starts_with("שע") => Some(to_hours(value)),
                _ => None,
            },
            Lang::Yid => match unit {
                s if s.starts_with("ס") => Some(to_seconds(value)),
                s if s.starts_with("מ") => Some(to_minutes(value)),
                s if s.starts_with("ש") => Some(to_hours(value)),
                _ => None,
            },
            Lang::Pol => match unit {
                s if s.starts_with('s') => Some(to_seconds(value)),
                s if s.starts_with('m') => Some(to_minutes(value)),
                s if s.starts_with('g') => Some(to_hours(value)),
                _ => None,
            },
            Lang::Amh => match unit {
                s if s.starts_with("ሰከ") => Some(to_seconds(value)),
                s if s.starts_with("ደ") => Some(to_minutes(value)),
                s if s.starts_with("ሰዓ") => Some(to_hours(value)),
                _ => None,
            },
            Lang::Jav | Lang::Ind => match unit {
                s if s.starts_with('d') => Some(to_seconds(value)),
                s if s.starts_with('m') => Some(to_minutes(value)),
                s if s.starts_with('j') => Some(to_hours(value)),
                _ => None,
            },
            Lang::Kor => match unit {
                s if s.starts_with("초") => Some(to_seconds(value)),
                s if s.starts_with("분") => Some(to_minutes(value)),
                s if s.starts_with("시") => Some(to_hours(value)),
                _ => None,
            },
            Lang::Nob | Lang::Dan | Lang::Swe | Lang::Fin | Lang::Est => match unit {
                s if s.starts_with('s') => Some(to_seconds(value)),
                s if s.starts_with('m') => Some(to_minutes(value)),
                s if s.starts_with('t') => Some(to_hours(value)),
                _ => None,
            },
            Lang::Tur | Lang::Aze => match unit {
                s if s.starts_with("san") => Some(to_seconds(value)),
                s if s.starts_with('d') => Some(to_minutes(value)),
                s if s.starts_with("saa") => Some(to_hours(value)),
                _ => None,
            },
            Lang::Nld | Lang::Slv | Lang::Afr => match unit {
                s if s.starts_with('s') => Some(to_seconds(value)),
                s if s.starts_with('m') => Some(to_minutes(value)),
                s if s.starts_with('u') => Some(to_hours(value)),
                _ => None,
            },
            Lang::Hun => match unit {
                s if s.starts_with('m') => Some(to_seconds(value)),
                s if s.starts_with('p') => Some(to_minutes(value)),
                s if s.starts_with('ó') => Some(to_hours(value)),
                _ => None,
            },
            Lang::Ell => match unit {
                s if s.starts_with('δ') => Some(to_seconds(value)),
                s if s.starts_with('λ') => Some(to_minutes(value)),
                s if s.starts_with('ώ') => Some(to_hours(value)),
                _ => None,
            },
            Lang::Ben => match unit {
                s if s.starts_with("সে") => Some(to_seconds(value)),
                s if s.starts_with("মি'") => Some(to_minutes(value)),
                s if s.starts_with('ঘ') => Some(to_hours(value)),
                _ => None,
            },
            Lang::Bel => match unit {
                s if s.starts_with('с') => Some(to_seconds(value)),
                s if s.starts_with('х') => Some(to_minutes(value)),
                s if s.starts_with('г') => Some(to_hours(value)),
                _ => None,
            },
            Lang::Mar => match unit {
                s if s.starts_with("से") => Some(to_seconds(value)),
                s if s.starts_with("मि") => Some(to_minutes(value)),
                s if s.starts_with("ता") => Some(to_hours(value)),
                _ => None,
            },
            Lang::Kan => match unit {
                s if s.starts_with("ಸೆ") => Some(to_seconds(value)),
                s if s.starts_with("ನಿ") => Some(to_minutes(value)),
                s if s.starts_with("ಗ") => Some(to_hours(value)),
                _ => None,
            },
            Lang::Hrv | Lang::Tuk => match unit {
                s if s.starts_with("se") => Some(to_seconds(value)),
                s if s.starts_with('m') => Some(to_minutes(value)),
                s if s.starts_with("sa") => Some(to_hours(value)),
                _ => None,
            },
            Lang::Srp => match unit {
                s if s.starts_with("се") => Some(to_seconds(value)),
                s if s.starts_with('м') => Some(to_minutes(value)),
                s if s.starts_with("са") => Some(to_hours(value)),
                _ => None,
            },
            Lang::Lit => match unit {
                s if s.starts_with('s') => Some(to_seconds(value)),
                s if s.starts_with('m') => Some(to_minutes(value)),
                s if s.starts_with('v') => Some(to_hours(value)),
                _ => None,
            },
            Lang::Tam => match unit {
                s if s.starts_with("வி") => Some(to_seconds(value)),
                s if s.starts_with("நி") => Some(to_minutes(value)),
                s if s.starts_with("ம") => Some(to_hours(value)),
                _ => None,
            },
            Lang::Vie => match unit {
                s if s.starts_with("giâ") => Some(to_seconds(value)),
                s if s.starts_with('p') => Some(to_minutes(value)),
                s if s.starts_with("giờ") => Some(to_hours(value)),
                _ => None,
            },
            Lang::Urd => match unit {
                s if s.starts_with("س") => Some(to_seconds(value)),
                s if s.starts_with("م") => Some(to_minutes(value)),
                s if s.starts_with("گ") => Some(to_hours(value)),
                _ => None,
            },
            Lang::Tha => match unit {
                s if s.starts_with("วิ") => Some(to_seconds(value)),
                s if s.starts_with('น') => Some(to_minutes(value)),
                s if s.starts_with("ชั่") => Some(to_hours(value)),
                _ => None,
            },
            Lang::Guj => match unit {
                s if s.starts_with("સે") => Some(to_seconds(value)),
                s if s.starts_with("મિ") => Some(to_minutes(value)),
                s if s.starts_with('ક') => Some(to_hours(value)),
                _ => None,
            },
            Lang::Uzb => match unit {
                s if s.starts_with("son") => Some(to_seconds(value)),
                s if s.starts_with('d') => Some(to_minutes(value)),
                s if s.starts_with("soa") => Some(to_hours(value)),
                _ => None,
            },
            Lang::Pan => match unit {
                s if s.starts_with("ਸਕ") => Some(to_seconds(value)),
                s if s.starts_with("ਮਿੰ") => Some(to_minutes(value)),
                s if s.starts_with("ਘੰ") => Some(to_hours(value)),
                _ => None,
            },
            Lang::Tel => match unit {
                s if s.starts_with("సె") => Some(to_seconds(value)),
                s if s.starts_with("ని") => Some(to_minutes(value)),
                s if s.starts_with("గం") => Some(to_hours(value)),
                _ => None,
            },
            Lang::Pes => match unit {
                s if s.starts_with("ث") => Some(to_seconds(value)),
                s if s.starts_with("د") => Some(to_minutes(value)),
                s if s.starts_with("س") => Some(to_hours(value)),
                _ => None,
            },
            Lang::Mal => match unit {
                s if s.starts_with("സെ") => Some(to_seconds(value)),
                s if s.starts_with("മി") => Some(to_minutes(value)),
                s if s.starts_with("മ") => Some(to_hours(value)),
                _ => None,
            },
            Lang::Ori => match unit {
                s if s.starts_with("ସେ") => Some(to_seconds(value)),
                s if s.starts_with("ମି") => Some(to_minutes(value)),
                s if s.starts_with("ଘ") => Some(to_hours(value)),
                _ => None,
            },
            Lang::Mya => match unit {
                s if s.starts_with("စ") => Some(to_seconds(value)),
                s if s.starts_with("မိ") => Some(to_minutes(value)),
                s if s.starts_with("န") => Some(to_hours(value)),
                _ => None,
            },
            Lang::Nep => match unit {
                s if s.starts_with("से") => Some(to_seconds(value)),
                s if s.starts_with("मि") => Some(to_minutes(value)),
                s if s.starts_with("घ") => Some(to_hours(value)),
                _ => None,
            },
            Lang::Sin => match unit {
                s if s.starts_with("ත") => Some(to_seconds(value)),
                s if s.starts_with("වි") => Some(to_minutes(value)),
                s if s.starts_with("ප") => Some(to_hours(value)),
                _ => None,
            },
            Lang::Khm => match unit {
                s if s.starts_with("វិនា") => Some(to_seconds(value)),
                s if s.starts_with("នា") => Some(to_minutes(value)),
                s if s.starts_with("ម៉ោ") => Some(to_hours(value)),
                _ => None,
            },
            Lang::Aka => match unit {
                s if s.starts_with("sik") => Some(to_seconds(value)),
                s if s.starts_with("si") => Some(to_minutes(value)),
                s if s.starts_with('n') => Some(to_hours(value)),
                _ => None,
            },
            Lang::Zul => match unit {
                s if s.starts_with('i') => Some(to_seconds(value)),
                s if s.starts_with("amam") => Some(to_minutes(value)),
                s if s.starts_with("amah") => Some(to_hours(value)),
                _ => None,
            },
            Lang::Sna => match unit {
                s if s.starts_with("mas") => Some(to_seconds(value)),
                s if s.starts_with("mam") => Some(to_minutes(value)),
                s if s.starts_with("maa") => Some(to_hours(value)),
                _ => None,
            },
            Lang::Hye => match unit {
                s if s.starts_with('վ') => Some(to_seconds(value)),
                s if s.starts_with('ր') => Some(to_minutes(value)),
                s if s.starts_with('ժ') => Some(to_hours(value)),
                _ => None,
            },
            Lang::Rus | Lang::Ukr | Lang::Bul | Lang::Mkd => match unit {
                s if s.starts_with('с') => Some(to_seconds(value)),
                s if s.starts_with('м') || s.starts_with('х') => Some(to_minutes(value)),
                s if s.starts_with('ч') || s.starts_with('г') => Some(to_hours(value)),
                _ => None,
            },
        }
    }
}

#[allow(clippy::cast_possible_truncation)]
const fn to_seconds(value: f32) -> i32 {
    value as i32
}

#[allow(clippy::cast_possible_truncation)]
const fn to_minutes(value: f32) -> i32 {
    value.round() as i32 * 60
}

#[allow(clippy::cast_possible_truncation)]
const fn to_hours(value: f32) -> i32 {
    (value * 3600.0).round() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    mod tests_time_parser {
        use super::*;

        mod tests_parse_max_time {
            use super::*;

            fn base_text(lang: Lang, t: &str) -> String {
                match lang {
                    Lang::Epo => format!(
                        "Post kiam la pasto kuiriĝis dum ĉirkaŭ {t}, aŭ ĝis la tuta akvo estas absorbita"
                    ),
                    Lang::Eng => format!(
                        "After pasta has cooked for approximately {t}, or until all the water has been absorbed"
                    ),
                    Lang::Rus => format!(
                        "После того, как макароны будут вариться примерно {t}, или пока вся вода не впитается"
                    ),
                    Lang::Cmn => format!("意大利面煮约{t}后，或直到所有水分都被吸收"),
                    Lang::Spa => format!(
                        "Después de que la pasta se haya cocinado durante aproximadamente {t}, o hasta que se haya absorbido toda el agua"
                    ),
                    Lang::Por => format!(
                        "Depois de a massa estar cozida durante cerca de {t}, ou até que toda a água tenha sido absorvida"
                    ),
                    Lang::Ita => format!(
                        "Dopo che la pasta è cotta per circa {t}, o finché tutta l'acqua non è stata assorbita"
                    ),
                    Lang::Ben => {
                        format!("পাস্তা প্রায় {t} ধরে রান্না হওয়ার পর, অথবা যতক্ষণ না সমস্ত জল শোষিত হয়")
                    }
                    Lang::Fra => format!(
                        "Après que les pâtes aient cuit pendant environ {t}, ou jusqu'à ce que toute l'eau ait été absorbée"
                    ),
                    Lang::Deu => format!(
                        "Nachdem die Nudeln etwa {t} gekocht haben oder bis das gesamte Wasser absorbiert wurde"
                    ),
                    Lang::Ukr => format!(
                        "Після того, як макарони зваряться приблизно {t}, або доки вся вода не вбереться"
                    ),
                    Lang::Kat => format!(
                        "მაკარონის დაახლოებით {t} განმავლობაში მოხარშვის შემდეგ, ან სანამ წყალი მთლიანად არ შეიწოვება"
                    ),
                    Lang::Ara => {
                        format!("بعد طهي المعكرونة لمدة {t} تقريبًا، أو حتى يتم امتصاص كل الماء")
                    }
                    Lang::Hin => {
                        format!("पास्ता लगभग {t} तक पकने के बाद, या जब तक सारा पानी सोख न लिया जाए")
                    }
                    Lang::Jpn => {
                        format!("パスタを約{t}分間茹でた後、または水分がすべて吸収されるまで")
                    }
                    Lang::Heb => format!("לאחר שהפסטה בישלה במשך כ-{t}, או עד שכל המים נספגו"),
                    Lang::Yid => format!(
                        "נאכדעם וואס די פאסטע האט געקאכט פאר בערך {t}, אדער ביז אלע וואסער איז איינגעזאפט געווארן"
                    ),
                    Lang::Pol => format!(
                        "Po ugotowaniu makaronu przez około {t} lub do momentu wchłonięcia całej wody"
                    ),
                    Lang::Amh => format!("ፓስታ በግምት {t} ከተበስል በኋላ ወይም ውሃው በሙሉ እስኪዋጥ ድረስ"),
                    Lang::Jav => format!(
                        "Sawise pasta masak kira-kira {t}, utawa nganti kabeh banyu wis diserap"
                    ),
                    Lang::Kor => format!("파스타가 약 {t} 동안 조리되거나 물이 모두 흡수될 때까지"),
                    Lang::Nob => format!(
                        "Bokmål er en av to offisielle målformer av norsk {t} skriftspråk, hvorav den andre er nynorsk."
                    ),
                    Lang::Dan => format!(
                        "Efter pastaen er kogt i cirka {t}, eller indtil alt vandet er absorberet"
                    ),
                    Lang::Swe => format!(
                        "Efter att pastan har kokat i ungefär {t}, eller tills allt vatten har absorberats"
                    ),
                    Lang::Fin => format!(
                        "Kun pasta on keitetty noin {t} tai kunnes kaikki vesi on imeytynyt"
                    ),
                    Lang::Tur => format!(
                        "Makarna yaklaşık {t} süre piştikten sonra veya tüm su emilene kadar"
                    ),
                    Lang::Nld => format!(
                        "Nadat de pasta ongeveer {t} heeft gekookt, of totdat al het water is opgenomen"
                    ),
                    Lang::Hun => format!(
                        "Miután a tészta körülbelül {t} percig főtt, vagy amíg az összes vizet felszívta"
                    ),
                    Lang::Ces => format!(
                        "Po uvaření těstovin přibližně {t} nebo dokud se veškerá voda nevsákne"
                    ),
                    Lang::Ell => format!(
                        "Αφού τα ζυμαρικά βράσουν για περίπου {t} ή μέχρι να απορροφηθεί όλο το νερό"
                    ),
                    Lang::Bul => format!(
                        "След като пастата се е сварила приблизително {t} или докато цялата вода се абсорбира"
                    ),
                    Lang::Bel => format!(
                        "Пасля таго, як макароны будуць варыцца прыблізна {t}, або пакуль уся вада не ўбярэцца"
                    ),
                    Lang::Mar => {
                        format!("पास्ता साधारण {t} पर्यंत शिजल्यानंतर, किंवा सर्व पाणी शोषले जाईपर्यंत")
                    }
                    Lang::Kan => format!("ಪಾಸ್ತಾ ಸುಮಾರು {t} ಬೇಯಿಸಿದ ನಂತರ, ಅಥವಾ ಎಲ್ಲಾ ನೀರು ಹೀರಿಕೊಳ್ಳುವವರೆಗೆ"),
                    Lang::Ron => format!(
                        "După ce pastele au fiert timp de aproximativ {t} sau până când toată apa a fost absorbită"
                    ),
                    Lang::Slv => format!(
                        "Ko se testenine kuhajo približno {t} oziroma dokler se vsa voda ne vpije"
                    ),
                    Lang::Hrv => format!(
                        "Nakon što se tjestenina kuha otprilike {t} ili dok se sva voda ne upije"
                    ),
                    Lang::Srp => format!(
                        "Након насељавања Срба на Балкан током 6. и 7. века, {t} Срби су у раном средњем веку основали неколико држава. Средњовековна Србија је 1217. постала краљевина, а врхунац је достигла 1346. проглашењем царства. Након турске најезде, Српска деспотовина је опстала до 1459, када је пала под власт"
                    ),
                    Lang::Mkd => format!(
                        "Откако тестенините ќе се варат приближно {t}, или додека целата вода не се апсорбира"
                    ),
                    Lang::Lit => {
                        format!("Makaronams išvirus maždaug {t} arba kol susigers visas vanduo")
                    }
                    Lang::Lav => format!(
                        "Pēc tam, kad makaroni ir vārījušies aptuveni {t} vai līdz viss ūdens ir uzsūcies"
                    ),
                    Lang::Est => format!(
                        "Pärast seda, kui pasta on keenud umbes {t} või kuni kogu vesi on imendunud"
                    ),
                    Lang::Tam => {
                        format!("பாஸ்தா தோராயமாக {t} வேகவைத்த பிறகு, அல்லது அனைத்து தண்ணீரும் உறிஞ்சப்படும் வரை")
                    }
                    Lang::Vie => format!(
                        "Sau khi mì đã nấu được khoảng {t}, hoặc cho đến khi toàn bộ nước đã được hấp thụ"
                    ),
                    Lang::Urd => {
                        format!("پاستا تقریباً {t} پکانے کے بعد، یا جب تک سارا پانی جذب نہ ہو جائے")
                    }
                    Lang::Tha => format!("หลังจากต้มเส้นพาสต้าประมาณ {t} หรือจนกว่าน้ำจะถูกดูดซึมจนหมด"),
                    Lang::Guj => {
                        format!("પાસ્તા લગભગ {t} સુધી રાંધ્યા પછી, અથવા બધું પાણી શોષાઈ જાય ત્યાં સુધી")
                    }
                    Lang::Uzb => format!(
                        "Makaron taxminan {t} pishganidan keyin yoki butun suv singib ketguncha"
                    ),
                    Lang::Pan => {
                        format!("ਪਾਸਤਾ ਦੇ ਲਗਭਗ {t} ਤੱਕ ਪੱਕਣ ਤੋਂ ਬਾਅਦ, ਜਾਂ ਜਦੋਂ ਤੱਕ ਸਾਰਾ ਪਾਣੀ ਸੋਖ ਨਹੀਂ ਜਾਂਦਾ")
                    }
                    Lang::Aze => format!(
                        "Makaron təxminən {t} bişirildikdən sonra və ya bütün su udulana qədər"
                    ),
                    Lang::Ind => format!(
                        "Setelah pasta dimasak selama kurang lebih {t}, atau sampai semua air terserap"
                    ),
                    Lang::Tel => format!("పాస్తా దాదాపు {t} సేపు ఉడికిన తర్వాత, లేదా నీళ్లన్నీ పీల్చుకునే వరకు"),
                    Lang::Pes => format!(
                        "بعد از اینکه پاستا تقریباً به مدت {t} پخته شد، یا تا زمانی که تمام آب آن جذب شود"
                    ),
                    Lang::Mal => format!(
                        "പാസ്ത ഏകദേശം {t} വേവിച്ചതിനുശേഷം, അല്ലെങ്കിൽ എല്ലാ വെള്ളവും ആഗിരണം ചെയ്യപ്പെടുന്നതുവരെ"
                    ),
                    Lang::Ori => {
                        format!("ପାସ୍ତା ପ୍ରାୟ {t} ପର୍ଯ୍ୟନ୍ତ ରନ୍ଧା ହେବା ପରେ, କିମ୍ବା ସମସ୍ତ ପାଣି ଶୋଷିତ ହେବା ପର୍ଯ୍ୟନ୍ତ")
                    }
                    Lang::Mya => {
                        format!("ခေါက်ဆွဲ ခန့်မှန်းခြေအားဖြင့် {t} ပြုတ်ပြီးနောက်၊ သို့မဟုတ် ရေအားလုံး စုပ်ယူသွားသည်အထိ")
                    }
                    Lang::Nep => format!("पास्ता लगभग {t} पाकेपछि, वा सबै पानी सोसिएसम्म"),
                    Lang::Sin => format!("පැස්ටා ආසන්න වශයෙන් {t} ක් පිසූ පසු, හෝ සියලු ජලය අවශෝෂණය වන තුරු"),
                    Lang::Khm => format!("បន្ទាប់ពីប៉ាស្តាបានចម្អិនប្រហែល {t} ឬរហូតដល់ទឹកទាំងអស់ត្រូវបានស្រូបយក"),
                    Lang::Tuk => {
                        format!("Makaron takmynan {t} for bişirilenden ýa-da ähli suw siňýänçä")
                    }
                    Lang::Aka => format!(
                        "Ԑwiee bere a pasta no ahyeɛ bɛyɛ {t}, anaa kosi sɛ nsu no nyinaa bɛhyew"
                    ),
                    Lang::Zul => format!(
                        "Ngemuva kokuthi i-pasta iphekwe cishe i-{t}, noma kuze kube yilapho wonke amanzi esemuncwa"
                    ),
                    Lang::Sna => format!(
                        "Mushure mokunge pasta yakabikwa kweinenge {t}, kana kusvikira mvura yose yave yatorwa"
                    ),
                    Lang::Afr => format!(
                        "Nadat die pasta vir ongeveer {t} gekook het, of totdat al die water geabsorbeer is"
                    ),
                    Lang::Lat => format!(
                        "Postquam pasta cocta est per circiter {t}, vel donec omnis aqua absorpta sit"
                    ),
                    Lang::Slk => format!(
                        "Po uvarení cestovín približne {t} alebo kým sa všetka voda nevstrebe"
                    ),
                    Lang::Cat => format!(
                        "Després que la pasta s'hagi cuit durant aproximadament {t}, o fins que s'hagi absorbit tota l'aigua"
                    ),
                    Lang::Tgl => format!(
                        "Matapos maluto ang pasta nang humigit-kumulang {t}, o hanggang sa masipsip ang lahat ng tubig"
                    ),
                    Lang::Hye => format!(
                        "Մակարոնեղենը մոտավորապես {t} եփելուց հետո կամ մինչև ամբողջ ջուրը ներծծվի"
                    ),
                }
            }

            mod tests_afrikaans {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Afr, "354 sekondes"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Afr, "10 minute"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Afr, "1.5 uur"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_akan {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Aka, "354 sikɔne"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Aka, "10 simma"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Aka, "1.5 nnɔnhwerew"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_amharic {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Amh, "354 ሰከንድ"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Amh, "10 ደቂቃዎች"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Amh, "1.5 ሰዓታት"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_arabic {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Ara, "354 ثانية"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Ara, "10 دقيقتان"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Ara, "1.5 ساعة واحدة"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_armenian {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Hye, "354 վայրկյան"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Hye, "10 րոպե"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Hye, "1.5 ժամ"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_azerbaijani {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Aze, "354 saniyə"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Aze, "10 dəqiqə"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Aze, "1.5 saat"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_belarusian {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Bel, "354 секунд"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Bel, "10 хвілін"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Bel, "1.5 гадзін"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_bulgarian {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Bul, "354 секунди"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Bul, "10 минути"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Bul, "1.5 часа"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_burmese {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Mya, "354 စက္ကန့်"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Mya, "10 မိနစ်"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Mya, "1.5 နာရီ"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_catalan {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Cat, "354 segons"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Cat, "10 minuts"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Cat, "1.5 hores"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_croatian {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Hrv, "354 sekundi"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Hrv, "10 minuta"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Hrv, "1.5 sati"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_danish {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Dan, "354 sekunder"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Dan, "10 minutter"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Dan, "1.5 timer"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_dutch {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Nld, "354 seconden"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Nld, "10 minuten"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Nld, "1.5 uur"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_czech {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Ces, "354 sekund"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Ces, "10 minut"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Ces, "1.5 hodin"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_english {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Eng, "354 seconds"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Eng, "10m"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Eng, "1.5 hours"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_esperanto {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Epo, "345 sekundoj"));

                    assert_eq!(got, Some(345));
                }

                #[test]
                fn test_minutes() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Epo, "10 minutoj"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Epo, "1.5 horoj"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_estonian {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Est, "345 sekundit"));

                    assert_eq!(got, Some(345));
                }

                #[test]
                fn test_minutes() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Est, "10 minutit"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Est, "1.5 tundi"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_finnish {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Fin, "354 sekuntia"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Fin, "10 minuutti"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Fin, "1.5 tunti"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_french {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Fra, "50 seconds"));

                    assert_eq!(got, Some(50));
                }

                #[test]
                fn test_minutes() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Fra, "50 minutes"));

                    assert_eq!(got, Some(50 * 60));
                }

                #[test]
                fn test_hours() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Fra, "1.5 heures"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_gujarati {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Guj, "50 સેકન્ડ"));

                    assert_eq!(got, Some(50));
                }

                #[test]
                fn test_minutes() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Guj, "50 મિનિટ"));

                    assert_eq!(got, Some(50 * 60));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Guj, "1.5 કલાક"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_indonesian {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Ind, "50 detik"));

                    assert_eq!(got, Some(50));
                }

                #[test]
                fn test_minutes() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Ind, "50 menit"));

                    assert_eq!(got, Some(50 * 60));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Ind, "1.5 jam"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_italian {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Ita, "50 secondi"));

                    assert_eq!(got, Some(50));
                }

                #[test]
                fn test_minutes() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Ita, "50 minuto"));

                    assert_eq!(got, Some(50 * 60));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Ita, "1.5 ora"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_georgian {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Kat, "354 წამი"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Kat, "10 წუთი"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Kat, "1.5 საათი"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_german {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Deu, "354 Sekunden"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Deu, "10 Minuten"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Deu, "1.5 Stunden"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_greek {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Ell, "354 δευτερόλεπτα"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Ell, "10 λεπτά"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Ell, "1.5 ώρες"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_hebrew {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Heb, "354 שניות"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Heb, "10 דקות"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Heb, "1.5 שעות"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_hindi {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Hin, "354 सेकंड"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Hin, "10 मिनटों"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Hin, "1.5 घंटे"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_hungarian {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Hun, "354 másodperc"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Hun, "10 perc"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Hun, "1.5 óra"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_latin {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Lat, "354 secunda"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Lat, "10 minuta"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Lat, "1.5 horae"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_latvian {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Lav, "354 sekundes"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Lav, "10 minūtes"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Lav, "1.5 stundas"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_lithuanian {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Lit, "354 sekundės"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Lit, "10 minutės"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Lit, "1.5 valandos"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_javanese {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Jav, "354 detik"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Jav, "10 menit"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Jav, "1.5 jam"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_japanese {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Jpn, "354 秒"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Jpn, "10 分"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Jpn, "1.5 時間"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_kannada {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Kan, "354 ಸೆಕೆಂಡುಗಳು"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Kan, "10 ನಿಮಿಷಗಳು"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Kan, "1.5 ಗಂಟೆಗಳು"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_khmer {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Khm, "354 វិនាទី"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Khm, "10 នាទី។"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Khm, "1.5 ម៉ោង។"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_korean {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Kor, "354초"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Kor, "10분"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Kor, "1.5시간"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_macedonian {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Mkd, "354 секунди"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Mkd, "50 минути"));

                    assert_eq!(got, Some(50 * 60));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Mkd, "1.5 часа"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_malayalam {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Mal, "354 സെക്കൻഡ്"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Mal, "50 മിനിറ്റ്"));

                    assert_eq!(got, Some(50 * 60));
                }

                #[test]
                fn test_hours() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Mal, "1.5 മണിക്കൂർ"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_mandarin {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Cmn, "354秒"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Cmn, "50分钟"));

                    assert_eq!(got, Some(50 * 60));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Cmn, "1.5小时"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_marathi {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Mar, "354 सेकंद"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Mar, "50 मिनिटे"));

                    assert_eq!(got, Some(50 * 60));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Mar, "1.5 तास"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_nepali {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Nep, "354 सेकेन्ड"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Nep, "50 मिनेट"));

                    assert_eq!(got, Some(50 * 60));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Nep, "1.5 घण्टा"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_norwegian_bokmal {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Nob, "354 sekunder"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Nob, "10 minutter"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Nob, "1.5 timer"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_oriya {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Ori, "354 ସେକେଣ୍ଡ"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Ori, "10 ମିନିଟ୍"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Ori, "1.5 ଘଣ୍ଟା"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_persian {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Pes, "354 ثانیه"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Pes, "10 دقیقه"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Pes, "1.5 ساعت"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_polish {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Pol, "354 sekundy"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Pol, "10 minuty"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Pol, "1.5 godziny"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_punjabi {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Pan, "354 ਸਕਿੰਟ"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Pan, "10 ਮਿੰਟ"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Pan, "1.5 ਘੰਟੇ"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_portuguese {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Por, "354 segundos"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Por, "10 minutos"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Por, "1.5 horas"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_serbian {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Srp, "354 секунди"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Srp, "10 минута"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Srp, "1.5 сати"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_slovak {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Slk, "354 sekúnd"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Slk, "10 minút"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Slk, "1.5 hodín"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_slovenian {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Slv, "354 sekund"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Slv, "10 minut"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Slv, "1.5 ur"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_spanish {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Spa, "354 segundos"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Spa, "10 minutos"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Spa, "1.5 horas"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_swedish {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Swe, "354 sekunder"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Swe, "10 minuter"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Swe, "1.5 timmar"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_romanian {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Ron, "354 secunde"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Ron, "10 minute"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Ron, "1.5 ore"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_russian {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Rus, "50 секунд"));

                    assert_eq!(got, Some(50));
                }

                #[test]
                fn test_minutes() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Rus, "50 минут"));

                    assert_eq!(got, Some(50 * 60));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Rus, "1.5 часа"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_sinhalese {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Sin, "50 තත්පර"));

                    assert_eq!(got, Some(50));
                }

                #[test]
                fn test_minutes() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Sin, "50 විනාඩි"));

                    assert_eq!(got, Some(50 * 60));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Sin, "1.5 පැය"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_shona {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Sna, "50 masekonzi"));

                    assert_eq!(got, Some(50));
                }

                #[test]
                fn test_minutes() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Sna, "50 maminitsi"));

                    assert_eq!(got, Some(50 * 60));
                }

                #[test]
                fn test_hours() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Sna, "1.5 maawa"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_tagalog {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Tgl, "354 segundo"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Tgl, "10 minuto"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Tgl, "1.5 oras"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_tamil {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Tam, "354 வினாடிகள்"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Tam, "10 நிமிடங்கள்"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Tam, "1.5 மணி நேரம்"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_telugu {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Tel, "354 సెకన్లు"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Tel, "10 నిమిషాలు"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Tel, "1.5 గంటలు"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_thai {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Tha, "354 วินาที"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Tha, "10 นาที"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Tha, "1.5 ชั่วโมง"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_turkmen {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Tuk, "354 sekunt"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Tuk, "10 minut"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Tuk, "1.5 sagat"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_turkish {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Tur, "354 saniye"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Tur, "10 dakika"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Tur, "1.5 saat"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_urdu {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Urd, "354 سیکنڈ"));

                    assert_eq!(got, Some(354));
                }

                #[test]
                fn test_minutes() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Urd, "10 منٹ"));

                    assert_eq!(got, Some(600));
                }

                #[test]
                fn test_hours() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Urd, "1.5  گھنٹے"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_ukrainian {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Ukr, "50 секунд"));

                    assert_eq!(got, Some(50));
                }

                #[test]
                fn test_minutes() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Ukr, "50 хвилин"));

                    assert_eq!(got, Some(50 * 60));
                }

                #[test]
                fn test_hours() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Ukr, "1.5 години"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_uzbek {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Uzb, "50 soniya"));

                    assert_eq!(got, Some(50));
                }

                #[test]
                fn test_minutes() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Uzb, "50 daqiqa"));

                    assert_eq!(got, Some(50 * 60));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Uzb, "1.5 soat"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_vietnamese {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Vie, "50 giây"));

                    assert_eq!(got, Some(50));
                }

                #[test]
                fn test_minutes() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Vie, "50 phút"));

                    assert_eq!(got, Some(50 * 60));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Vie, "1.5 giờ"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_yiddish {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Yid, "50 סעקונדעס"));

                    assert_eq!(got, Some(50));
                }

                #[test]
                fn test_minutes() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Yid, "50 מינוט"));

                    assert_eq!(got, Some(50 * 60));
                }

                #[test]
                fn test_hours() {
                    let got =
                        TimeParser::new().parse_max_time_seconds(&base_text(Lang::Yid, "1.5 שעה"));

                    assert_eq!(got, Some(5400));
                }
            }

            mod tests_zulu {
                use super::*;

                #[test]
                fn test_seconds() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Zul, "50 imizuzwana"));

                    assert_eq!(got, Some(50));
                }

                #[test]
                fn test_minutes() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Zul, "50 amaminithi"));

                    assert_eq!(got, Some(50 * 60));
                }

                #[test]
                fn test_hours() {
                    let got = TimeParser::new()
                        .parse_max_time_seconds(&base_text(Lang::Zul, "1.5 amahora"));

                    assert_eq!(got, Some(5400));
                }
            }
        }
    }
}
