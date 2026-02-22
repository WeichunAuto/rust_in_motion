use leptos::{
    prelude::{expect_context, ServerFnError},
    server,
};

use crate::dto::blog_dto::BlogDto;

#[server]
pub async fn insert_blog(blog_dto: BlogDto) -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::state::app_state::AppState;

        let state = expect_context::<AppState>();
        let db = state.db();

        Ok(true)
        
    }

    #[cfg(not(feature = "ssr"))]
    unreachable!("update_about_page should only run on the server");
}
