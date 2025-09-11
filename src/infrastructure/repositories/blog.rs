use actix_threadpool::run;
use async_trait::async_trait;
use diesel::prelude::*;
use std::sync::Arc;

use crate::{
    domain::{
        models::blog::{Blog, CreateBlog, UpdateBlog},
        models::tag::Tag,
        repositories::{
            blog::{BlogQueryParams, BlogRepository},
            repository::{QueryParams, RepositoryResult, ResultPaging},
        },
    },
    infrastructure::{
        databases::postgresql::DBConn,
        error::DieselRepositoryError,
        models::{
            blog::{BlogDiesel, CreateBlogDiesel},
            tag::TagDiesel,
        },
    },
};

pub struct BlogDieselRepository {
    pub pool: Arc<DBConn>,
}

impl BlogDieselRepository {
    pub fn new(db: Arc<DBConn>) -> Self {
        BlogDieselRepository { pool: db }
    }

    // 辅助方法：加载博客的标签
    async fn load_blog_tags(&self, blog_id: i32) -> RepositoryResult<Vec<Tag>> {
        use crate::infrastructure::schema::{blog_tag_relations, tags};
        let pool = self.pool.clone();

        let result = run(move || {
            let mut conn = pool.get().unwrap();
            tags::table
                .inner_join(blog_tag_relations::table.on(tags::id.eq(blog_tag_relations::tag_id)))
                .filter(blog_tag_relations::blog_id.eq(blog_id))
                .select(tags::all_columns)
                .load::<TagDiesel>(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        Ok(result.into_iter().map(|tag| tag.into()).collect())
    }

    // 辅助方法：设置博客的标签关系
    async fn set_blog_tags(&self, blog_id: i32, tag_ids: &[i32]) -> RepositoryResult<()> {
        use crate::infrastructure::schema::blog_tag_relations;
        let pool = self.pool.clone();
        let tag_ids = tag_ids.to_vec();

        run(move || -> Result<(), diesel::result::Error> {
            let mut conn = pool.get().unwrap();

            // 删除现有关系
            diesel::delete(blog_tag_relations::table)
                .filter(blog_tag_relations::blog_id.eq(blog_id))
                .execute(&mut conn)?;

            // 插入新关系
            for tag_id in tag_ids {
                diesel::insert_into(blog_tag_relations::table)
                    .values((
                        blog_tag_relations::blog_id.eq(blog_id),
                        blog_tag_relations::tag_id.eq(tag_id),
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
impl BlogRepository for BlogDieselRepository {
    async fn create(&self, new_blog: &CreateBlog) -> RepositoryResult<Blog> {
        use crate::infrastructure::schema::blogs::dsl::blogs;
        let new_blog_diesel: CreateBlogDiesel = CreateBlogDiesel::from(new_blog.clone());
        let tag_ids = new_blog.tag_ids.clone();
        let mut conn = self.pool.get().unwrap();

        let result: BlogDiesel = run(move || {
            diesel::insert_into(blogs)
                .values(new_blog_diesel)
                .get_result(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        // 设置标签关系
        if !tag_ids.is_empty() {
            self.set_blog_tags(result.id, &tag_ids).await?;
        }

        // 加载标签并转换为Blog
        let tags = self.load_blog_tags(result.id).await?;
        let mut blog: Blog = result.into();
        blog.tags = tags;

        Ok(blog)
    }

    async fn list(&self, params: BlogQueryParams) -> RepositoryResult<ResultPaging<Blog>> {
        use crate::infrastructure::schema::blogs::dsl::*;
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
                let mut query = blogs.into_boxed();

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
            let mut query = blogs.into_boxed();

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
                .load::<BlogDiesel>(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        // 为每个博客加载标签
        let mut blogs_with_tags = Vec::new();
        for blog_diesel in result {
            let tags = self.load_blog_tags(blog_diesel.id).await?;
            let mut blog: Blog = blog_diesel.into();
            blog.tags = tags;
            blogs_with_tags.push(blog);
        }

        Ok(ResultPaging {
            total,
            items: blogs_with_tags,
        })
    }

    async fn get(&self, blog_id: i32) -> RepositoryResult<Option<Blog>> {
        use crate::infrastructure::schema::blogs::dsl::{blogs, id};
        let pool = self.pool.clone();

        let result = run(move || {
            let mut conn = pool.get().unwrap();
            blogs
                .filter(id.eq(blog_id))
                .first::<BlogDiesel>(&mut conn)
                .optional()
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        if let Some(blog_diesel) = result {
            let tags = self.load_blog_tags(blog_diesel.id).await?;
            let mut blog: Blog = blog_diesel.into();
            blog.tags = tags;
            Ok(Some(blog))
        } else {
            Ok(None)
        }
    }

    async fn get_by_slug(&self, slug_val: &str) -> RepositoryResult<Option<Blog>> {
        use crate::infrastructure::schema::blogs::dsl::{blogs, slug};
        let pool = self.pool.clone();
        let slug_val = slug_val.to_string();

        let result = run(move || {
            let mut conn = pool.get().unwrap();
            blogs
                .filter(slug.eq(slug_val))
                .first::<BlogDiesel>(&mut conn)
                .optional()
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        if let Some(blog_diesel) = result {
            let tags = self.load_blog_tags(blog_diesel.id).await?;
            let mut blog: Blog = blog_diesel.into();
            blog.tags = tags;
            Ok(Some(blog))
        } else {
            Ok(None)
        }
    }

    async fn get_published(&self) -> RepositoryResult<Vec<Blog>> {
        use crate::infrastructure::schema::blogs::dsl::*;
        let pool = self.pool.clone();

        let result = run(move || {
            let mut conn = pool.get().unwrap();
            blogs
                .filter(published.eq(true))
                .order(created_at.desc())
                .load::<BlogDiesel>(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        let mut blogs_with_tags = Vec::new();
        for blog_diesel in result {
            let tags = self.load_blog_tags(blog_diesel.id).await?;
            let mut blog: Blog = blog_diesel.into();
            blog.tags = tags;
            blogs_with_tags.push(blog);
        }

        Ok(blogs_with_tags)
    }

    async fn get_published_by_slug(&self, slug_val: &str) -> RepositoryResult<Option<Blog>> {
        use crate::infrastructure::schema::blogs::dsl::{blogs, published, slug};
        let pool = self.pool.clone();
        let slug_val = slug_val.to_string();

        let result = run(move || {
            let mut conn = pool.get().unwrap();
            blogs
                .filter(slug.eq(slug_val))
                .filter(published.eq(true))
                .first::<BlogDiesel>(&mut conn)
                .optional()
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        if let Some(blog_diesel) = result {
            let tags = self.load_blog_tags(blog_diesel.id).await?;
            let mut blog: Blog = blog_diesel.into();
            blog.tags = tags;
            Ok(Some(blog))
        } else {
            Ok(None)
        }
    }

    async fn update(
        &self,
        blog_id: i32,
        update_blog: &UpdateBlog,
    ) -> RepositoryResult<Option<Blog>> {
        use crate::infrastructure::schema::blogs::dsl::*;
        let pool = self.pool.clone();
        let update_blog = update_blog.clone();

        let result = run(move || {
            let mut conn = pool.get().unwrap();

            // 先获取现有记录
            let existing = match blogs
                .filter(id.eq(blog_id))
                .first::<BlogDiesel>(&mut conn)
                .optional()?
            {
                Some(blog) => blog,
                None => return Ok(None),
            };

            // 准备更新值
            let new_title = update_blog.title.unwrap_or(existing.title);
            let new_slug = update_blog.slug.unwrap_or(existing.slug);
            let new_description = update_blog.description.unwrap_or(existing.description);
            let new_body = update_blog.body.unwrap_or(existing.body);
            let new_cover = update_blog.cover.or(existing.cover);
            let new_author = update_blog.author.or(existing.author);
            let new_published = update_blog.published.unwrap_or(existing.published);

            // 执行更新
            diesel::update(blogs.filter(id.eq(blog_id)))
                .set((
                    title.eq(new_title),
                    slug.eq(new_slug),
                    description.eq(new_description),
                    body.eq(new_body),
                    cover.eq(new_cover),
                    author.eq(new_author),
                    published.eq(new_published),
                ))
                .get_result::<BlogDiesel>(&mut conn)
                .optional()
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        if let Some(blog_diesel) = result {
            // 更新标签关系
            if let Some(tag_ids) = &update_blog.tag_ids {
                self.set_blog_tags(blog_diesel.id, tag_ids).await?;
            }

            let tags = self.load_blog_tags(blog_diesel.id).await?;
            let mut blog: Blog = blog_diesel.into();
            blog.tags = tags;
            Ok(Some(blog))
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, blog_id: i32) -> RepositoryResult<bool> {
        use crate::infrastructure::schema::blogs::dsl::{blogs, id};
        let pool = self.pool.clone();

        let result = run(move || {
            let mut conn = pool.get().unwrap();
            diesel::delete(blogs.filter(id.eq(blog_id))).execute(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        Ok(result > 0)
    }

    async fn exists(&self, blog_id: i32) -> RepositoryResult<bool> {
        use crate::infrastructure::schema::blogs::dsl::{blogs, id};
        let pool = self.pool.clone();

        let result = run(move || {
            let mut conn = pool.get().unwrap();
            diesel::select(diesel::dsl::exists(blogs.filter(id.eq(blog_id))))
                .get_result::<bool>(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        Ok(result)
    }

    async fn toggle_published(&self, blog_id: i32) -> RepositoryResult<Option<Blog>> {
        use crate::infrastructure::schema::blogs::dsl::*;
        let pool = self.pool.clone();

        let result = run(move || {
            let mut conn = pool.get().unwrap();

            // 先获取当前状态
            let current_blog = match blogs
                .filter(id.eq(blog_id))
                .first::<BlogDiesel>(&mut conn)
                .optional()?
            {
                Some(blog) => blog,
                None => return Ok(None),
            };

            // 切换发布状态
            diesel::update(blogs.filter(id.eq(blog_id)))
                .set(published.eq(!current_blog.published))
                .get_result::<BlogDiesel>(&mut conn)
                .optional()
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        if let Some(blog_diesel) = result {
            let tags = self.load_blog_tags(blog_diesel.id).await?;
            let mut blog: Blog = blog_diesel.into();
            blog.tags = tags;
            Ok(Some(blog))
        } else {
            Ok(None)
        }
    }
}
