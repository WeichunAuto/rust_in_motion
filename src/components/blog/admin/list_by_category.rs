use leptos::prelude::*;
use reactive_stores::Store;

use crate::{dto::blog_dto::BlogDto, server_fn::blog::load_blogs_by_category};

#[derive(Store, Debug, Clone)]
pub struct Data {
    #[store(key: String = |blog_dto| format!("{}-{}", blog_dto.get_id().unwrap_or_default(), blog_dto.get_create_at().clone()))]
    rows: Vec<BlogDto>,
}

#[component]
pub fn ListByCategory(selected_category: ReadSignal<i32>) -> impl IntoView {
    // 加载博客数据
    let blog_data_resource = Resource::new(
        move || selected_category.get(),
        |category_id| load_blogs_by_category(category_id),
    );

    // Store 管理 Vec<BlogDto>
    let blogs_store = Store::new(Vec::<BlogDto>::new());

    // 更新 Store 值
    Effect::watch(
        move || blog_data_resource.get(),
        move |new_data, _, _| {
            if let Some(Ok(blog_dtos)) = new_data {
                blogs_store.set(blog_dtos.clone());
            }
        },
        false,
    );

    view! {
        <Suspense fallback=move || view! {<p>"Loading..."</p>}>
            {
                match blog_data_resource.get() {
                    Some(Ok(blog_dtos)) => {

                        view! {

                        }.into_any()
                    },
                    Some(Err(e)) => view! {
                        <div class="text-red-500">
                            {format!("加载失败: {:?}", e)}
                        </div>
                    }.into_any(),
                    None => view! {
                        <p>"加载中..."</p>
                    }.into_any(),
                }
            }
        </Suspense>
    }
}
