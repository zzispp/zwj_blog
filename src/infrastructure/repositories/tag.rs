use std::sync::Arc;

use actix_threadpool::run;
use async_trait::async_trait;
use diesel::prelude::*;

use crate::{
    domain::{
        models::tag::{CreateTag, Tag, TagType, UpdateTag},
        repositories::{
            repository::{QueryParams, RepositoryResult, ResultPaging},
            tag::{TagQueryParams, TagRepository},
        },
    },
    infrastructure::{
        databases::postgresql::DBConn,
        error::DieselRepositoryError,
        models::tag::{CreateTagDiesel, TagDiesel},
    },
};

pub struct TagDieselRepository {
    pub pool: Arc<DBConn>,
}

impl TagDieselRepository {
    pub fn new(db: Arc<DBConn>) -> Self {
        TagDieselRepository { pool: db }
    }
}

#[async_trait]
impl TagRepository for TagDieselRepository {
    async fn create(&self, new_tag: &CreateTag) -> RepositoryResult<Tag> {
        use crate::infrastructure::schema::tags::dsl::tags;
        let new_tag_diesel: CreateTagDiesel = CreateTagDiesel::from(new_tag.clone());
        let mut conn = self.pool.get().unwrap();
        let result: TagDiesel = run(move || {
            diesel::insert_into(tags)
                .values(new_tag_diesel)
                .get_result(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        Ok(result.into())
    }

    async fn list(&self, params: TagQueryParams) -> RepositoryResult<ResultPaging<Tag>> {
        use crate::infrastructure::schema::tags::dsl::tags;
        let pool = self.pool.clone();
        let limit = params.limit();
        let offset = params.offset();

        // 获取总记录数
        let total = {
            let pool_clone = pool.clone();
            run(move || {
                let mut conn = pool_clone.get().unwrap();
                tags.count().get_result::<i64>(&mut conn)
            })
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())?
        };

        // 获取分页数据
        let result = run(move || {
            let mut conn = pool.get().unwrap();
            tags.limit(limit)
                .offset(offset)
                .load::<TagDiesel>(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        Ok(ResultPaging {
            total,
            items: result.into_iter().map(|v| v.into()).collect(),
        })
    }

    async fn get(&self, tag_id: i32) -> RepositoryResult<Option<Tag>> {
        use crate::infrastructure::schema::tags::dsl::{id, tags};
        let pool = self.pool.clone();
        let result = run(move || {
            let mut conn = pool.get().unwrap();
            tags.filter(id.eq(tag_id))
                .first::<TagDiesel>(&mut conn)
                .optional()
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        Ok(result.map(|tag| tag.into()))
    }

    async fn update(&self, tag_id: i32, update_tag: &UpdateTag) -> RepositoryResult<Option<Tag>> {
        use crate::infrastructure::schema::tags::dsl::{
            icon, icon_dark, id, name, slug, tags, type_,
        };
        let pool = self.pool.clone();

        // 简单的方式：先获取现有记录，然后用新值替换需要更新的字段
        let name_val = update_tag.name.clone();
        let slug_val = update_tag.slug.clone();
        let type_val = update_tag.tag_type.clone();
        let icon_val = update_tag.icon.clone();
        let icon_dark_val = update_tag.icon_dark.clone();

        let result = run(move || {
            let mut conn = pool.get().unwrap();

            // 先获取现有记录
            let existing = match tags
                .filter(id.eq(tag_id))
                .first::<TagDiesel>(&mut conn)
                .optional()?
            {
                Some(tag) => tag,
                None => return Ok(None),
            };

            // 准备更新值（如果字段为 None 则保持原值）
            let new_name = name_val.unwrap_or(existing.name);
            let new_slug = slug_val.unwrap_or(existing.slug);
            let new_type = type_val
                .map(|t| match t {
                    TagType::All => "ALL".to_string(),
                    TagType::Blog => "BLOG".to_string(),
                    TagType::Note => "NOTE".to_string(),
                    TagType::Snippet => "SNIPPET".to_string(),
                })
                .unwrap_or(existing.type_);
            let new_icon = icon_val.or(existing.icon);
            let new_icon_dark = icon_dark_val.or(existing.icon_dark);

            // 执行更新
            diesel::update(tags.filter(id.eq(tag_id)))
                .set((
                    name.eq(new_name),
                    slug.eq(new_slug),
                    type_.eq(new_type),
                    icon.eq(new_icon),
                    icon_dark.eq(new_icon_dark),
                ))
                .get_result::<TagDiesel>(&mut conn)
                .optional()
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        Ok(result.map(|tag| tag.into()))
    }

    async fn delete(&self, tag_id: i32) -> RepositoryResult<bool> {
        use crate::infrastructure::schema::tags::dsl::{id, tags};
        let pool = self.pool.clone();
        let result = run(move || {
            let mut conn = pool.get().unwrap();
            diesel::delete(tags.filter(id.eq(tag_id))).execute(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        Ok(result > 0)
    }

    async fn exists(&self, tag_id: i32) -> RepositoryResult<bool> {
        use crate::infrastructure::schema::tags::dsl::{id, tags};
        let pool = self.pool.clone();
        let result = run(move || {
            let mut conn = pool.get().unwrap();
            diesel::select(diesel::dsl::exists(tags.filter(id.eq(tag_id))))
                .get_result::<bool>(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        Ok(result)
    }

    async fn get_all(&self, tag_type: Option<TagType>) -> RepositoryResult<Vec<Tag>> {
        use crate::infrastructure::schema::tags::dsl::{tags, type_};
        let pool = self.pool.clone();
        let type_filter = tag_type.map(|t| match t {
            TagType::All => "ALL".to_string(),
            TagType::Blog => "BLOG".to_string(),
            TagType::Note => "NOTE".to_string(),
            TagType::Snippet => "SNIPPET".to_string(),
        });

        let result = run(move || {
            let mut conn = pool.get().unwrap();
            if let Some(filter_type) = type_filter {
                tags.filter(type_.eq(filter_type))
                    .load::<TagDiesel>(&mut conn)
            } else {
                tags.load::<TagDiesel>(&mut conn)
            }
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        Ok(result.into_iter().map(|v| v.into()).collect())
    }
}
