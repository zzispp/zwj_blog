use crate::domain::models::tag::TagType as TagTypeModel;
use serde::{Deserialize, Serialize};

use crate::domain::models::tag::CreateTag;

#[derive(Deserialize, Serialize, Debug)]
pub enum TagType {
    #[serde(alias = "ALL", alias = "All")]
    All,
    #[serde(alias = "BLOG", alias = "Blog")]
    Blog,
    #[serde(alias = "NOTE", alias = "Note")]
    Note,
    #[serde(alias = "SNIPPET", alias = "Snippet")]
    Snippet,
}
impl Into<TagTypeModel> for TagType {
    fn into(self) -> TagTypeModel {
        match self {
            TagType::All => TagTypeModel::All,
            TagType::Blog => TagTypeModel::Blog,
            TagType::Note => TagTypeModel::Note,
            TagType::Snippet => TagTypeModel::Snippet,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CreateTagDTO {
    pub name: String,
    pub slug: String,
    #[serde(rename = "type")]
    pub tag_type: TagType,
    pub icon: Option<String>,
    #[serde(rename = "iconDark")]
    pub icon_dark: Option<String>,
}

impl Into<CreateTag> for CreateTagDTO {
    fn into(self) -> CreateTag {
        CreateTag {
            name: self.name,
            slug: self.slug,
            tag_type: self.tag_type.into(),
            icon: self.icon,
            icon_dark: self.icon_dark,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct UpdateTagDTO {
    pub name: Option<String>,
    pub slug: Option<String>,
    #[serde(rename = "type")]
    pub tag_type: Option<TagType>,
    pub icon: Option<String>,
    #[serde(rename = "iconDark")]
    pub icon_dark: Option<String>,
}

impl Into<crate::domain::models::tag::UpdateTag> for UpdateTagDTO {
    fn into(self) -> crate::domain::models::tag::UpdateTag {
        crate::domain::models::tag::UpdateTag {
            name: self.name,
            slug: self.slug,
            tag_type: self.tag_type.map(|t| t.into()),
            icon: self.icon,
            icon_dark: self.icon_dark,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TagDTO {
    pub id: String,
    pub name: String,
    pub slug: String,
    #[serde(rename = "type")]
    pub tag_type: TagType,
    pub icon: Option<String>,
    #[serde(rename = "iconDark")]
    pub icon_dark: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    // pub blogs: Vec<BlogDTO>,
    // pub notes: Vec<NoteDTO>,
    // pub snippets: Vec<SnippetDTO>,
    #[serde(rename = "_count")]
    pub count: TagCountDTO,
}

#[derive(Debug, Serialize)]
pub struct TagCountDTO {
    pub blogs: i32,
    pub notes: i32,
    pub snippets: i32,
}
