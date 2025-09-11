use crate::domain::models::tag::{CreateTag, Tag, TagType};
use crate::infrastructure::schema::tags;
use chrono::{DateTime, Utc};
use diesel;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct TagDiesel {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub type_: String,
    pub icon: Option<String>,
    pub icon_dark: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Factory method for creating a new TagDiesel from a Tag
impl From<Tag> for TagDiesel {
    fn from(t: Tag) -> Self {
        TagDiesel {
            id: t.id,
            name: t.name,
            slug: t.slug,
            type_: match t.tag_type {
                TagType::All => "ALL".to_string(),
                TagType::Blog => "BLOG".to_string(),
                TagType::Note => "NOTE".to_string(),
                TagType::Snippet => "SNIPPET".to_string(),
            },
            icon: t.icon,
            icon_dark: t.icon_dark,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = tags)]
pub struct CreateTagDiesel {
    pub name: String,
    pub slug: String,
    pub type_: String,
    pub icon: Option<String>,
    pub icon_dark: Option<String>,
}

// Factory method for creating a new Tag from a TagDiesel
impl Into<Tag> for TagDiesel {
    fn into(self) -> Tag {
        Tag {
            id: self.id,
            name: self.name,
            slug: self.slug,
            tag_type: match self.type_.as_str() {
                "ALL" => TagType::All,
                "BLOG" => TagType::Blog,
                "NOTE" => TagType::Note,
                "SNIPPET" => TagType::Snippet,
                _ => TagType::All,
            },
            icon: self.icon,
            icon_dark: self.icon_dark,
        }
    }
}

impl From<CreateTag> for CreateTagDiesel {
    fn from(t: CreateTag) -> Self {
        CreateTagDiesel {
            name: t.name,
            slug: t.slug,
            type_: match t.tag_type {
                TagType::All => "ALL".to_string(),
                TagType::Blog => "BLOG".to_string(),
                TagType::Note => "NOTE".to_string(),
                TagType::Snippet => "SNIPPET".to_string(),
            },
            icon: t.icon,
            icon_dark: t.icon_dark,
        }
    }
}

impl Into<Tag> for CreateTagDiesel {
    fn into(self) -> Tag {
        Tag {
            id: 0,
            name: self.name,
            slug: self.slug,
            tag_type: match self.type_.as_str() {
                "ALL" => TagType::All,
                "BLOG" => TagType::Blog,
                "NOTE" => TagType::Note,
                "SNIPPET" => TagType::Snippet,
                _ => TagType::All,
            },
            icon: self.icon,
            icon_dark: self.icon_dark,
        }
    }
}
