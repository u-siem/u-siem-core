use crate::prelude::types::LogString;
use crossbeam_channel::Sender;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Serialize, Debug)]
pub enum UpdateI18n {
    Add((LogString, LogString)),
    Remove(LogString),
    Replace(I18nDataset),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialOrd, PartialEq, Ord, Eq)]
pub enum Language {
    AA,
    AB,
    AE,
    AF,
    AK,
    AM,
    AN,
    AR,
    AS,
    AV,
    AY,
    AZ,
    BA,
    BE,
    BG,
    BH,
    BI,
    BM,
    BN,
    BO,
    BR,
    BS,
    CA,
    CE,
    CH,
    CO,
    CR,
    CS,
    CU,
    CV,
    CY,
    DA,
    DE,
    DV,
    DZ,
    EE,
    EL,
    #[default]
    EN,
    EO,
    ES,
    ET,
    EU,
    FA,
    FF,
    FI,
    FJ,
    FO,
    FR,
    FY,
    GA,
    GD,
    GL,
    GN,
    GU,
    GV,
    HA,
    HE,
    HI,
    HO,
    HR,
    HT,
    HU,
    HY,
    HZ,
    IA,
    ID,
    IE,
    IG,
    II,
    IK,
    IO,
    IS,
    IT,
    IU,
    JA,
    JV,
    KA,
    KG,
    KI,
    KJ,
    KK,
    KL,
    KM,
    KN,
    KO,
    KR,
    KS,
    KU,
    KV,
    KW,
    KY,
    LA,
    LB,
    LG,
    LI,
    LN,
    LO,
    LT,
    LU,
    LV,
    MG,
    MH,
    MI,
    MK,
    ML,
    MN,
    MR,
    MS,
    MT,
    MY,
    NA,
    NB,
    ND,
    NE,
    NG,
    NL,
    NN,
    NO,
    NR,
    NV,
    NY,
    OC,
    OJ,
    OM,
    OR,
    OS,
    PA,
    PI,
    PL,
    PS,
    PT,
    QU,
    RM,
    RN,
    RO,
    RU,
    RW,
    SA,
    SC,
    SD,
    SE,
    SG,
    SI,
    SK,
    SL,
    SM,
    SN,
    SO,
    SQ,
    SR,
    SS,
    ST,
    SU,
    SV,
    SW,
    TA,
    TE,
    TG,
    TH,
    TI,
    TK,
    TL,
    TN,
    TO,
    TR,
    TS,
    TT,
    TW,
    TY,
    UG,
    UK,
    UR,
    UZ,
    VE,
    VI,
    VO,
    WA,
    WO,
    XH,
    YI,
    YO,
    ZA,
    ZH,
    ZU,
    Other(&'static str),
}
impl Copy for Language {}

impl From<String> for Language {
    fn from(mut value: String) -> Self {
        value.make_ascii_uppercase();
        match value.as_str() {
            "AA" => Language::AA,
            "AB" => Language::AB,
            "AE" => Language::AE,
            "AF" => Language::AF,
            "AK" => Language::AK,
            "AM" => Language::AM,
            "AN" => Language::AN,
            "AR" => Language::AR,
            "AS" => Language::AS,
            "AV" => Language::AV,
            "AY" => Language::AY,
            "AZ" => Language::AZ,
            "BA" => Language::BA,
            "BE" => Language::BE,
            "BG" => Language::BG,
            "BH" => Language::BH,
            "BI" => Language::BI,
            "BM" => Language::BM,
            "BN" => Language::BN,
            "BO" => Language::BO,
            "BR" => Language::BR,
            "BS" => Language::BS,
            "CA" => Language::CA,
            "CE" => Language::CE,
            "CH" => Language::CH,
            "CO" => Language::CO,
            "CR" => Language::CR,
            "CS" => Language::CS,
            "CU" => Language::CU,
            "CV" => Language::CV,
            "CY" => Language::CY,
            "DA" => Language::DA,
            "DE" => Language::DE,
            "DV" => Language::DV,
            "DZ" => Language::DZ,
            "EE" => Language::EE,
            "EL" => Language::EL,
            "EN" => Language::EN,
            "EO" => Language::EO,
            "ES" => Language::ES,
            "ET" => Language::ET,
            "EU" => Language::EU,
            "FA" => Language::FA,
            "FF" => Language::FF,
            "FI" => Language::FI,
            "FJ" => Language::FJ,
            "FO" => Language::FO,
            "FR" => Language::FR,
            "FY" => Language::FY,
            "GA" => Language::GA,
            "GD" => Language::GD,
            "GL" => Language::GL,
            "GN" => Language::GN,
            "GU" => Language::GU,
            "GV" => Language::GV,
            "HA" => Language::HA,
            "HE" => Language::HE,
            "HI" => Language::HI,
            "HO" => Language::HO,
            "HR" => Language::HR,
            "HT" => Language::HT,
            "HU" => Language::HU,
            "HY" => Language::HY,
            "HZ" => Language::HZ,
            "IA" => Language::IA,
            "ID" => Language::ID,
            "IE" => Language::IE,
            "IG" => Language::IG,
            "II" => Language::II,
            "IK" => Language::IK,
            "IO" => Language::IO,
            "IS" => Language::IS,
            "IT" => Language::IT,
            "IU" => Language::IU,
            "JA" => Language::JA,
            "JV" => Language::JV,
            "KA" => Language::KA,
            "KG" => Language::KG,
            "KI" => Language::KI,
            "KJ" => Language::KJ,
            "KK" => Language::KK,
            "KL" => Language::KL,
            "KM" => Language::KM,
            "KN" => Language::KN,
            "KO" => Language::KO,
            "KR" => Language::KR,
            "KS" => Language::KS,
            "KU" => Language::KU,
            "KV" => Language::KV,
            "KW" => Language::KW,
            "KY" => Language::KY,
            "LA" => Language::LA,
            "LB" => Language::LB,
            "LG" => Language::LG,
            "LI" => Language::LI,
            "LN" => Language::LN,
            "LO" => Language::LO,
            "LT" => Language::LT,
            "LU" => Language::LU,
            "LV" => Language::LV,
            "MG" => Language::MG,
            "MH" => Language::MH,
            "MI" => Language::MI,
            "MK" => Language::MK,
            "ML" => Language::ML,
            "MN" => Language::MN,
            "MR" => Language::MR,
            "MS" => Language::MS,
            "MT" => Language::MT,
            "MY" => Language::MY,
            "NA" => Language::NA,
            "NB" => Language::NB,
            "ND" => Language::ND,
            "NE" => Language::NE,
            "NG" => Language::NG,
            "NL" => Language::NL,
            "NN" => Language::NN,
            "NO" => Language::NO,
            "NR" => Language::NR,
            "NV" => Language::NV,
            "NY" => Language::NY,
            "OC" => Language::OC,
            "OJ" => Language::OJ,
            "OM" => Language::OM,
            "OR" => Language::OR,
            "OS" => Language::OS,
            "PA" => Language::PA,
            "PI" => Language::PI,
            "PL" => Language::PL,
            "PS" => Language::PS,
            "PT" => Language::PT,
            "QU" => Language::QU,
            "RM" => Language::RM,
            "RN" => Language::RN,
            "RO" => Language::RO,
            "RU" => Language::RU,
            "RW" => Language::RW,
            "SA" => Language::SA,
            "SC" => Language::SC,
            "SD" => Language::SD,
            "SE" => Language::SE,
            "SG" => Language::SG,
            "SI" => Language::SI,
            "SK" => Language::SK,
            "SL" => Language::SL,
            "SM" => Language::SM,
            "SN" => Language::SN,
            "SO" => Language::SO,
            "SQ" => Language::SQ,
            "SR" => Language::SR,
            "SS" => Language::SS,
            "ST" => Language::ST,
            "SU" => Language::SU,
            "SV" => Language::SV,
            "SW" => Language::SW,
            "TA" => Language::TA,
            "TE" => Language::TE,
            "TG" => Language::TG,
            "TH" => Language::TH,
            "TI" => Language::TI,
            "TK" => Language::TK,
            "TL" => Language::TL,
            "TN" => Language::TN,
            "TO" => Language::TO,
            "TR" => Language::TR,
            "TS" => Language::TS,
            "TT" => Language::TT,
            "TW" => Language::TW,
            "TY" => Language::TY,
            "UG" => Language::UG,
            "UK" => Language::UK,
            "UR" => Language::UR,
            "UZ" => Language::UZ,
            "VE" => Language::VE,
            "VI" => Language::VI,
            "VO" => Language::VO,
            "WA" => Language::WA,
            "WO" => Language::WO,
            "XH" => Language::XH,
            "YI" => Language::YI,
            "YO" => Language::YO,
            "ZA" => Language::ZA,
            "ZH" => Language::ZH,
            "ZU" => Language::ZU,
            _ => Language::Other("¿?"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct I18nSynDataset {
    dataset: Arc<I18nDataset>,
    comm: Sender<UpdateI18n>,
}
impl I18nSynDataset {
    pub fn new(dataset: Arc<I18nDataset>, comm: Sender<UpdateI18n>) -> Self {
        Self { dataset, comm }
    }
    pub fn empty() -> Self {
        let (sender, _) = crossbeam_channel::bounded(1);
        Self {
            dataset: Arc::new(I18nDataset::new()),
            comm: sender,
        }
    }
    pub fn insert<S>(&self, key: S, data: S)
    where
        S: Into<LogString>,
    {
        let _ = self
            .comm
            .try_send(UpdateI18n::Add((key.into(), data.into())));
    }
    pub fn remove<S>(&self, key: S)
    where
        S: Into<LogString>,
    {
        // Todo: improve with local cache to send retries
        let _ = self.comm.try_send(UpdateI18n::Remove(key.into()));
    }
    pub fn update(&self, data: I18nDataset) {
        // Todo: improve with local cache to send retries
        let _ = self.comm.try_send(UpdateI18n::Replace(data));
    }
    pub fn get(&self, text: &LogString, language: &Language) -> Option<&LogString> {
        // Todo improve with cached content
        self.dataset.get(text, language)
    }
    /// Obtains a text from the language library
    ///
    /// ```rust
    /// use usiem::prelude::i18n::*;
    /// use usiem::prelude::types::LogString;
    /// let mut dataset = I18nDataset::new();
    /// dataset.insert(
    ///     Language::ES,
    ///     LogString::Borrowed("LocalIp"),
    ///     LogString::Borrowed("IP Local"),
    /// );
    /// dataset.insert(
    ///     Language::EN,
    ///     LogString::Borrowed("LocalIp"),
    ///     LogString::Borrowed("Local IP"),
    /// );
    /// assert_eq!(
    ///     dataset.get_or_default("LocalIp", &Language::ES),
    ///     Some(&LogString::Borrowed("IP Local"))
    /// );
    /// assert_eq!(
    ///     dataset.get_or_default("LocalIp", &Language::FR),
    ///     Some(&LogString::Borrowed("Local IP"))
    /// );
    /// ```
    pub fn get_or_default(&self, text: &str, language: &Language) -> Option<&LogString> {
        self.dataset.get_or_default(text, language)
    }
}
#[derive(Serialize, Debug, Default)]
pub struct I18nDataset {
    /// Language -> Text -> Value
    data: BTreeMap<LogString, BTreeMap<Language, LogString>>,
}

impl I18nDataset {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_dictionary<M>(&mut self, language: Language, data: M)
    where
        M: Into<BTreeMap<LogString, LogString>>,
    {
        let new_data: BTreeMap<LogString, LogString> = data.into();
        for (k, value) in new_data {
            if self.data.contains_key(&k) {
                let map = match self.data.get_mut(&k) {
                    Some(v) => v,
                    None => return,
                };
                map.insert(language, value);
            } else {
                let mut map = BTreeMap::new();
                map.insert(language, value);
                self.data.insert(k, map);
            }
        }
    }
    pub fn insert(&mut self, language: Language, text: LogString, value: LogString) {
        if self.data.contains_key(&text) {
            let map = match self.data.get_mut(&text) {
                Some(v) => v,
                None => return,
            };
            map.insert(language, value);
        } else {
            let mut map = BTreeMap::new();
            map.insert(language, value);
            self.data.insert(text, map);
        }
    }
    pub fn get(&self, text: &str, language: &Language) -> Option<&LogString> {
        self.data.get(text)?.get(language)
    }
    pub fn get_or_default(&self, text: &str, language: &Language) -> Option<&LogString> {
        let langs = self.data.get(text)?;

        match langs.get(language) {
            Some(lang) => Some(lang),
            None => {
                match langs.get(&Language::EN) {
                    Some(lang) => Some(lang),
                    None => {
                        // Return first to be found
                        Some(langs.first_key_value()?.1)
                    }
                }
            }
        }
    }
    pub fn internal_ref(&self) -> &BTreeMap<LogString, BTreeMap<Language, LogString>> {
        &self.data
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn should_find_data_in_map() {
        let mut dataset = I18nDataset::new();
        dataset.insert(
            Language::ES,
            LogString::Borrowed("LocalIp"),
            LogString::Borrowed("IP Local"),
        );
        dataset.insert(
            Language::EN,
            LogString::Borrowed("LocalIp"),
            LogString::Borrowed("Local IP"),
        );
        assert_eq!(
            dataset.get_or_default("LocalIp", &Language::ES),
            Some(&LogString::Borrowed("IP Local"))
        );
        assert_eq!(
            dataset.get_or_default("LocalIp", &Language::FR),
            Some(&LogString::Borrowed("Local IP"))
        );
    }
}
