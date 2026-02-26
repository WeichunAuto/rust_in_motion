use leptos::logging::log;
use leptos::prelude::*;
use leptos_router::components::A;
use web_sys::wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{FileReader, HtmlInputElement};

use crate::server_fn::blog::UploadMarkdownImage;
use crate::{
    dto::blog_dto::BlogDto,
    server_fn::{
        blog::{load_blog_categories, InsertBlog},
        common::markdown_to_html,
    },
};

#[component]
pub fn AddBlog() -> impl IntoView {
    let submit = ServerAction::<InsertBlog>::new();
    // let (content, set_content) = signal(String::new());

    let (cover_base64, set_cover_base64) = signal::<Option<String>>(None);

    let blog_category_resourse = OnceResource::new(load_blog_categories());

    view! {
        <ActionForm action=submit>
             <div class="min-h-screen bg-white px-6 py-12">

                <div class="max-w-3xl mx-auto space-y-10">
                    <div class="w-full flex flex-row justify-between">
                        {/* 标题 */}
                        <h1 class="text-3xl font-semibold tracking-tight">
                            "Blog 发布"
                        </h1>

                        <div class="">
                            <A href="../blog_list">"> 进入博客列表"</A>
                        </div>
                    </div>


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
                                <Suspense fallback=move || view! {<option value="-1">"load...."</option>}>
                                {
                                    move || match blog_category_resourse.get() {
                                        Some(Ok(category_dtos)) => view! {
                                            {
                                                category_dtos.into_iter()
                                                .map(|category_dto| {
                                                    view! {
                                                        <option value={category_dto.get_id()}>{category_dto.get_category_name()}</option>
                                                    }
                                                })
                                                .collect_view()
                                            }
                                        }.into_any(),
                                        Some(Err(_)) => view! {
                                            <option value="-1">"DB加载错误"</option>
                                        }.into_any(),
                                        None => view! {
                                            <option value="-1">"加载错误"</option>
                                        }.into_any()
                                    }
                                }
                                </ Suspense>

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
                        <input type="hidden"
                            name="blog_dto[cover_image_base64]"
                            value=move || cover_base64.get().unwrap_or_default()
                        />
                        <CoverUploader set_base64=set_cover_base64/>

                    </div>

                    {/* 图片上传组件，用于markdown格式的博客内容中的图片 */ }
                    <MarkdownImageUploader />

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

        {
            let message = move || if submit.version().get() > 0 {
                "提交成功了"
            } else {
                ""
            };
            view! {
                <div class="text-green-600">{move || message()}</div>
            }
        }
    }
}

/**
 * 博客封面的上传组件
 */
#[component]
pub fn CoverUploader(set_base64: WriteSignal<Option<String>>) -> impl IntoView {
    let (preview, set_preview) = signal::<Option<String>>(None);

    view! {
        <div class="space-y-3">

            <input
                type="file"
                accept="image/*"
                class="block w-full text-sm text-gray-600
                       file:mr-4 file:py-2 file:px-4
                       file:rounded-md file:border-0
                       file:text-sm file:font-medium
                       file:bg-black file:text-white
                       hover:file:bg-gray-800"
                on:change=move |ev| {
                    let input: HtmlInputElement = event_target(&ev);

                    if let Some(files) = input.files() {
                        if let Some(file) = files.get(0) {

                            let reader = FileReader::new().unwrap();
                            let reader_clone = reader.clone();

                            let set_base64 = set_base64.clone();
                            let set_preview = set_preview.clone();

                            let onload = Closure::wrap(Box::new(move |_e: web_sys::Event| {
                                if let Ok(result) = reader_clone.result() {
                                    if let Some(base64) = result.as_string() {

                                        // 存入外部 signal
                                        set_base64.set(Some(base64.clone()));

                                        // 本地预览
                                        set_preview.set(Some(base64));
                                    }
                                }
                            }) as Box<dyn FnMut(_)>);

                            reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                            reader.read_as_data_url(&file).unwrap();

                            onload.forget();
                        }
                    }
                }
            />

            {
                move || preview.get().map(|img| {
                    view! {
                        <img
                            src={img}
                            class="w-48 h-32 object-cover rounded-md border border-gray-200"
                        />
                    }
                })
            }

        </div>
    }
}

#[component]
pub fn MarkdownImageUploader() -> impl IntoView {
    let upload_action = ServerAction::<UploadMarkdownImage>::new();
    // let (_, set_preview_path) = signal::<Option<String>>(None);

    view! {
        <div class="space-y-3 border p-4 rounded-md">

            <div class="font-medium text-sm">
                "正文图片上传"
            </div>

            <input
                type="file"
                accept="image/*"
                on:change=move |ev| {
                    let input: HtmlInputElement = event_target(&ev);

                    if let Some(files) = input.files() {
                        if let Some(file) = files.get(0) {

                            let reader = FileReader::new().unwrap();
                            let reader_clone = reader.clone();
                            let upload_action = upload_action.clone();

                            let onload = Closure::wrap(Box::new(move |_e: web_sys::Event| {
                                if let Ok(result) = reader_clone.result() {
                                    if let Some(base64) = result.as_string() {
                                        upload_action.dispatch(
                                            UploadMarkdownImage {
                                                base64_data: base64,
                                            }
                                        );
                                    }
                                }
                            }) as Box<dyn FnMut(_)>);

                            reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                            reader.read_as_data_url(&file).unwrap();
                            onload.forget();
                        }
                    }
                }
            />

            {/* 显示返回路径 */}
            {
                move || {
                    upload_action.value().get().map(|res| {
                        match res {
                            Ok(path) => {
                                // set_preview_path.set(Some(path.clone()));

                                view! {
                                    <div class="space-y-2">
                                        <div class="text-green-600 text-sm">
                                            "上传成功"
                                        </div>

                                        <input
                                            type="text"
                                            readonly
                                            value=path.clone()
                                            class="w-full border p-2 text-xs rounded"
                                        />

                                        <div class="text-xs text-gray-500">
                                            "复制后粘贴到 Markdown 中"
                                        </div>
                                    </div>
                                }.into_any()
                            }
                            Err(_) => view! {
                                <div class="text-red-500 text-sm">
                                    "上传失败"
                                </div>
                            }.into_any()
                        }
                    })
                }
            }
        </div>
    }
}
