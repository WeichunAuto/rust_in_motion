use leptos::prelude::*;

use crate::dto::blog_dto::BlogDto;

#[component]
pub fn AddOrEditBlog(
    #[prop(default=None)] blog_dto_opt: Option<BlogDto>) -> impl IntoView {
    let title = match blog_dto_opt {
        Some(_) => "博客编辑页面",
        None => "博客新增页面",
    };
    view! {
        <h1>{title}</h1>
    }
}
