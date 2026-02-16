use leptos::prelude::*;

use crate::server_fn::about_me::load_about_me;

#[component]
pub fn AboutMeSector() -> impl IntoView {
    let about_me_resource = OnceResource::new(load_about_me());

    view! {
        <div class="w-full px-4 sm:px-6 lg:w-9/12 lg:px-0 mx-auto flex flex-col justify-start">

            // 顶部区域：Mobile 居中 / PC 左右布局
            <div class="mt-6 flex flex-col gap-4 sm:flex-row sm:justify-between sm:items-center">

                // 左侧：头像 + 名字（移动端居中，PC 靠左）
                <div class="flex flex-col sm:flex-row items-center gap-3 sm:gap-4
                            text-center sm:text-left">
                    <div class="size-24 sm:size-20 overflow-hidden rounded-full bg-green-200 shrink-0">
                        <img src="images/profile.jpg" class="w-full h-full object-cover"/>
                    </div>

                    <div class="flex flex-col justify-center">
                        <div class="text-2xl font-medium">"Bobby Wang"</div>
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

            // 内容区域
            <div class="w-full mt-6 rounded-xl bg-green-200 p-4 sm:p-6">
                "剩余内容"
            </div>
        </div>
    }
}
