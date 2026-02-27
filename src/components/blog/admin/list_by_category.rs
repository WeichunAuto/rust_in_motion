use leptos::prelude::*;
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

    // Signal 管理 Vec<BlogDto>
    let (blogs_data, set_blogs_data) = signal(Vec::<BlogDto>::new());

    // 置顶 和 取消置顶 的请求Action
    let toggle_featured_action = Action::new(|input: &(i32, Option<bool>)| {
        let (id, is_featured) = *input;
        async move { toggle_featured_by_id(id, is_featured).await }
    });

    // let handle_featured_toggle = move |id: i32, is_featured: bool| {
    //     spawn_local(async move {
    //         if let Some(win) = window() {
    //             let confirmed = win.confirm_with_message("你确定要置顶该博客吗？").unwrap_or(false);
    //             if confirmed {
    //                 // action.dispatch((id, Some(is_featured)));
    //                 let _ = toggle_featured_by_id(id, Some(is_featured)).await;
    //             }
    //         }
    //     });
    // };

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
                        children=move |_, blog_dto| {
                            let id = blog_dto.get_id().unwrap_or_default();
                            let cover = match blog_dto.get_cover_image_url() {
                                Some(url) => {
                                    if url=="" { // 如果url为空串，则返回None,下方的view里会判断为无图片
                                        None
                                    } else {
                                        Some(url)
                                    }
                                },
                                None => None
                            };
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
                                                    // <div class="w-24 h-24 bg-gray-200 flex items-center justify-center rounded">
                                                    //     "No Image"
                                                    // </div>
                                                    <></>
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
                                    {
                                        let featured_text = if is_featured {
                                            "取消置顶"
                                        } else {
                                            "置 顶"
                                        };

                                        view! {
                                            <div class="flex flex-col gap-2">
                                                <button class="border text-sm rounded p-2">"删 除"</button>
                                                <button class="border text-sm rounded p-2"
                                                    on:click= move |_| {
                                                        if let Some(win) = window() {
                                                            let confirmed = win.confirm_with_message(format!("你确定要 {} 该博客吗？", featured_text).as_str()).unwrap_or(false);
                                                            if confirmed {
                                                                toggle_featured_action.dispatch((id, Some(is_featured)));
                                                            }
                                                        }
                                                    }
                                                >{featured_text}</button>
                                            </div>
                                        }
                                    }
                                    

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
