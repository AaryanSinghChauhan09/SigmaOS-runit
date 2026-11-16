// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-runit: runit component model.
//!
//! Real implementation: runit_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunitItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunitError {
    EmptyRunitItemName,
    DuplicateRunitItem(String),
    EmptyEventSubject,
}

impl fmt::Display for RunitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyRunitItemName => write!(f, "runit_item name must not be empty"),
            Self::DuplicateRunitItem(id) => write!(f, "duplicate runit_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for RunitError {}

pub struct Runit {
    pub name: String,
    runit_items: BTreeMap<String, RunitItem>,
    events: BTreeMap<String, Event>,
}

impl Runit {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            runit_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_runit_item(&mut self, item: RunitItem) -> Result<(), RunitError> {
        if item.name.trim().is_empty() {
            return Err(RunitError::EmptyRunitItemName);
        }
        let id = item.id.clone();
        if self.runit_items.contains_key(&id) {
            return Err(RunitError::DuplicateRunitItem(id.clone()));
        }
        self.runit_items.insert(id, item);
        Ok(())
    }

    pub fn runit_item_count(&self) -> usize {
        self.runit_items.len()
    }

    pub fn get_runit_item(&self, id: &str) -> Option<&RunitItem> {
        self.runit_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), RunitError> {
        if event.subject.trim().is_empty() {
            return Err(RunitError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_runit_item(id: &str, name: &str) -> RunitItem {
        RunitItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Runit::new("Runit");
        assert_eq!(m.name, "Runit");
        assert_eq!(m.runit_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_runit_item() {
        let mut m = Runit::new("Runit");
        m.add_runit_item(sample_runit_item("a1", "Alpha")).unwrap();
        assert_eq!(m.runit_item_count(), 1);
        assert!(m.get_runit_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Runit::new("Runit");
        assert!(m.add_runit_item(sample_runit_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Runit::new("Runit");
        m.add_runit_item(sample_runit_item("x", "One")).unwrap();
        assert!(m.add_runit_item(sample_runit_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Runit::new("Runit");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
