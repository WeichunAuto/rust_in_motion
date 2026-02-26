use leptos::prelude::*;

use crate::{dto::blog_dto::BlogDto, server_fn::blog::load_blogs_by_category};

#[component]
pub fn ListByCategory(selected_category: ReadSignal<i32>) -> impl IntoView {
    // 请求博客数据
    let blog_data_resource = Resource::new(
        move || selected_category.get(),
        |category_id| load_blogs_by_category(category_id),
    );

    // Signal 管理 Vec<BlogDto>
    let (blogs_data, set_blogs_data) = signal(Vec::<BlogDto>::new());

    view! {

        <Suspense fallback=move|| view! {<p>"loading..."</p>}>
        {
            move || {
                if let Some(Ok(blog_vec)) = blog_data_resource.get() {
                    set_blogs_data.set(blog_vec)
                }

                view! {
                    <ForEnumerate
                        each=move || blogs_data.get()
                        key=|blog_dto| blog_dto.get_id()
                        children=|_, blog_dto| {
                            let cover = blog_dto.get_cover_image_url();
                            let title = blog_dto.get_blog_title();
                            let mut create_at = blog_dto.get_create_at().unwrap_or_default();
                            create_at.truncate(16);
                            let is_featured = blog_dto.get_is_featured().unwrap_or_default();
                            view! {
                                <div class="border rounded-lg p-4 shadow-sm flex flex-row gap-4 items-center">
                                    // 左侧 封面
                                    <div>
                                        // 封面图
                                        {
                                            if let Some(url) = cover {
                                                view! {
                                                    <img
                                                        src=url
                                                        class="size-16 object-cover rounded"
                                                    />
                                                }.into_any()
                                            } else {
                                                view! {
                                                    <div class="w-24 h-24 bg-gray-200 flex items-center justify-center rounded">
                                                        "No Image"
                                                    </div>
                                                }.into_any()
                                            }
                                        }
                                    </div>

                                    // 中间侧信息
                                    <div class="flex flex-col flex-1">

                                        <div class="flex items-center gap-2">

                                            <h2 class="text-lg font-semibold">
                                                {title}
                                            </h2>

                                            {
                                                if is_featured {
                                                    view! {
                                                        <span class="px-2 py-0.5 text-xs bg-yellow-400 rounded">
                                                            "Featured"
                                                        </span>
                                                    }.into_any()
                                                } else {
                                                    view! {
                                                        <>
                                                        </>
                                                    }.into_any()
                                                }
                                            }

                                        </div>

                                        <p class="text-sm text-gray-500 mt-1">
                                            {create_at}
                                        </p>
                                    </div>

                                    // 右侧
                                    <div class="w-16 flex flex-col gap-2">
                                        <button class="border text-sm rounded p-1">"删 除"</button>
                                        <button class="border text-sm rounded p-1"
                                            on:click= move |_| {

                                            }
                                        >"置 顶"</button>
                                    </div>

                                </div>
                            }
                        }
                    />
                }
            }
        }

        </Suspense>


    }
}
