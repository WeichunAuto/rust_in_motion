use leptos::prelude::*;

use crate::{
    dto::blog_dto::BlogDto,
    server_fn::{blog::InsertBlog, common::markdown_to_html},
};

#[component]
pub fn AddBlog() -> impl IntoView {
    let submit = ServerAction::<InsertBlog>::new();
    let (content, set_content) = create_signal(String::new());

    let rendered_html = move || markdown_to_html(&content.get());

    view! {
        <ActionForm action=submit>
             <div class="min-h-screen bg-white px-6 py-12">

                <div class="max-w-3xl mx-auto space-y-10">

                    {/* 标题 */}
                    <h1 class="text-3xl font-semibold tracking-tight">
                        "Blog 发布"
                    </h1>

                    {/* 博客标题 */}
                    <div class="space-y-2">
                        <label class="block text-sm font-medium text-gray-600">
                            "博客标题"
                        </label>
                        <input
                            type="text"
                            name="blog_dto[blog_title]"
                            class="w-full border-b border-gray-300
                                   px-0 py-2
                                   text-lg
                                   focus:outline-none
                                   focus:border-black
                                   transition"
                        />
                    </div>

                    {/* 博客简介 */}
                    <div class="space-y-2">
                        <label class="block text-sm font-medium text-gray-600">
                            "博客简介"
                        </label>
                        <textarea
                            name="blog_dto[introduction]"
                            rows="3"
                            class="w-full border border-gray-200
                                   rounded-md px-4 py-3
                                   focus:outline-none
                                   focus:ring-1 focus:ring-black
                                   transition resize-none"
                        ></textarea>
                    </div>

                    {/* 分类 + 标签 */}
                    <div class="grid grid-cols-2 gap-6">

                        <div class="space-y-2">
                            <label class="block text-sm font-medium text-gray-600">
                                "分类"
                            </label>
                            <select
                                name="blog_dto[category_id]"
                                class="w-full border border-gray-200
                                       rounded-md px-4 py-2
                                       focus:outline-none
                                       focus:ring-1 focus:ring-black"
                            >
                                <option value="1">"技术"</option>
                                <option value="2">"Rust"</option>
                                <option value="3">"机器人"</option>
                                <option value="4">"随笔"</option>
                            </select>
                        </div>

                        <div class="space-y-2">
                            <label class="block text-sm font-medium text-gray-600">
                                "标签（逗号分隔）"
                            </label>
                            <input
                                type="text"
                                name="blog_dto[tags]"
                                placeholder="rust, leptos, robotics"
                                class="w-full border border-gray-200
                                       rounded-md px-4 py-2
                                       focus:outline-none
                                       focus:ring-1 focus:ring-black"
                            />
                        </div>

                    </div>

                    {/* 封面图片 */}
                    <div class="space-y-2">
                        <label class="block text-sm font-medium text-gray-600">
                            "封面图片"
                        </label>
                        <input
                            type="file"
                            name="cover_image"
                            accept="image/*"
                            class="block w-full text-sm text-gray-600
                                   file:mr-4 file:py-2 file:px-4
                                   file:rounded-md file:border-0
                                   file:text-sm file:font-medium
                                   file:bg-black file:text-white
                                   hover:file:bg-gray-800"
                        />
                    </div>

                    {/* Markdown 正文 */}
                    <div class="space-y-3">
                        <label class="block text-sm font-medium text-gray-600">
                            "Markdown 正文"
                        </label>
                        <textarea
                            name="blog_dto[content]"
                            placeholder="# 开始写作..."
                            class="w-full min-h-[500px]
                                   border border-gray-200
                                   rounded-lg
                                   p-6
                                   font-mono text-sm leading-relaxed
                                   focus:outline-none
                                   focus:ring-1 focus:ring-black
                                   resize-y"
                        ></textarea>
                    </div>

                    {/* 提交按钮 */}
                    <div class="pt-6 border-t border-gray-100">
                        <button
                            type="submit"
                            class="px-10 py-2.5
                                   bg-black text-white
                                   rounded-md
                                   hover:bg-gray-800
                                   transition"
                        >
                            "Publish"
                        </button>
                    </div>

                </div>
            </div>
        </ActionForm>
    }
}
