use leptos::prelude::*;
use leptos_router::{hooks::use_params, params::Params};

use crate::server_fn::{blog::load_blog_by_id, common::render_markdown};

// 博客详情页-博客ID参数
#[derive(Params, PartialEq)]
struct BlogIdParams {
    blog_id: Option<i32>,
}

#[component]
pub fn BlogDetailPage() -> impl IntoView {
    let params = use_params::<BlogIdParams>();
    let blog_id = params
        .read()
        .as_ref()
        .ok()
        .and_then(|p| p.blog_id)
        .unwrap_or_default();

    let blog_resource = OnceResource::new(load_blog_by_id(blog_id));

    view! {
        <div class="w-full">
            <Suspense fallback=move || view! {
                <div class="w-10/12 mx-auto py-20 text-center text-gray-500">
                    "Loading blog..."
                </div>
            }>

                {move || {
                    blog_resource.get().map(|result| match result {
                        Ok(blog) => {

                            let html_content = render_markdown(&blog.get_content());
                            let cover = blog.get_cover_image_url();
                            let mut created_at = blog.get_create_at();
                            // created_at.truncate(11);
                            view! {
                                <article class="w-10/12 md:w-8/12 lg:w-7/12 mx-auto py-10">
                                    <h1 class="text-3xl md:text-4xl font-bold mb-6">
                                        {blog.get_blog_title()}
                                    </h1>

                                    <div class="text-sm text-gray-500 mb-6">
                                        {created_at} " | " {blog.get_read_time()} " min read."
                                    </div>
                                    <img
                                        class="w-full rounded-xl mb-8"
                                        src={cover.unwrap_or_default()}
                                    />

                                    <div
                                        class="prose prose-neutral max-w-none"
                                        inner_html=html_content
                                    >
                                    </div>

                                </article>

                            }.into_any()
                        }

                        Err(_) => {
                            view! {
                                <div class="text-center py-20 text-red-500">
                                    "Failed to load blog"
                                </div>
                            }.into_any()
                        }
                    })
                }}

            </Suspense>
        </div>
    }
}
