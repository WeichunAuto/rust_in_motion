use leptos::prelude::*;

use crate::{components::blog::admin::list_by_category::ListByCategory, server_fn::blog::load_blog_categories};

#[component]
pub fn BlogList() -> impl IntoView {
    // 博客分类
    let blog_category_resourse = OnceResource::new(load_blog_categories());

    // 已选择的博客类型
    let (selected_category, set_selected_category) = signal(1);

    view! {
        <h1>"这是博客列表页"</h1>

        <div class="min-h-screen bg-white px-6 py-12">

            <div class="max-w-3xl mx-auto flex flex-col gap-2">
                <div class="flex flex-row gap-2">
                    <Suspense fallback=move || view! {<div>"category load...."</div>}>
                        {
                            move || match blog_category_resourse.get() {
                                Some(Ok(category_dtos)) => view! {
                                    {
                                        category_dtos.into_iter()
                                        .map(|category_dto| {
                                            let category_id = category_dto.get_id();
                                            let category_name = category_dto.get_category_name();

                                            let base = "px-3 py-1 rounded-sm border cursor-pointer ";
                                            let dynamic_class = move || {
                                                if selected_category.get() == category_id {
                                                    "bg-blue-200"
                                                } else {
                                                    ""
                                                }
                                            };
                                            view! {
                                                <div class={format!("{}{}", base, dynamic_class())}
                                                    on:click = move |_| {
                                                        set_selected_category.set(category_id);
                                                    }
                                                >
                                                    {category_name}
                                                </div>
                                            }
                                        })
                                        .collect_view()
                                    }
                                }.into_any(),
                                Some(Err(_)) => view! {
                                    <div>"DB加载错误"</div>
                                }.into_any(),
                                None => view! {
                                    <div>"加载错误"</div>
                                }.into_any()
                            }
                        }
                    </Suspense>

                </div>

                // 详细列表部分
                <div class="border">
                    <ListByCategory selected_category=selected_category/>
                </div>
            </div>


        </div>
    }
}
