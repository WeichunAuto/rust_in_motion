use crate::{
    components::icons::icons::{CloseIcon, MenuFolderIcon, SearchIcon},
    dto::menu_dto::MenuDto,
};
use leptos::{either::Either, prelude::*};
use leptos_router::components::{Outlet, A};

#[component]
pub fn MobileHeader(menu_once: OnceResource<Result<Vec<MenuDto>, ServerFnError>>) -> impl IntoView {
    // 控制移动屏幕上的菜单开关
    let is_menu_open = RwSignal::new(false);

    view! {
        <div class="relative min-h-screen">
                // sticky 导航时改变样式
            <header class="sticky top-0 z-50 bg-white text-black shadow-md">
                <div class="mx-auto px-4 h-12 flex items-center justify-between">
                    <div class="w-1/4"></div>
                    <div class="w-2/4 font-light text-3xl flex gap-5 justify-left">

                    </div>

                    // search icon
                    <div class="w-1/4 flex justify-between gap-4 items-center">

                        <SearchIcon fill="#000000" class="size-5"/>
                        <div class="size-8 flex flex-row  justify-center items-center"
                            on:click = move |_| {
                                is_menu_open.set(!is_menu_open.get());
                            }
                        >
                        {
                            move || {
                                if is_menu_open.get() {
                                    Either::Left(view! {
                                        <CloseIcon />
                                    })
                                } else {
                                    Either::Right(view! {
                                        <MenuFolderIcon />
                                    })
                                }
                            }
                        }

                        </div>
                    </div>

                </div>

                // 下拉菜单
                <div class=move || {
                    let base = "absolute left-0 top-12 w-screen bg-white opacity-90 transition-all duration-200 overflow-hidden";
                    if is_menu_open.get() {
                        format!("{base} h-screen z-50")
                    } else {
                        format!("{base} h-0 z-0")
                    }
                }>
                    <nav class="flex flex-col gap-5 pt-4">
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
                                                                    <div class="content-center text-center">
                                                                        <A href=href_link
                                                                            on:click= move |_| {
                                                                                leptos::logging::log!("on clicked.");
                                                                                is_menu_open.set(!is_menu_open.get());
                                                                            }
                                                                        >{menu.menu_name}</A>
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
            </header>

            // <div class=move || {
            //     let base = "w-screen bg-blue-300 opacity-70 transition-all duration-200";
            //     if is_menu_open.get() {
            //         format!("{base} h-screen z-50")
            //     } else {
            //         format!("{base} h-0 z-0")
            //     }

            // }>

            // </div>

            <Outlet/>
        </div>
    }
}
