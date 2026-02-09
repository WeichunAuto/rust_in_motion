use leptos::{either::Either, prelude::*};
use leptos_router::components::{Outlet, A};

use crate::{components::icons::icons::SearchIcon, dto::menu_dto::MenuDto};

#[component]
pub fn PcHeader(
    menu_once: OnceResource<Result<Vec<MenuDto>, ServerFnError>>,
    scrolled: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <div class="relative min-h-screen">

            <div class="absolute h-36 inset-0 -z-10 bg-[url('/images/header_bg.png')] bg-cover bg-top"></div>
            <div class="w-screen h-24">
                <p class="text-3xl text-white pt-3 ml-20 flex align-bottom">"Rust in Motion"</p>
                <p class="text-white text-lg font-light ml-20 tracking-wide">"Thoughts on robotics, with safety, systems, and performance in mind."</p>
            </div>
                // sticky 导航时改变样式
                <header
                    class=move || {
                        let base = "sticky top-0 z-50 transition-all duration-200";
                        if scrolled.get() {
                            format!("{base} bg-white text-black shadow-md")
                        } else {
                            format!("{base} bg-black/50 text-gray-200 backdrop-blur")
                        }
                    }
                >
                <div class="mx-auto px-4 h-12 flex items-center justify-between overflow-hidden">
                    <div class="w-1/4 pl-8 overflow-hidden">
                        <p
                            class=move || {
                                let base = " text-xl";
                                if scrolled.get() {
                                    format!("{base} font-bold transition-all duration-800 ease-out")
                                } else {
                                    format!("{base} ml-[-160px]")
                                }
                            }
                        >"Rust in Motion"</p>
                    </div>
                    <div class="w-2/4 font-light text-lg flex gap-5 justify-left">
                        <nav class="flex gap-5">
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
                        </nav>
                    </div>

                    // search icon
                    <div class="w-1/4 flex justify-center items-center ">
                    {
                        move || {
                            if scrolled.get() {
                                view! {
                                    <SearchIcon fill="#000000" class="size-5"/>
                                }
                            } else {
                                view! {
                                    <SearchIcon fill="#D6D5D5" class="size-5"/>
                                }
                            }
                        }
                    }
                    </div>
                </div>
                </header>


            <Outlet/>
        </div>
    }
}
