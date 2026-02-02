use leptos::{either::Either, prelude::*};
use leptos_router::components::{Outlet, A};

use crate::server_fn::menu::get_menus;

/**
 * 系统头部公共导航部分
 */
#[component]
pub fn HomePage() -> impl IntoView {
    let menu_once = OnceResource::new(get_menus());

    view! {
        <div class="w-full h-40 bg-amber-300 flex gap-4">
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
                                            <div>
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

            <Outlet/>
        </div>
    }
}
