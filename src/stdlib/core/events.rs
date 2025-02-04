use std::collections::HashMap;
use crate::core::{Error, Result};

/// Event representation
#[derive(Clone, Debug)]
pub struct Event {
    pub name: String,
    pub topics: Vec<[u8; 32]>,
    pub data: Vec<u8>,
    pub indexed: HashMap<String, Vec<u8>>,
}

impl Event {
    /// Create a new event
    pub fn new(name: impl Into<String>) -> Self {
        Event {
            name: name.into(),
            topics: Vec::new(),
            data: Vec::new(),
            indexed: HashMap::new(),
        }
    }
    
    /// Add a topic
    pub fn add_topic(&mut self, topic: [u8; 32]) {
        self.topics.push(topic);
    }
    
    /// Set event data
    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }
    
    /// Add indexed field
    pub fn add_indexed(&mut self, name: impl Into<String>, value: Vec<u8>) {
        self.indexed.insert(name.into(), value);
    }
    
    /// Get indexed field
    pub fn get_indexed(&self, name: &str) -> Option<&Vec<u8>> {
        self.indexed.get(name)
    }
}

/// Event log for storing events
#[derive(Default)]
pub struct EventLog {
    events: Vec<Event>,
}

impl EventLog {
    /// Create a new event log
    pub fn new() -> Self {
        EventLog {
            events: Vec::new(),
        }
    }
    
    /// Add an event
    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }
    
    /// Get all events
    pub fn events(&self) -> &[Event] {
        &self.events
    }
    
    /// Get events by name
    pub fn get_events(&self, name: &str) -> Vec<&Event> {
        self.events.iter()
            .filter(|e| e.name == name)
            .collect()
    }
    
    /// Clear all events
    pub fn clear(&mut self) {
        self.events.clear();
    }
}

/// Event filter for filtering events
pub struct EventFilter {
    name: Option<String>,
    topics: Vec<Option<[u8; 32]>>,
    indexed: HashMap<String, Vec<u8>>,
}

impl EventFilter {
    /// Create a new event filter
    pub fn new() -> Self {
        EventFilter {
            name: None,
            topics: Vec::new(),
            indexed: HashMap::new(),
        }
    }
    
    /// Filter by event name
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    
    /// Add topic filter
    pub fn topic(mut self, topic: Option<[u8; 32]>) -> Self {
        self.topics.push(topic);
        self
    }
    
    /// Add indexed field filter
    pub fn indexed(mut self, name: impl Into<String>, value: Vec<u8>) -> Self {
        self.indexed.insert(name.into(), value);
        self
    }
    
    /// Apply filter to events
    pub fn filter<'a>(&self, events: &'a [Event]) -> Vec<&'a Event> {
        events.iter()
            .filter(|e| self.matches(e))
            .collect()
    }
    
    /// Check if event matches filter
    fn matches(&self, event: &Event) -> bool {
        // Check name
        if let Some(name) = &self.name {
            if event.name != *name {
                return false;
            }
        }
        
        // Check topics
        for (i, topic) in self.topics.iter().enumerate() {
            if let Some(topic) = topic {
                if event.topics.get(i) != Some(topic) {
                    return false;
                }
            }
        }
        
        // Check indexed fields
        for (name, value) in &self.indexed {
            if event.get_indexed(name) != Some(value) {
                return false;
            }
        }
        
        true
    }
}