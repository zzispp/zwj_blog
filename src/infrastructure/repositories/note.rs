use std::sync::Arc;

use actix_threadpool::run;
use async_trait::async_trait;
use diesel::prelude::*;

use crate::{
    domain::{
        models::{
            note::{CreateNote, Note, UpdateNote},
            tag::Tag,
        },
        repositories::{
            note::{NoteQueryParams, NoteRepository},
            repository::{QueryParams, RepositoryResult, ResultPaging},
        },
    },
    infrastructure::{
        databases::postgresql::DBConn,
        error::DieselRepositoryError,
        models::{
            note::{CreateNoteDiesel, NoteDiesel},
            tag::TagDiesel,
        },
    },
};

pub struct NoteDieselRepository {
    pub pool: Arc<DBConn>,
}

impl NoteDieselRepository {
    pub fn new(db: Arc<DBConn>) -> Self {
        NoteDieselRepository { pool: db }
    }

    // 辅助方法：加载笔记的标签
    async fn load_note_tags(&self, note_id: i32) -> RepositoryResult<Vec<Tag>> {
        use crate::infrastructure::schema::{note_tag_relations, tags};
        let pool = self.pool.clone();

        let result = run(move || {
            let mut conn = pool.get().unwrap();
            tags::table
                .inner_join(note_tag_relations::table.on(tags::id.eq(note_tag_relations::tag_id)))
                .filter(note_tag_relations::note_id.eq(note_id))
                .select(tags::all_columns)
                .load::<TagDiesel>(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        Ok(result.into_iter().map(|tag| tag.into()).collect())
    }

    // 辅助方法：设置笔记的标签关系
    async fn set_note_tags(&self, note_id: i32, tag_ids: &[i32]) -> RepositoryResult<()> {
        use crate::infrastructure::schema::note_tag_relations;
        let pool = self.pool.clone();
        let tag_ids = tag_ids.to_vec();

        run(move || -> Result<(), diesel::result::Error> {
            let mut conn = pool.get().unwrap();

            // 删除现有关系
            diesel::delete(note_tag_relations::table)
                .filter(note_tag_relations::note_id.eq(note_id))
                .execute(&mut conn)?;

            // 插入新关系
            for tag_id in tag_ids {
                diesel::insert_into(note_tag_relations::table)
                    .values((
                        note_tag_relations::note_id.eq(note_id),
                        note_tag_relations::tag_id.eq(tag_id),
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
impl NoteRepository for NoteDieselRepository {
    async fn create(&self, new_note: &CreateNote) -> RepositoryResult<Note> {
        use crate::infrastructure::schema::notes::dsl::notes;
        let new_note_diesel: CreateNoteDiesel = CreateNoteDiesel::from(new_note.clone());
        let tag_ids = new_note.tag_ids.clone();
        let mut conn = self.pool.get().unwrap();

        let result: NoteDiesel = run(move || {
            diesel::insert_into(notes)
                .values(new_note_diesel)
                .get_result(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        // 设置标签关系
        if !tag_ids.is_empty() {
            self.set_note_tags(result.id, &tag_ids).await?;
        }

        // 加载标签并转换为Note
        let tags = self.load_note_tags(result.id).await?;
        let mut note: Note = result.into();
        note.tags = tags;

        Ok(note)
    }

    async fn list(&self, params: NoteQueryParams) -> RepositoryResult<ResultPaging<Note>> {
        use crate::infrastructure::schema::notes::dsl::*;
        let pool = self.pool.clone();
        let limit_val = params.limit();
        let offset_val = params.offset();
        let body_filter = params.body.clone();
        let published_filter = params.published;

        // 获取总数
        let total = {
            let pool_clone = pool.clone();
            let body_filter_clone = body_filter.clone();
            run(move || {
                let mut conn = pool_clone.get().unwrap();
                let mut query = notes.into_boxed();

                if let Some(body_val) = body_filter_clone {
                    query = query.filter(body.ilike(format!("%{}%", body_val)));
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
            let mut query = notes.into_boxed();

            if let Some(body_val) = body_filter {
                query = query.filter(body.ilike(format!("%{}%", body_val)));
            }
            if let Some(published_val) = published_filter {
                query = query.filter(published.eq(published_val));
            }

            query
                .order(created_at.desc())
                .limit(limit_val)
                .offset(offset_val)
                .load::<NoteDiesel>(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        // 为每个笔记加载标签
        let mut notes_with_tags = Vec::new();
        for note_diesel in result {
            let tags = self.load_note_tags(note_diesel.id).await?;
            let mut note: Note = note_diesel.into();
            note.tags = tags;
            notes_with_tags.push(note);
        }

        Ok(ResultPaging {
            total,
            items: notes_with_tags,
        })
    }

    async fn get(&self, note_id: i32) -> RepositoryResult<Option<Note>> {
        use crate::infrastructure::schema::notes::dsl::{id, notes};
        let pool = self.pool.clone();

        let result = run(move || {
            let mut conn = pool.get().unwrap();
            notes
                .filter(id.eq(note_id))
                .first::<NoteDiesel>(&mut conn)
                .optional()
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        if let Some(note_diesel) = result {
            let tags = self.load_note_tags(note_diesel.id).await?;
            let mut note: Note = note_diesel.into();
            note.tags = tags;
            Ok(Some(note))
        } else {
            Ok(None)
        }
    }

    async fn get_all(&self) -> RepositoryResult<Vec<Note>> {
        use crate::infrastructure::schema::notes::dsl::*;
        let pool = self.pool.clone();

        let result = run(move || {
            let mut conn = pool.get().unwrap();
            notes.order(created_at.desc()).load::<NoteDiesel>(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        let mut notes_with_tags = Vec::new();
        for note_diesel in result {
            let tags = self.load_note_tags(note_diesel.id).await?;
            let mut note: Note = note_diesel.into();
            note.tags = tags;
            notes_with_tags.push(note);
        }

        Ok(notes_with_tags)
    }

    async fn update(
        &self,
        note_id: i32,
        update_note: &UpdateNote,
    ) -> RepositoryResult<Option<Note>> {
        use crate::infrastructure::schema::notes::dsl::*;
        let pool = self.pool.clone();
        let update_note = update_note.clone();

        let result = run(move || {
            let mut conn = pool.get().unwrap();

            // 先获取现有记录
            let existing = match notes
                .filter(id.eq(note_id))
                .first::<NoteDiesel>(&mut conn)
                .optional()?
            {
                Some(note) => note,
                None => return Ok(None),
            };

            // 准备更新值
            let new_body = update_note.body.unwrap_or(existing.body);
            let new_published = update_note.published.unwrap_or(existing.published);

            // 执行更新
            diesel::update(notes.filter(id.eq(note_id)))
                .set((body.eq(new_body), published.eq(new_published)))
                .get_result::<NoteDiesel>(&mut conn)
                .optional()
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        if let Some(note_diesel) = result {
            // 更新标签关系
            if let Some(tag_ids) = &update_note.tag_ids {
                self.set_note_tags(note_diesel.id, tag_ids).await?;
            }

            let tags = self.load_note_tags(note_diesel.id).await?;
            let mut note: Note = note_diesel.into();
            note.tags = tags;
            Ok(Some(note))
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, note_id: i32) -> RepositoryResult<bool> {
        use crate::infrastructure::schema::notes::dsl::{id, notes};
        let pool = self.pool.clone();

        let result = run(move || {
            let mut conn = pool.get().unwrap();
            diesel::delete(notes.filter(id.eq(note_id))).execute(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        Ok(result > 0)
    }

    async fn exists(&self, note_id: i32) -> RepositoryResult<bool> {
        use crate::infrastructure::schema::notes::dsl::{id, notes};
        let pool = self.pool.clone();

        let result = run(move || {
            let mut conn = pool.get().unwrap();
            diesel::select(diesel::dsl::exists(notes.filter(id.eq(note_id))))
                .get_result::<bool>(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        Ok(result)
    }

    async fn toggle_published(&self, note_id: i32) -> RepositoryResult<Option<Note>> {
        use crate::infrastructure::schema::notes::dsl::*;
        let pool = self.pool.clone();

        let result = run(move || {
            let mut conn = pool.get().unwrap();

            // 先获取当前状态
            let current_note = match notes
                .filter(id.eq(note_id))
                .first::<NoteDiesel>(&mut conn)
                .optional()?
            {
                Some(note) => note,
                None => return Ok(None),
            };

            // 切换发布状态
            diesel::update(notes.filter(id.eq(note_id)))
                .set(published.eq(!current_note.published))
                .get_result::<NoteDiesel>(&mut conn)
                .optional()
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        if let Some(note_diesel) = result {
            let tags = self.load_note_tags(note_diesel.id).await?;
            let mut note: Note = note_diesel.into();
            note.tags = tags;
            Ok(Some(note))
        } else {
            Ok(None)
        }
    }
}
