use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};

use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes},
    path,
};

use crate::components::{
    about_me::admin::{
        update_about_page::UpdateAboutPage, update_quez::UpdateQuez, update_summary::UpdateSummary,
    },
    header::{headers::Headers, sectors::Sectors},
};
use leptos::ev::resize;
use web_sys::window;
use web_sys::MediaQueryList;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // 监听是否是移动屏幕
    let (is_mobile, set_is_mobile) = signal(false);
    Effect::new(move || {
        let Some(win) = window() else { return };
        // 创建 matchMedia
        let mql: MediaQueryList = win.match_media("(max-width: 640px)").unwrap().unwrap();
        // 先设置一次初始值
        set_is_mobile.set(mql.matches());

        // 只要 resize 就重新判断 matches()
        window_event_listener(resize, move |_| {
            set_is_mobile.set(mql.matches());
        });
    });

    // 将是否移动屏幕，放入全局context中去。
    provide_context(is_mobile);

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/rust_in_motion.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    // 文章编辑的路由，仅开放给管理员
                    <ParentRoute path=path!("/rust_in_motion/article/admin/about_me") view=move || view! {
                        <div>
                            <h1 class="font-bold text-6xl w-screen text-center">"闲人勿进！"</h1>
                            <Outlet/>
                        </div>
                    }>
                        <Route path=path!("summary") view=UpdateSummary/>
                        <Route path=path!("quez") view=UpdateQuez/>
                        <Route path=path!("about_this_page") view=UpdateAboutPage/>

                    </ParentRoute>

                    // 用户访问路由
                    <ParentRoute path=path!("/") view=Headers>
                        <Route path=path!(":sector") view=Sectors/>
                        <Route path=path!("") view= move || view! {<h2>"Can not match any route."</h2>}/>
                    </ParentRoute>
                </Routes>
            </main>
        </Router>

        // footer down part
        <div class="text-xs sm:text-sm font-light mt-20 mb-6 text-center text-gray-400 w-full">"© 2025 Bobby Wang. All rights reserved."</div>
    }
}
