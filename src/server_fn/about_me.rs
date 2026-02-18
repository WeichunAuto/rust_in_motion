use leptos::{
    prelude::{expect_context, ServerFnError},
    server,
};

#[cfg(feature = "ssr")]
use sea_orm::prelude::*;

use crate::dto::{about_me_dto::AboutMeDto, question_dto::QuestionDto};
#[cfg(feature = "ssr")]
use crate::state::app_state::AppState;

/**
 * 更新指定 quez 中的答案
 */
#[server]
pub async fn update_answer_by_quezid(id: i32) -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::{
            constant::QUEZ_ANSWER_DIR, entity::question, server_fn::common::read_from_markdown,
        };
        use sea_orm::ActiveValue::Set;

        let state = expect_context::<AppState>();
        let db = state.db();

        let path = format!("{}answer_for_quez_{}.md", QUEZ_ANSWER_DIR, id);
        let content = match read_from_markdown(&path.as_str()) {
            Ok(c) => c,
            Err(e) => {
                return Err(ServerFnError::ServerError(format!(
                    "read md file error: {path}, err={e}"
                )));
            }
        };

        // 更新
        let rt = question::Entity::update(question::ActiveModel {
            id: Set(id),
            answer: Set(Some(content)),
            ..Default::default()
        })
        .exec(db)
        .await;

        match rt {
            Ok(_) => {
                return Ok(true);
            }
            Err(DbErr::RecordNotUpdated) => {
                return Ok(false);
            }
            Err(_) => {
                return Ok(false);
            }
        }
    }

    #[cfg(not(feature = "ssr"))]
    unreachable!("update_answer_by_quezid should only run on the server");
}

// 更新 Summary
#[server]
pub async fn update_summary() -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::{
            constant::SUMMARY_DIR, entity::about_me, server_fn::common::read_from_markdown,
        };
        use sea_orm::ActiveValue::Set;

        let state = expect_context::<AppState>();
        let db = state.db();

        let path = format!("{}", SUMMARY_DIR);
        let content = match read_from_markdown(&path.as_str()) {
            Ok(c) => c,
            Err(e) => {
                return Err(ServerFnError::ServerError(format!(
                    "read md file error: {path}, err={e}"
                )));
            }
        };

        // 首先查出第一条记录
        let first_record_opt = about_me::Entity::find().one(db).await?;

        // 再更新第一条记录的 summary 字段
        match first_record_opt {
            Some(first) => {
                let mut active: about_me::ActiveModel = first.into();
                active.summary = Set(content);

                active.update(db).await?;
                return Ok(true);
            }
            None => {
                return Err(ServerFnError::ServerError(format!("no record.")));
            }
        }
    }

    #[cfg(not(feature = "ssr"))]
    unreachable!("update_summary should only run on the server");
}

#[server]
pub async fn update_about_page() -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::{
            constant::ABOUT_PAGE_DIR, entity::about_me, server_fn::common::read_from_markdown,
        };
        use sea_orm::ActiveValue::Set;

        let state = expect_context::<AppState>();
        let db = state.db();

        let path = format!("{}", ABOUT_PAGE_DIR);
        let content = match read_from_markdown(&path.as_str()) {
            Ok(c) => c,
            Err(e) => {
                return Err(ServerFnError::ServerError(format!(
                    "read md file error: {path}, err={e}"
                )));
            }
        };

        // 首先查出第一条记录
        let first_record_opt = about_me::Entity::find().one(db).await?;

        // 再更新第一条记录的 summary 字段
        match first_record_opt {
            Some(first) => {
                let mut active: about_me::ActiveModel = first.into();
                active.about_page = Set(Some(content));

                active.update(db).await?;
                return Ok(true);
            }
            None => {
                return Err(ServerFnError::ServerError(format!("no record.")));
            }
        }
    }

    #[cfg(not(feature = "ssr"))]
    unreachable!("update_about_page should only run on the server");
}

/**
 * 查询出 about me 的 name, summary, 和 问题id
 */
#[server]
pub async fn load_about_me() -> Result<AboutMeDto, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::entity::about_me;

        let state = expect_context::<AppState>();
        let db = state.db();

        // 查出第一条记录
        let first_record_opt = about_me::Entity::find().one(db).await?;
        // println!("first_record_opt = {:?}", first_record_opt);
        // 打包 AboutMeDto
        match first_record_opt {
            Some(first) => {
                return Ok(AboutMeDto::new(
                    first.name,
                    first.summary,
                    first.quez_id,
                    first.about_page,
                ));
            }
            None => {
                return Err(ServerFnError::ServerError(format!("no record.")));
            }
        }
    }

    #[cfg(not(feature = "ssr"))]
    unreachable!("update_summary should only run on the server");
}

/**
 * 根据 IDs 查询出 question
 */
#[server]
pub async fn get_question_by_ids(ids: Vec<i32>) -> Result<Vec<QuestionDto>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use sea_orm::QueryOrder;

        use crate::entity::question;

        let state = expect_context::<AppState>();
        let db = state.db();

        // 根据 ID 查询
        let quez_models = question::Entity::find()
            .filter(question::Column::Id.is_in(ids))
            .order_by_asc(question::Column::Id)
            .all(db)
            .await?;

        let quez_dtos = quez_models
            .into_iter()
            .map(|quez| QuestionDto::new(quez.id, quez.quez, quez.answer))
            .collect::<Vec<QuestionDto>>();

        Ok(quez_dtos)
    }

    #[cfg(not(feature = "ssr"))]
    unreachable!("update_summary should only run on the server");
}
