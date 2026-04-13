use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use leptos_router::{hooks::use_params, params::Params};

use crate::{
    components::icons::icons::LinkedIcon,
    constant::SITE_URL,
    server_fn::{blog::load_blog_by_id, common::render_markdown},
};
use urlencoding::encode;


// 博客详情页-博客ID参数
#[derive(Params, PartialEq)]
struct BlogIdParams {
    blog_id: Option<i32>,
}

#[component]
pub fn BlogDetailPage() -> impl IntoView {
    let params = use_params::<BlogIdParams>();
    let blog_id = params
        .read_untracked()
        .as_ref()
        .ok()
        .and_then(|p| p.blog_id)
        .unwrap_or_default();

    // 使用 BlockingResource 阻塞 SSR 流式渲染，确保博客数据在 <head> 发出前已就绪
    // 这样 OG meta tags 才能出现在初始 HTML 响应中，LinkedIn 等爬虫才能正确抓取
    let blog_resource = Resource::new_blocking(
        move || blog_id,
        |blog_id| async move { load_blog_by_id(blog_id).await },
    );
    
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

                            // 用 SITE_URL + blog_id 静态拼接页面绝对 URL
                            let og_url = format!("{}/blog_details/{}", SITE_URL, blog.get_id());
                            // LinkedIn 分享按钮 href：直接用已知 URL 结构构造，无需 web_sys
                            // 避免 #[cfg(wasm32)] 导致 SSR/WASM 返回类型不一致引起的水合失败
                            let linkedin_share_url = format!(
                                "https://www.linkedin.com/sharing/share-offsite/?url={}",
                                encode(&og_url)
                            );
                            let og_image = cover.clone()
                                .filter(|s| !s.is_empty())
                                .map(|path| format!("{}{}", SITE_URL, path))
                                .unwrap_or_default();

                            view! {
                                // 动态设置浏览器 Tab 标题为当前文章标题
                                <Title text=blog.get_blog_title() />
                                // Open Graph 标签：供 LinkedIn 等社交平台分享时抓取预览信息
                                // SSR 渲染时这些标签会直接注入 <head>，爬虫无需执行 JS 即可读取
                                <Meta property="og:type" content="article" />
                                <Meta property="og:title" content=blog.get_blog_title() />
                                <Meta property="og:description" content=blog.get_introduction() />
                                <Meta property="og:url" content=og_url />
                                // 封面图为空时不渲染该标签，避免 og:image 指向无效地址
                                {(!og_image.is_empty()).then(|| view! {
                                    <Meta property="og:image" content=og_image />
                                })}

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
                                        class="prose prose-neutral max-w-none
                                            [&_ol]:list-decimal! [&_ol]:pl-6!
                                            [&_ul]:list-disc! [&_ul]:pl-6!
                                            [&_li]:my-1
                                        "
                                        inner_html=html_content
                                    >
                                    </div>

                                    <div class="mt-20 border-gray-200 flex flex-row justify-between items-center">
                                        <div class="text-lg text-gray-500">"Share this article:"</div>
                                        <div class="flex flex-row gap-3 justify-end items-center">
                                            <div>
                                                <a
                                                    href={linkedin_share_url}
                                                    target="_blank"
                                                    rel="noopener noreferrer"
                                                    class="inline-flex items-center gap-2 px-4 py-2 rounded-lg
                                                        bg-blue-500 text-white text-xs font-medium
                                                        hover:bg-[#004182] transition-colors duration-200
                                                        "

                                                >
                                                    <LinkedIcon />
                                                     "LinkedIn"
                                                </a>
                                            </div>

                                        </div>

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
