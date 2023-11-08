use crate::prelude::types::LogString;
use crossbeam_channel::Sender;
use serde::{Serialize, Deserialize};
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
    ES,
    #[default]
    EN,
    FR,
    Other(&'static str)
}
impl Copy for Language {

}

#[derive(Debug, Clone)]
pub struct I18nSynDataset {
    dataset: Arc<I18nDataset>,
    comm: Sender<UpdateI18n>,
}
impl I18nSynDataset {
    pub fn new(dataset: Arc<I18nDataset>, comm: Sender<UpdateI18n>) -> I18nSynDataset {
        return I18nSynDataset { dataset, comm };
    }
    pub fn empty() -> Self {
        let (sender, _) = crossbeam_channel::bounded(1);

        return Self { dataset : Arc::new(I18nDataset::new()), comm : sender };
    }
    pub fn insert<S>(&self, key: S, data: S)
    where
        S: Into<LogString>,
    {
        match self
            .comm
            .try_send(UpdateI18n::Add((key.into(), data.into())))
        {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn remove<S>(&self, key: S)
    where
        S: Into<LogString>,
    {
        // Todo: improve with local cache to send retries
        match self.comm.try_send(UpdateI18n::Remove(key.into())) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn update(&self, data: I18nDataset) {
        // Todo: improve with local cache to send retries
        match self.comm.try_send(UpdateI18n::Replace(data)) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn get(&self, text: &LogString, language : &Language) -> Option<&LogString> {
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
    pub fn get_or_default(&self, text: &str, language : &Language) -> Option<&LogString> {
        self.dataset.get_or_default(text, language)
    }
}
#[derive(Serialize, Debug)]
pub struct I18nDataset {
    /// Language -> Text -> Value
    data: BTreeMap<LogString, BTreeMap<Language, LogString>>,
}

impl I18nDataset {
    pub fn new() -> I18nDataset {
        return I18nDataset {
            data: BTreeMap::new(),
        };
    }
    pub fn set_dictionary<M>(&mut self, language: Language, data: M)
    where
        M: Into<BTreeMap<LogString, LogString>>,
    {
        let new_data : BTreeMap<LogString, LogString> = data.into();
        for (k, value) in new_data {
            if self.data.contains_key(&k) {
                let map = match self.data.get_mut(&k) {
                    Some(v) => v,
                    None => return
                };
                map.insert(language, value);
            }else {
                let mut map = BTreeMap::new();
                map.insert(language, value);
                self.data.insert(k, map);
            }
        }
    }
    pub fn insert(&mut self, language: Language, text: LogString, value : LogString){
        if self.data.contains_key(&text) {
            let map = match self.data.get_mut(&text) {
                Some(v) => v,
                None => return
            };
            map.insert(language, value);
        }else {
            let mut map = BTreeMap::new();
            map.insert(language, value);
            self.data.insert(text, map);
        }
    }
    pub fn get(&self, text: &str, language : &Language) -> Option<&LogString> {
        self.data.get(text)?.get(language)
    }
    pub fn get_or_default(&self, text: &str, language : &Language) -> Option<&LogString> {
        let langs = self.data.get(text)?;

        match langs.get(language){
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
