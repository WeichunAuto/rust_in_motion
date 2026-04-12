use leptos::prelude::*;
use web_sys::window;

use crate::{
    dto::blog_response_dto::BlogResponsetDto,
    server_fn::blog::{delete_blog_by_id, load_resblogs_by_category, toggle_featured_by_id},
};

#[component]
pub fn ListByCategory(selected_category: ReadSignal<i32>) -> impl IntoView {
    // 请求博客数据
    let blog_data_resource =
        Resource::new(move || selected_category.get(), load_resblogs_by_category);

    // Signal 管理 Vec<BlogDto>
    let (blogs_data, set_blogs_data) = signal(Vec::<BlogResponsetDto>::new());

    // 置顶 和 取消置顶 的请求Action
    let toggle_featured_action = Action::new(|input: &(i32, Option<bool>)| {
        let (id, is_featured) = *input;
        async move {
            let result = toggle_featured_by_id(id, is_featured).await;
            result
        }
    });

    // 删除 blog 的请求Action
    let delete_blog_action = Action::new(|blog_id: &i32| {
        let blog_id = *blog_id;
        async move { delete_blog_by_id(blog_id).await }
    });

    // 置顶 或 取消置顶后，更新前端信号
    let update_view_for_featured = move |blog_id: i32, target_featured: bool| {
        // update 是对原始vec的直接修改，不会 clone() vec。 比 map 和 filter 更加高效。
        set_blogs_data.update(|blogs| {
            if let Some(blog_dto) = blogs
                .iter_mut()
                .find(|target_blog| target_blog.get_id() == blog_id)
            {
                blog_dto.set_is_featured(target_featured);
            }
        });
    };

    // 删除一个博客后，更新前端信号
    let update_view_for_delete = move |blog_id| {
        set_blogs_data.update(|blogs| {
            blogs.retain(|blog_dto| blog_dto.get_id() != blog_id);
        });
    };

    // 监听 toggle_featured_action 的结果，避免无限循环
    Effect::watch(
        move || toggle_featured_action.value().get(),
        move |new_value, _old_value, _| {
            if let Some(Ok(Some((blog_id, target_featured)))) = new_value {
                update_view_for_featured(*blog_id, *target_featured);
            }
        },
        false, // 只在变化时执行，第一次不立即执行
    );

    // 监听 delete_blog_action 的执行结果
    Effect::watch(
        move || delete_blog_action.value().get(),
        move |new_value, _old_value, _| {
            if let Some(Ok(blog_id)) = new_value {
                update_view_for_delete(*blog_id);
            }
        },
        false,
    );

    view! {

        <Suspense fallback=move|| view! {<p>"loading..."</p>}>
        {
            move || {
                if let Some(Ok(blogs_signal)) = blog_data_resource.get() {
                    set_blogs_data.set(blogs_signal)
                }

                view! {
                    <ForEnumerate
                        each=move || blogs_data.get()
                        key=|blog| (blog.get_id(), blog.get_is_featured()) // key 使用一个元祖，包含 is_featured, 这样是否置顶变化时,页面UI能够reactive
                       children=move |_, blog| {
                            let id = blog.get_id();
                            let title = blog.get_blog_title();

                            let cover = blog.get_cover_image_url().and_then(|url| {
                                    if url.is_empty() { None } else { Some(url) }
                                });

                            let create_at = {
                                let mut date = blog.get_create_at();
                                date.truncate(16);
                                date
                            };

                            let is_featured = blog.get_is_featured();

                            view! {
                                <div class="border rounded-lg p-4 shadow-sm flex flex-row gap-4 items-center">
                                    // 左侧 封面
                                    <div>
                                        // 封面图，一定要用闭包来确保响应式
                                        // { move ||
                                            {
                                                if let Some(url) = cover {
                                                    view! {
                                                        <img
                                                            src=url
                                                            class="size-16 object-cover rounded"
                                                        />
                                                    }.into_any()
                                                } else {
                                                    let _: () = view! {
                                                        // <div class="w-24 h-24 bg-gray-200 flex items-center justify-center rounded">
                                                        //     "No Image"
                                                        // </div>
                                                        <></>
                                                    };
                                                    ().into_any()
                                                }
                                            }
                                        // }
                                    </div>

                                    // 中间侧信息
                                    <div class="flex flex-col flex-1">

                                        <div class="flex items-center gap-2">

                                            <h2 class="text-lg font-semibold">
                                                {title}
                                                // {move || blog.get_blog_title()}
                                            </h2>

                                            { move || {

                                                if is_featured {
                                                    view! {
                                                        <span class="px-2 py-0.5 text-xs bg-yellow-400 rounded">
                                                            "Featured"
                                                        </span>
                                                    }.into_any()
                                                } else {
                                                    let _: () = view! {
                                                        <>
                                                        </>
                                                    };
                                                    ().into_any()
                                                }
                                                }
                                            }

                                        </div>

                                        <p class="text-sm text-gray-500 mt-1">
                                            {create_at}
                                        </p>
                                    </div>

                                    // 右侧
                                    // { move ||
                                        {
                                            let featured_text = move || if is_featured {
                                                "取消置顶"
                                            } else {
                                                "置 顶"
                                            };

                                            view! {
                                                <div class="flex flex-col gap-2">
                                                    <button class="border text-sm rounded p-2"
                                                        on:click= move |_| {
                                                            if let Some(win) = window() {
                                                                let confirmed = win.confirm_with_message("你确定要 删除 该博客吗？").unwrap_or(false);
                                                                if confirmed {
                                                                    delete_blog_action.dispatch(id);
                                                                }
                                                            }
                                                        }
                                                    >"删 除"</button>
                                                    <button class="border text-sm rounded p-2"
                                                        on:click= move |_| {
                                                            if let Some(win) = window() {
                                                                let confirmed = win.confirm_with_message(format!("你确定要 {} 该博客吗？", featured_text()).as_str()).unwrap_or(false);
                                                                if confirmed {
                                                                    toggle_featured_action.dispatch((id, Some(is_featured)));
                                                                }
                                                            }
                                                        }
                                                    >{featured_text()}</button>
                                                </div>
                                            }.into_any()
                                        }
                                    // }
                                </div>
                            }.into_any()
                        }
                    />
                }.into_any()
            }
        }
        </Suspense>

    }
}
