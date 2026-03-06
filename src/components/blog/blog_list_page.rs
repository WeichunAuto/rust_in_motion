use leptos::{logging::log, prelude::*};
use leptos_router::components::A;

use crate::{
    components::common::Tag,
    dto::blog_response_dto::BlogResponsetDto,
    server_fn::{blog::load_resblogs_by_category, common::TagType},
};

/**
 * Web 模块组件
 */
#[component]
pub fn BlogListPage(category_id: i32) -> impl IntoView {
    // 请求博客数据
    let blog_data_resource = OnceResource::new(load_resblogs_by_category(category_id));

    view! {
        <Suspense fallback=move || view! {<p>"blog data is loading..."</p>}>
            {
                move || {
                    match blog_data_resource.get() {
                        Some(Ok(blogs_data)) => {
                            // log!("blogs_data={:?}", blogs_data);
                            view! {
                                <div class="max-w-6xl mx-auto px-4 py-8
                                            grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3
                                            gap-8">
                                    <ForEnumerate
                                        each=move || blogs_data.clone()
                                        key=|blog_signal| blog_signal.with_untracked(|blog| blog.get_id())
                                        children=move |_, blog_signal| {
                                            // let blog = blog_signal.get();
                                            let tags = move || blog_signal.get().get_vtags();
                                            let title = move || blog_signal.get().get_blog_title();
                                            let introduction = move || blog_signal.get().get_introduction();
                                            let cover = move || {
                                                blog_signal.get().get_cover_image_url().and_then(|url| {
                                                    if url.is_empty() { None } else { Some(url) }
                                                })
                                            };
                                            let create_at = move || {
                                                let mut create_at = blog_signal.get().get_create_at();
                                                create_at.truncate(16);
                                                create_at

                                            };
                                            let is_featured = move || blog_signal.get().get_is_featured();
                                            view! {
                                                // ===== 单个博客卡片 =====
                                                <div class="bg-white rounded-2xl
                                                            shadow-md hover:shadow-xl
                                                            transition duration-300
                                                            overflow-hidden flex flex-col"

                                                >
                                                <A href={format!("/blog_details/1212")}>
                                                    // 封面图
                                                    {
                                                        move || {
                                                            match cover() {
                                                                Some(url) =>
                                                                    view! {
                                                                        <img
                                                                            src=url
                                                                            class="w-full h-48 object-cover"
                                                                        />
                                                                    }.into_any(),
                                                                None => view! {
                                                                    <img
                                                                        src="images/blog_default.jpg"
                                                                            class="w-full h-48 object-cover"
                                                                    />
                                                                }.into_any()
                                                            }
                                                        }
                                                    }

                                                    // 内容区域
                                                    <div class="px-6 pb-6 pt-3 flex flex-col flex-1">
                                                        <div class="flex flex-row gap-2 mb-2">
                                                        {
                                                            move || {
                                                                tags().into_iter()
                                                                .map(|tag| {
                                                                    // log!("tag: {:?}", tag);
                                                                    view! {
                                                                        <Tag tag_type=TagType::Tag>{tag.clone()}</Tag>
                                                                    }
                                                                })
                                                                .collect_view()
                                                            }
                                                        }
                                                         </div>


                                                        // 标题
                                                        <h2 class="text-lg font-semibold
                                                                    mb-3 hover:text-blue-600
                                                                    transition cursor-pointer">
                                                            {title}
                                                        </h2>

                                                        // 简介
                                                        <p class="text-gray-600 text-sm
                                                                    flex-1 line-clamp-3 break-all">
                                                            {introduction}
                                                        </p>

                                                        // 日期
                                                        <div class="mt-4 text-xs flex flex-row justify-between text-gray-400">
                                                            <div>
                                                                {create_at}
                                                            </div>
                                                            <div>
                                                            // featured 标签
                                                            {
                                                                move || {
                                                                    if is_featured() {
                                                                        view! {
                                                                            <span class="text-xs
                                                                                bg-yellow-100
                                                                                text-yellow-600
                                                                                px-3 py-1
                                                                                rounded-full
                                                                                w-fit mb-3">
                                                                                "Featured"
                                                                            </span>
                                                                        }.into_any()
                                                                    } else {
                                                                        view! { <></> }.into_any()
                                                                    }
                                                                }
                                                            }
                                                            </div>

                                                        </div>
                                                    </div>
                                                    </A>
                                                </div>
                                            }
                                        }
                                    />
                                </div>
                            }.into_any()
                        },
                        Some(Err(_)) => view! {
                            <p>"blog data loaded error."</p>
                        }.into_any(),
                        None => view! {
                            <p>"No blog data."</p>
                        }.into_any()
                    }
                }
            }
        </Suspense>
    }
}
