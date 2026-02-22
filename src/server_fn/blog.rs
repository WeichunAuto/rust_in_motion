use leptos::{
    prelude::{expect_context, ServerFnError},
    server,
};

use crate::dto::{blog_category_dto::BlogCategoryDto, blog_dto::BlogDto};

/**
 * 发布博客
 */
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
    unreachable!("insert_blog should only run on the server");
}

/**
 * 加载所有的博客类型
 */
#[server]
pub async fn load_blog_categories() -> Result<Vec<BlogCategoryDto>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use sea_orm::EntityTrait;

        use crate::{entity::prelude::BlogCategory, state::app_state::AppState};

        let state = expect_context::<AppState>();
        let db = state.db();

        let category_model = BlogCategory::find().all(db).await.unwrap();

        let category_dtos = category_model
            .into_iter()
            .map(|category| BlogCategoryDto::new(category.id, category.category_name))
            .collect::<Vec<BlogCategoryDto>>();

        // println!("{:?}", category_dtos);

        Ok(category_dtos)
    }

    #[cfg(not(feature = "ssr"))]
    unreachable!("update_about_page should only run on the server");
}
