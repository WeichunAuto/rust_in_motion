use leptos::{either::Either, prelude::*};
use leptos_router::components::{Outlet, A};

use crate::{components::icons::search::SearchIcon, server_fn::menu::get_menus};

/**
 * 系统头部公共导航部分
 */
#[component]
pub fn HomePage() -> impl IntoView {
    let menu_once = OnceResource::new(get_menus());

    view! {
        <div class="w-screen h-screen bg-amber-300 flex flex-col gap-4 ">
            <div class="w-screen h-32 flex flex-col bg-[url('/images/header_bg.png')]">
                <div class="w-screen h-28"></div>
                <div class="w-screen h-14 flex bg-black/50 text-gray-200 font-light">
                    <div class="w-1/4"></div>
                    <div class="w-2/4 font-dongle font-light text-2xl flex gap-5 justify-left">
                        <Suspense fallback=|| view! { <div>"菜单加载中..."</div> }>
                        {
                            move || {
                                menu_once.get().map(|result| {
                                    match result {
                                        Ok(menus) => Either::Left(view! {
                                            <ForEnumerate
                                                each = move || menus.clone()
                                                key = |menu| menu.id
                                                children = move |_, menu| {
                                                    let href_link = if menu.menu_name == "About Me" {
                                                        "/About_Me".to_string()
                                                    } else {
                                                        format!("/{}", menu.menu_name)
                                                    };
                                                    view! {
                                                        <div class="content-center">
                                                            <A href=href_link>{menu.menu_name}</A>
                                                        </div>
                                                    }
                                                }
                                            />
                                        }),
                                        Err(_) => Either::Right(view! {
                                            <div>"加载菜单错误！"</div>
                                    })
                                    }
                                })
                            }
                        }
                        </Suspense>
                    </div>
                    <div class="w-1/4 flex justify-center items-center">
                        <SearchIcon class="size-5"/>
                    </div>
                </div>
            </div>


            <Outlet/>
        </div>
    }
}
