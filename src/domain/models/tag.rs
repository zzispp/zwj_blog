use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum TagType {
    All,
    Blog,
    Note,
    Snippet,
}
#[derive(Clone, Deserialize, Serialize)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub tag_type: TagType,
    pub icon: Option<String>,
    pub icon_dark: Option<String>,
}

#[derive(Clone)]
pub struct CreateTag {
    pub name: String,
    pub slug: String,
    pub tag_type: TagType,
    pub icon: Option<String>,
    pub icon_dark: Option<String>,
}

#[derive(Clone)]
pub struct UpdateTag {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub tag_type: Option<TagType>,
    pub icon: Option<String>,
    pub icon_dark: Option<String>,
}
