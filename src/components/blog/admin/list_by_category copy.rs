use leptos::{logging::log, prelude::*};
use web_sys::window;

use crate::{
    dto::blog_dto::BlogDto,
    server_fn::blog::{load_blogs_by_category, toggle_featured_by_id},
};

#[component]
pub fn ListByCategory(selected_category: ReadSignal<i32>) -> impl IntoView {
    // 请求博客数据
    let blog_data_resource = Resource::new(
        move || selected_category.get(),
        |category_id| load_blogs_by_category(category_id),
    );

    // 置顶 和 取消置顶 的请求Action
    let toggle_featured_action = Action::new(|input: &(i32, Option<bool>)| {
        let (id, is_featured) = *input;
        async move {
            let result = toggle_featured_by_id(id, is_featured).await;
            result
        }
    });

    view! {

        

    }
}
