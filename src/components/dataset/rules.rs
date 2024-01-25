use crate::prelude::rule::SiemRule;
use crate::prelude::types::LogString;
use crossbeam_channel::Sender;
use serde::Serialize;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;

#[derive(Serialize, Debug)]
pub enum UpdateRules {
    Add(SiemRule),
    Remove(LogString),
    Replace(RulesDataset),
}

#[derive(Debug, Clone)]
pub struct CorrelationRulesDataset {
    dataset: Arc<RulesDataset>,
    comm: Sender<UpdateRules>,
}
impl CorrelationRulesDataset {
    pub fn new(dataset: Arc<RulesDataset>, comm: Sender<UpdateRules>) -> Self {
        Self { dataset, comm }
    }
    pub fn insert(&self, rule: SiemRule) {
        let _ = self.comm.send(UpdateRules::Add(rule));
    }
    pub fn insert_timeout(&self, rule: SiemRule, timeout: Duration) -> Result<(), SiemRule> {
        let init = std::time::Instant::now();
        let mut rule = rule;
        loop {
            rule = match self.comm.try_send(UpdateRules::Add(rule)) {
                Ok(_) => return Ok(()),
                Err(e) => match e {
                    crossbeam_channel::TrySendError::Full(r) => extract_rule_from_update(r),
                    crossbeam_channel::TrySendError::Disconnected(r) => extract_rule_from_update(r),
                },
            };
            let now = std::time::Instant::now();
            if now > init + timeout {
                return Err(rule);
            }
        }
    }
    pub fn try_insert(&self, rule: SiemRule) -> Result<(), SiemRule> {
        match self.comm.try_send(UpdateRules::Add(rule)) {
            Ok(_) => Ok(()),
            Err(e) => match e {
                crossbeam_channel::TrySendError::Full(r) => Err(extract_rule_from_update(r)),
                crossbeam_channel::TrySendError::Disconnected(r) => {
                    Err(extract_rule_from_update(r))
                }
            },
        }
    }
    pub fn get(&self, id: &LogString) -> Option<&SiemRule> {
        // Todo improve with cached added IPs
        self.dataset.get(id)
    }
    pub fn apply_updates(&self, updates : Vec<UpdateRules>) -> Self {
        let mut iter = updates.into_iter();
        let first = iter.next().unwrap();
        let mut new = match first {
            UpdateRules::Add(a) => {
                let mut dataset = self.dataset.as_ref().clone();
                dataset.insert(a);
                dataset
            },
            UpdateRules::Remove(v) => {
                let mut dataset = self.dataset.as_ref().clone();
                dataset.remove(&v);
                dataset
            },
            UpdateRules::Replace(v) => v,
        };
        for update in iter {
            match update {
                UpdateRules::Add(a) => {
                    new.insert(a);
                },
                UpdateRules::Remove(v) => {
                    new.remove(&v);
                },
                UpdateRules::Replace(v) => {
                    new = v;
                },
            };
        }
        Self::new(Arc::new(new), self.comm.clone())
    }
}
#[derive(Serialize, Debug, Default, Clone)]
pub struct RulesDataset {
    rules: BTreeMap<LogString, SiemRule>,
}

impl RulesDataset {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert(&mut self, rule: SiemRule) {
        self.rules.insert(rule.name.clone(), rule);
    }
    pub fn get(&self, id: &LogString) -> Option<&SiemRule> {
        self.rules.get(id)
    }
    pub fn remove(&mut self, id: &LogString) {
        self.rules.remove(id);
    }
}

fn extract_rule_from_update(update: UpdateRules) -> SiemRule {
    match update {
        UpdateRules::Add(r) => r,
        UpdateRules::Remove(_) => unreachable!(),
        UpdateRules::Replace(_) => unreachable!(),
    }
}
