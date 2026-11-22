use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IeeeEvent {
    pub id: String,
    pub title: String,
    pub dates: String,
    pub location: String,
    pub description: String,
    pub details: String,
    pub category: String,
    pub is_upcoming: bool,
    pub image_url: String,
    pub registration_url: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamMember {
    pub id: String,
    pub name: String,
    pub role: String,
    pub image_url: Option<String>,
    pub linkedin: Option<String>,
    pub image_scale: Option<f32>,
    pub image_offset: Option<i32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Committee {
    pub title: String,
    pub members: Vec<TeamMember>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamData {
    pub officers: Vec<TeamMember>,
    pub senior_execs: Vec<TeamMember>,
    pub committees: Vec<Committee>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Blog {
    pub id: String,
    pub title: String,
    pub excerpt: String,
    pub content: String,
    pub date: String,
    pub author: String,
    pub author_role: String,
    pub category: String,
    pub read_time: String,
    pub thumbnail: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryItem {
    pub id: String,
    pub title: String,
    pub date: String,
    pub category: String,
    pub image_url: String,
}
