use leptos::{either::Either, prelude::*};
use leptos_router::components::{Outlet, A};

use crate::{components::icons::search::SearchIcon, server_fn::menu::get_menus};
use wasm_bindgen::prelude::*;
use web_sys::wasm_bindgen;
use web_sys::window;
/**
 * 系统头部公共导航部分
 */
#[component]
pub fn HomePage() -> impl IntoView {
    let scrolled = RwSignal::new(false);
    let menu_once = OnceResource::new(get_menus());

    // 监听滚动
    Effect::new(move |_| {
        let Some(win) = window() else { return };
        let win_clone = win.clone();
        let closure = Closure::<dyn FnMut()>::new({
            move || {
                let y = win_clone.scroll_y().unwrap_or(0.0);
                scrolled.set(y > 96.0); // 修改滚动阀值
            }
        });

        win.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
            .unwrap();

        // 防止 closure 被 rust drop回收
        closure.forget();
    });

    view! {
        <div class="relative min-h-screen">

            <div class="absolute h-36 inset-0 -z-10 bg-[url('/images/header_bg.png')] bg-cover bg-top"></div>
            <div class="w-screen h-24">
                <p class="text-5xl text-white pt-2.5 ml-20 flex align-bottom">"Rust in Motion"</p>
                <p class="text-white text-2xl mt-[-15px] ml-20 tracking-wide">"Thoughts on robotics, with safety, systems, and performance in mind."</p>
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
                <div class="mx-auto px-4 h-12 flex items-center justify-between">
                    <div class="w-1/4"></div>
                    <div class="w-2/4 font-dongle font-light text-3xl flex gap-5 justify-left">
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
