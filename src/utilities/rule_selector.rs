use std::collections::{BTreeMap, BTreeSet};

/**
 * Simple InvertedIndex to find rule names based on a rule description.
 * Useful with IPS rules that need to be translated into a more generic type like Virus, Malware, XSS, SQLi...
 */
#[derive(Default)]
pub struct RuleSelector {
    inverted_index: BTreeMap<&'static str, BTreeSet<&'static str>>,
    rule_list: BTreeSet<&'static str>,
}

impl RuleSelector {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add(&mut self, rule_name: &'static str, rule_description: &'static str) {
        let words: Vec<&'static str> = rule_description
            .split(|c: char| {
                !(c.is_ascii_digit()
                    || c.is_ascii_lowercase()
                    || c.is_ascii_uppercase()
                    || c == '-'
                    || c == '_')
            })
            .collect();
        self.rule_list.insert(rule_name);
        for word in words {
            if let Some(rule_list) = self.inverted_index.get_mut(word) {
                rule_list.insert(rule_name);
            } else {
                let mut rule_list = BTreeSet::new();
                rule_list.insert(rule_name);
                self.inverted_index.insert(word, rule_list);
            }
        }
    }

    pub fn search(&self, possible_rule: &str) -> Option<&'static str> {
        let words: Vec<&str> = possible_rule
            .split(|c: char| {
                !(c.is_ascii_digit()
                    || c.is_ascii_lowercase()
                    || c.is_ascii_uppercase()
                    || c == '-'
                    || c == '_')
            })
            .collect();
        let mut score_words: BTreeMap<&'static str, u32> = BTreeMap::new();
        for rule in &self.rule_list {
            score_words.insert(rule, 0);
        }
        for word in words {
            if let Some(rule_list) = self.inverted_index.get(word) {
                for rule in rule_list {
                    if let Some(rule_score) = score_words.get_mut(rule) {
                        *rule_score += 1;
                    }
                }
            }
        }
        let mut max_score = 0;
        let mut rule_name = "";
        for (rule, score) in score_words {
            if score > max_score {
                max_score = score;
                rule_name = rule;
            }
        }
        if max_score > 0 {
            Some(rule_name)
        } else {
            None
        }
    }
}
