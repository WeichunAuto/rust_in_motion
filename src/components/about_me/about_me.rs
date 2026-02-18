use leptos::prelude::*;

use crate::{
    components::common::Tag,
    dto::about_me_dto::AboutMeDto,
    server_fn::{
        about_me::{get_question_by_ids, load_about_me},
        common::{markdown_to_html, TagType},
    },
};

#[component]
pub fn AboutMeSector() -> impl IntoView {
    // 一次性加载 about me 数据
    let about_me_resource = OnceResource::new(load_about_me());

    view! {
        <div class="w-full px-4 sm:px-6 lg:w-8/12 lg:px-0 mx-auto flex flex-col justify-start">
            // Profile, Summary, and Quez 视图
            <Suspense fallback=move|| view! {<p>"loading..."</p>}>
            {
                move || match about_me_resource.get(){
                    Some(Ok(data)) => view!{<SummaryAndQuezView data=data />}.into_any(),
                    Some(Err(_)) => view! {<p>"unknown"</p>}.into_any(),
                    None => view! {<p>"I'm loading..."</p>}.into_any()
                }
            }
            </Suspense>

            // Like 视图
            <LikeView />

            // Dislike 视图
            <DislikeView />
        </div>
    }
}

/**
 * Like 视图
 */
#[component]
fn LikeView() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-2 p-4 sm:p-6">
            <div class="text-2xl font-bold flex flex-row gap-1">
                <div class="content-center">"Likes"</div>
                <div class="">
                    <img class="size-10" src="/images/like.png"/>
                </div>
            </div>
            <div class="flex flex-row flex-wrap gap-2">
                <Tag>"Spending Time in the Kitchen"</Tag>
                <Tag>"Black Coffee"</Tag>
                <Tag>"Reading"</Tag>
                <Tag>"Meditation"</Tag>
                <Tag>"Jazz Music"</Tag>
                <Tag>"Robotics"</Tag>
                <Tag>"ROS2"</Tag>
                <Tag>"Path Planning"</Tag>
                <Tag>"Building My Physique"</Tag>
                <Tag>"Running"</Tag>
                <Tag>"Hiking"</Tag>
                <Tag>"Building Software to Solve Reall Problems"</Tag>
                <Tag>"Product Thinking"</Tag>
                <Tag>"Inspiring, Visually Striking Donghua with Strong Storytelling"</Tag>
            </div>
        </div>
    }
}

/**
 * DisLike 视图
 */
#[component]
fn DislikeView() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-2 p-4 sm:p-6">
            <div class="text-2xl font-bold flex flex-row gap-1">
                <div class="content-center">"Dislikes"</div>
                <div class="">
                    <img class="size-10" src="/images/dislike.png"/>
                </div>
            </div>
            <div class="flex flex-row flex-wrap gap-2">
                <Tag tag_type=TagType::Dislike>"Overly Processed Food"</Tag>
                <Tag tag_type=TagType::Dislike>"Fizzy Beverages"</Tag>
                <Tag tag_type=TagType::Dislike>"Getting Drunk"</Tag>
                <Tag tag_type=TagType::Dislike>"Procrastination"</Tag>
                <Tag tag_type=TagType::Dislike>"Scrolling on Social Media"</Tag>
                <Tag tag_type=TagType::Dislike>"Products with Poor User Experience"</Tag>
                <Tag>"Robotics"</Tag>
                <Tag>"ROS2"</Tag>
                <Tag>"Path Planning"</Tag>
                <Tag>"Building My Physique"</Tag>
                <Tag>"Running"</Tag>
                <Tag>"Hiking"</Tag>
                <Tag>"Building Software to Solve Reall Problems"</Tag>
                <Tag>"Product Thinking"</Tag>
                <Tag>"Inspiring, Visually Striking Donghua with Strong Storytelling"</Tag>
            </div>
        </div>
    }
}

/**
 * Profile, Summary, and Quez 视图
 */
#[component]
fn SummaryAndQuezView(data: AboutMeDto) -> impl IntoView {
    let quez_ids = data.get_quez_id();

    // 一次性加载 question 数据
    let quez_resource = OnceResource::new(get_question_by_ids(quez_ids));

    view! {
        // 顶部区域：Mobile 居中 / PC 左右布局
        <div class="mt-6 flex flex-col gap-4 sm:flex-row sm:justify-between sm:items-center">

            // 左侧：头像 + 名字（移动端居中，PC 靠左）
            <div class="flex flex-col sm:flex-row items-center gap-3 sm:gap-4
                        text-center sm:text-left">
                <div class="size-24 sm:size-20 overflow-hidden rounded-full bg-green-200 shrink-0">
                    <img src="images/profile.jpg" class="w-full h-full object-cover"/>
                </div>

                <div class="flex flex-col justify-center">
                    <div class="text-2xl font-medium">
                        {data.get_name()}
                    </div>
                    <div class="text-sm text-gray-500">"20 Jan 2026"</div>
                </div>
            </div>

            // 右侧：icon（移动端居中，PC 靠右）
            <div class="flex flex-row gap-4 justify-center sm:justify-end items-center">
                <a href="#" class="p-2 rounded-md hover:bg-gray-100 transition">
                    <img src="images/email_icon.png" class="size-5"/>
                </a>
                <a href="#" class="p-2 rounded-md hover:bg-gray-100 transition">
                    <img src="images/linkedin_icon.png" class="size-5"/>
                </a>
                <a href="#" class="p-2 rounded-md hover:bg-gray-100 transition">
                    <img src="images/github_icon.png" class="size-5"/>
                </a>
            </div>
        </div>

        // Summary 内容区域
        <div class=
        "
        w-full mt-2 rounded-xl p-4 sm:p-6
        [&_ol]:list-decimal! [&_ol]:pl-6!
        [&_ul]:list-disc! [&_ul]:pl-6!
        [&_li]:my-1
        "
        >
            {
                let content = data.get_summary();
                let html = markdown_to_html(&content);
                view! {
                    <div inner_html=html></div>
                }
            }
        </div>

        // Quez 区域
        <div class=
        "
        w-full mt-2 rounded-xl p-4 sm:p-6
        [&_ol]:list-decimal! [&_ol]:pl-6!
        [&_ul]:list-disc! [&_ul]:pl-6!
        [&_li]:my-1
        "
        >
            <div class="flex flex-col gap-6">
                <Suspense fallback=move || view! {<p>"quez loading..."</p>}>
                    {
                        move || match quez_resource.get() {
                            Some(Ok(quez_dtos)) => view! {
                                {
                                    quez_dtos.into_iter()
                                    .map(|quez_dto| {
                                        let content = quez_dto.get_answer().unwrap_or("No writing answer yet.".to_string());
                                        let html = markdown_to_html(&content);
                                        view! {
                                            <div class="flex flex-col gap-2">
                                                // 问题
                                                <div class="text-2xl font-bold">{quez_dto.get_quez()}</div>
                                                // 回答
                                                <div inner_html=html></div>
                                            </div>
                                        }
                                    })
                                    .collect_view()
                                }
                            }.into_any(),
                            Some(Err(_)) => view! {<p class="text-red-500">"quez loading failed"</p>}.into_any(),
                            None => view! {<p>"quez loading..."</p>}.into_any()
                        }
                    }
                </Suspense>
            </div>
        </div>

    }
}
