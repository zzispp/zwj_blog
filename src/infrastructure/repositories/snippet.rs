use std::sync::Arc;

use actix_threadpool::run;
use async_trait::async_trait;
use diesel::prelude::*;

use crate::{
    domain::{
        models::{
            snippet::{CreateSnippet, Snippet, UpdateSnippet},
            tag::Tag,
        },
        repositories::{
            repository::{QueryParams, RepositoryResult, ResultPaging},
            snippet::{SnippetQueryParams, SnippetRepository},
        },
    },
    infrastructure::{
        databases::postgresql::DBConn,
        error::DieselRepositoryError,
        models::{
            snippet::{CreateSnippetDiesel, SnippetDiesel},
            tag::TagDiesel,
        },
    },
};

pub struct SnippetDieselRepository {
    pub pool: Arc<DBConn>,
}

impl SnippetDieselRepository {
    pub fn new(db: Arc<DBConn>) -> Self {
        SnippetDieselRepository { pool: db }
    }

    // 辅助方法：加载代码片段的标签
    async fn load_snippet_tags(&self, snippet_id: i32) -> RepositoryResult<Vec<Tag>> {
        use crate::infrastructure::schema::{snippet_tag_relations, tags};
        let pool = self.pool.clone();

        let result = run(move || {
            let mut conn = pool.get().unwrap();
            tags::table
                .inner_join(
                    snippet_tag_relations::table.on(tags::id.eq(snippet_tag_relations::tag_id)),
                )
                .filter(snippet_tag_relations::snippet_id.eq(snippet_id))
                .select(tags::all_columns)
                .load::<TagDiesel>(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        Ok(result.into_iter().map(|tag| tag.into()).collect())
    }

    // 辅助方法：设置代码片段的标签关系
    async fn set_snippet_tags(&self, snippet_id: i32, tag_ids: &[i32]) -> RepositoryResult<()> {
        use crate::infrastructure::schema::snippet_tag_relations;
        let pool = self.pool.clone();
        let tag_ids = tag_ids.to_vec();

        run(move || -> Result<(), diesel::result::Error> {
            let mut conn = pool.get().unwrap();

            // 删除现有关系
            diesel::delete(snippet_tag_relations::table)
                .filter(snippet_tag_relations::snippet_id.eq(snippet_id))
                .execute(&mut conn)?;

            // 插入新关系
            for tag_id in tag_ids {
                diesel::insert_into(snippet_tag_relations::table)
                    .values((
                        snippet_tag_relations::snippet_id.eq(snippet_id),
                        snippet_tag_relations::tag_id.eq(tag_id),
                    ))
                    .execute(&mut conn)?;
            }
            Ok(())
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        Ok(())
    }
}

#[async_trait]
impl SnippetRepository for SnippetDieselRepository {
    async fn create(&self, new_snippet: &CreateSnippet) -> RepositoryResult<Snippet> {
        use crate::infrastructure::schema::snippets::dsl::snippets;
        let new_snippet_diesel: CreateSnippetDiesel =
            CreateSnippetDiesel::from(new_snippet.clone());
        let tag_ids = new_snippet.tag_ids.clone();
        let mut conn = self.pool.get().unwrap();

        let result: SnippetDiesel = run(move || {
            diesel::insert_into(snippets)
                .values(new_snippet_diesel)
                .get_result(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        // 设置标签关系
        if !tag_ids.is_empty() {
            self.set_snippet_tags(result.id, &tag_ids).await?;
        }

        // 加载标签并转换为Snippet
        let tags = self.load_snippet_tags(result.id).await?;
        let mut snippet: Snippet = result.into();
        snippet.tags = tags;

        Ok(snippet)
    }

    async fn list(&self, params: SnippetQueryParams) -> RepositoryResult<ResultPaging<Snippet>> {
        use crate::infrastructure::schema::snippets::dsl::*;
        let pool = self.pool.clone();
        let limit_val = params.limit();
        let offset_val = params.offset();
        let title_filter = params.title.clone();
        let slug_filter = params.slug.clone();
        let published_filter = params.published;

        // 获取总数
        let total = {
            let pool_clone = pool.clone();
            let title_filter_clone = title_filter.clone();
            let slug_filter_clone = slug_filter.clone();
            run(move || {
                let mut conn = pool_clone.get().unwrap();
                let mut query = snippets.into_boxed();

                if let Some(title_val) = title_filter_clone {
                    query = query.filter(title.ilike(format!("%{}%", title_val)));
                }
                if let Some(slug_val) = slug_filter_clone {
                    query = query.filter(slug.eq(slug_val));
                }
                if let Some(published_val) = published_filter {
                    query = query.filter(published.eq(published_val));
                }

                query.count().get_result::<i64>(&mut conn)
            })
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())?
        };

        // 获取分页数据
        let result = run(move || {
            let mut conn = pool.get().unwrap();
            let mut query = snippets.into_boxed();

            if let Some(title_val) = title_filter {
                query = query.filter(title.ilike(format!("%{}%", title_val)));
            }
            if let Some(slug_val) = slug_filter {
                query = query.filter(slug.eq(slug_val));
            }
            if let Some(published_val) = published_filter {
                query = query.filter(published.eq(published_val));
            }

            query
                .order(created_at.desc())
                .limit(limit_val)
                .offset(offset_val)
                .load::<SnippetDiesel>(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        // 为每个代码片段加载标签
        let mut snippets_with_tags = Vec::new();
        for snippet_diesel in result {
            let tags = self.load_snippet_tags(snippet_diesel.id).await?;
            let mut snippet: Snippet = snippet_diesel.into();
            snippet.tags = tags;
            snippets_with_tags.push(snippet);
        }

        Ok(ResultPaging {
            total,
            items: snippets_with_tags,
        })
    }

    async fn get(&self, snippet_id: i32) -> RepositoryResult<Option<Snippet>> {
        use crate::infrastructure::schema::snippets::dsl::{id, snippets};
        let pool = self.pool.clone();

        let result = run(move || {
            let mut conn = pool.get().unwrap();
            snippets
                .filter(id.eq(snippet_id))
                .first::<SnippetDiesel>(&mut conn)
                .optional()
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        if let Some(snippet_diesel) = result {
            let tags = self.load_snippet_tags(snippet_diesel.id).await?;
            let mut snippet: Snippet = snippet_diesel.into();
            snippet.tags = tags;
            Ok(Some(snippet))
        } else {
            Ok(None)
        }
    }

    async fn get_by_slug(&self, slug_val: &str) -> RepositoryResult<Option<Snippet>> {
        use crate::infrastructure::schema::snippets::dsl::{slug, snippets};
        let pool = self.pool.clone();
        let slug_val = slug_val.to_string();

        let result = run(move || {
            let mut conn = pool.get().unwrap();
            snippets
                .filter(slug.eq(slug_val))
                .first::<SnippetDiesel>(&mut conn)
                .optional()
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        if let Some(snippet_diesel) = result {
            let tags = self.load_snippet_tags(snippet_diesel.id).await?;
            let mut snippet: Snippet = snippet_diesel.into();
            snippet.tags = tags;
            Ok(Some(snippet))
        } else {
            Ok(None)
        }
    }

    async fn get_published(&self) -> RepositoryResult<Vec<Snippet>> {
        use crate::infrastructure::schema::snippets::dsl::*;
        let pool = self.pool.clone();

        let result = run(move || {
            let mut conn = pool.get().unwrap();
            snippets
                .filter(published.eq(true))
                .order(created_at.desc())
                .load::<SnippetDiesel>(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        let mut snippets_with_tags = Vec::new();
        for snippet_diesel in result {
            let tags = self.load_snippet_tags(snippet_diesel.id).await?;
            let mut snippet: Snippet = snippet_diesel.into();
            snippet.tags = tags;
            snippets_with_tags.push(snippet);
        }

        Ok(snippets_with_tags)
    }

    async fn update(
        &self,
        snippet_id: i32,
        update_snippet: &UpdateSnippet,
    ) -> RepositoryResult<Option<Snippet>> {
        use crate::infrastructure::schema::snippets::dsl::*;
        let pool = self.pool.clone();
        let update_snippet = update_snippet.clone();

        let result = run(move || {
            let mut conn = pool.get().unwrap();

            // 先获取现有记录
            let existing = match snippets
                .filter(id.eq(snippet_id))
                .first::<SnippetDiesel>(&mut conn)
                .optional()?
            {
                Some(snippet) => snippet,
                None => return Ok(None),
            };

            // 准备更新值
            let new_title = update_snippet.title.unwrap_or(existing.title);
            let new_slug = update_snippet.slug.unwrap_or(existing.slug);
            let new_description = update_snippet.description.unwrap_or(existing.description);
            let new_body = update_snippet.body.unwrap_or(existing.body);
            let new_published = update_snippet.published.unwrap_or(existing.published);

            // 执行更新
            diesel::update(snippets.filter(id.eq(snippet_id)))
                .set((
                    title.eq(new_title),
                    slug.eq(new_slug),
                    description.eq(new_description),
                    body.eq(new_body),
                    published.eq(new_published),
                ))
                .get_result::<SnippetDiesel>(&mut conn)
                .optional()
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        if let Some(snippet_diesel) = result {
            // 更新标签关系
            if let Some(tag_ids) = &update_snippet.tag_ids {
                self.set_snippet_tags(snippet_diesel.id, tag_ids).await?;
            }

            let tags = self.load_snippet_tags(snippet_diesel.id).await?;
            let mut snippet: Snippet = snippet_diesel.into();
            snippet.tags = tags;
            Ok(Some(snippet))
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, snippet_id: i32) -> RepositoryResult<bool> {
        use crate::infrastructure::schema::snippets::dsl::{id, snippets};
        let pool = self.pool.clone();

        let result = run(move || {
            let mut conn = pool.get().unwrap();
            diesel::delete(snippets.filter(id.eq(snippet_id))).execute(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        Ok(result > 0)
    }

    async fn exists(&self, snippet_id: i32) -> RepositoryResult<bool> {
        use crate::infrastructure::schema::snippets::dsl::{id, snippets};
        let pool = self.pool.clone();

        let result = run(move || {
            let mut conn = pool.get().unwrap();
            diesel::select(diesel::dsl::exists(snippets.filter(id.eq(snippet_id))))
                .get_result::<bool>(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        Ok(result)
    }

    async fn toggle_published(&self, snippet_id: i32) -> RepositoryResult<Option<Snippet>> {
        use crate::infrastructure::schema::snippets::dsl::*;
        let pool = self.pool.clone();

        let result = run(move || {
            let mut conn = pool.get().unwrap();

            // 先获取当前状态
            let current_snippet = match snippets
                .filter(id.eq(snippet_id))
                .first::<SnippetDiesel>(&mut conn)
                .optional()?
            {
                Some(snippet) => snippet,
                None => return Ok(None),
            };

            // 切换发布状态
            diesel::update(snippets.filter(id.eq(snippet_id)))
                .set(published.eq(!current_snippet.published))
                .get_result::<SnippetDiesel>(&mut conn)
                .optional()
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        if let Some(snippet_diesel) = result {
            let tags = self.load_snippet_tags(snippet_diesel.id).await?;
            let mut snippet: Snippet = snippet_diesel.into();
            snippet.tags = tags;
            Ok(Some(snippet))
        } else {
            Ok(None)
        }
    }
}
