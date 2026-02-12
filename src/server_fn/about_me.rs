use leptos::{
    prelude::{expect_context, ServerFnError},
    server,
};

#[cfg(feature = "ssr")]
use sea_orm::prelude::*;

#[cfg(feature = "ssr")]
use crate::state::app_state::AppState;

/**
 * 更新指定 quez 中的答案
 */
#[server]
pub async fn update_answer_by_quezid(id: i32) -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::entity::question;
        use sea_orm::ActiveValue::Set;

        let state = expect_context::<AppState>();
        let db = state.db();

        let path = format!("article/about_me/answer_for_quez_{}.md", id);
        let content = match read_from_markdown(&path.as_str()) {
            Ok(c) => c,
            Err(e) => {
                return Err(ServerFnError::ServerError(format!(
                    "read md file error: {path}, err={e}"
                )));
            },
        };
        
        // println!("content = {content}");

        // 更新
        let rt = question::Entity::update(question::ActiveModel {
            id: Set(id),
            answer: Set(Some(content)),
            ..Default::default()
        })
        .exec(db)
        .await;

        match rt {
            Ok(user) => {
                // tracing::info!(
                //     "user updated successfully with id = : {:?}, name = : {:?}",
                //     user.id,
                //     user.fullname
                // );
                return Ok(true);
            }
            Err(DbErr::RecordNotUpdated) => {
                // tracing::error!("User id: {} not found", users_dto.id);
                return Ok(false);
            }
            Err(e) => {
                // tracing::error!("error updating user: {:?}", e);
                return Ok(false);
            }
        }
    }

    #[cfg(not(feature = "ssr"))]
    unreachable!("get_users should only run on the server");
}

// cong markdown 读取内容
fn read_from_markdown(path: &str) -> anyhow::Result<String> {
    let content = std::fs::read_to_string(path)?;
    Ok(content)
}
