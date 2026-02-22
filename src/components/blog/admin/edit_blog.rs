use leptos::prelude::*;

use crate::components::blog::admin::add_or_edit_blog::AddOrEditBlog;

#[component]
pub fn EditBlog() -> impl IntoView {
    view! {
        <AddOrEditBlog blog_dto_opt=None />
    }
}
