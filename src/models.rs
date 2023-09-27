use std::collections::HashMap;
use std::fmt::Display;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    NavigateToEpicDetail { epic_id: u32 },
    NavigateToStoryDetail { epic_id: u32, story_id: u32 },
    NavigateToPreviousPage,
    CreateEpic,
    UpdateEpicStatus { epic_id: u32 },
    DeleteEpic { epic_id: u32 },
    CreateStory { epic_id: u32 },
    UpdateStoryStatus {story_id: u32 },
    DeleteStory { epic_id: u32, story_id: u32 },
    Exit,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Open => write!(f, "OPEN"),
            Status::InProgress => write!(f, "IN PROGRESS"),
            Status::Resolved => write!(f, "RESOLVED"),
            Status::Closed => write!(f, "CLOSED"),
        }
    }
}

impl PartialEq for Status {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Status::Open, Status::Open) => true,
            (Status::InProgress, Status::InProgress) => true,
            (Status::Resolved, Status::Resolved) => true,
            (Status::Closed, Status::Closed) => true,
            (_, _) => false
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<u32>,
}


impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::Open,
            stories: Vec::new(),
        }
    }
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Story {
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::Open,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DBState {
    pub last_item_id: u32,
    pub epics: HashMap<u32, Epic>,
    pub stories: HashMap<u32, Story>,
}
