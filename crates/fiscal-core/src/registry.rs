use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

#[derive(Debug, Clone, Deserialize)]
struct DomainRegistry {
    registry_version: String,
    audited_at: String,
    domain: String,
    rules: Vec<Rule>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct Rule {
    pub id: String,
    pub value: toml::Value,
    pub unit: String,
    pub income_year: u16,
    pub declaration_year: u16,
    pub effective_from: String,
    pub effective_to: String,
    pub checked_at: String,
    pub source_url: String,
    pub legal_basis: String,
}

#[derive(Debug)]
pub struct Registry {
    version: String,
    audited_at: String,
    domains: BTreeSet<String>,
    rules: BTreeMap<String, Rule>,
}

impl Registry {
    fn load() -> Self {
        let documents = [
            include_str!("../../../data/income_tax.toml"),
            include_str!("../../../data/business.toml"),
            include_str!("../../../data/wealth_savings.toml"),
            include_str!("../../../data/property_transmission.toml"),
            include_str!("../../../data/calendar_aids.toml"),
        ];
        let mut version = String::new();
        let mut audited_at = String::new();
        let mut domains = BTreeSet::new();
        let mut rules = BTreeMap::new();

        for document in documents {
            let parsed: DomainRegistry = toml::from_str(document)
                .unwrap_or_else(|error| panic!("invalid embedded fiscal registry: {error}"));
            if version.is_empty() {
                version.clone_from(&parsed.registry_version);
                audited_at.clone_from(&parsed.audited_at);
            }
            assert_eq!(version, parsed.registry_version, "mixed registry versions");
            assert_eq!(audited_at, parsed.audited_at, "mixed audit dates");
            domains.insert(parsed.domain);
            for rule in parsed.rules {
                let id = rule.id.clone();
                assert!(
                    rules.insert(id.clone(), rule).is_none(),
                    "duplicate rule {id}"
                );
            }
        }

        Self {
            version,
            audited_at,
            domains,
            rules,
        }
    }

    pub fn global() -> &'static Self {
        static REGISTRY: OnceLock<Registry> = OnceLock::new();
        REGISTRY.get_or_init(Self::load)
    }

    pub fn version(&self) -> &str {
        &self.version
    }
    pub fn audited_at(&self) -> &str {
        &self.audited_at
    }
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }
    pub fn domain_count(&self) -> usize {
        self.domains.len()
    }

    pub fn rule(&self, id: &str) -> &Rule {
        self.rules
            .get(id)
            .unwrap_or_else(|| panic!("missing fiscal rule {id}"))
    }

    pub fn number(&self, id: &str) -> f64 {
        self.rule(id)
            .value
            .as_float()
            .or_else(|| self.rule(id).value.as_integer().map(|v| v as f64))
            .unwrap_or_else(|| panic!("rule {id} is not numeric"))
    }

    pub fn number_array(&self, id: &str) -> Vec<f64> {
        self.rule(id)
            .value
            .as_array()
            .unwrap_or_else(|| panic!("rule {id} is not an array"))
            .iter()
            .map(|value| {
                value
                    .as_float()
                    .or_else(|| value.as_integer().map(|integer| integer as f64))
                    .unwrap_or_else(|| panic!("rule {id} contains a non-number"))
            })
            .collect()
    }

    pub fn string_array(&self, id: &str) -> Vec<String> {
        self.rule(id)
            .value
            .as_array()
            .unwrap_or_else(|| panic!("rule {id} is not an array"))
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .unwrap_or_else(|| panic!("rule {id} contains a non-string"))
                    .to_owned()
            })
            .collect()
    }

    pub fn sources(&self, ids: &[&str]) -> Vec<Source> {
        let mut seen = BTreeSet::new();
        ids.iter()
            .filter_map(|id| {
                let rule = self.rule(id);
                if !seen.insert(rule.source_url.clone()) {
                    return None;
                }
                Some(Source {
                    rule_id: rule.id.clone(),
                    url: rule.source_url.clone(),
                    legal_basis: rule.legal_basis.clone(),
                    checked_at: rule.checked_at.clone(),
                })
            })
            .collect()
    }

    pub fn all_sources(&self) -> Vec<Source> {
        let ids: Vec<&str> = self.rules.keys().map(String::as_str).collect();
        self.sources(&ids)
    }

    pub fn coverage(&self) -> Coverage {
        let sourced = self
            .rules
            .values()
            .filter(|r| !r.source_url.is_empty() && !r.legal_basis.is_empty())
            .count();
        Coverage {
            domains: self.domain_count(),
            total_rules: self.rule_count(),
            sourced_rules: sourced,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub rule_id: String,
    pub url: String,
    pub legal_basis: String,
    pub checked_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Coverage {
    pub domains: usize,
    pub total_rules: usize,
    pub sourced_rules: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_rule_is_sourced_and_effective() {
        let registry = Registry::global();
        assert_eq!(
            registry.coverage().total_rules,
            registry.coverage().sourced_rules
        );
        for rule in registry.rules.values() {
            assert!(!rule.effective_from.is_empty());
            assert!(!rule.effective_to.is_empty());
            assert!(rule.effective_from <= rule.effective_to);
        }
    }
}
