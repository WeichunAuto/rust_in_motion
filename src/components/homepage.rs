use leptos::{either::Either, prelude::*};
use leptos_router::components::Outlet;

use crate::server_fn::menu::get_menus;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // let menu_resource = Resource::new(|| (), |_| get_menus());
    let menu_once = OnceResource::new(get_menus());
    // let (menus, set_menus) = signal(vec![]);

    // // 加载菜单
    // Effect::watch(
    //     move || menu_resource.get(),
    //     move |new_val, _, _| {
    //         if let Some(Ok(menus_data)) = new_val {
    //             if !menus_data.is_empty() {
    //                 set_menus.set(menus_data.to_owned());
    //                 leptos::logging::log!("menus: {:?}", menus_data);
    //             }
    //         }
    //     },
    //     true,
    // );

    view! {
        <div>
            // <Suspense
            //     fallback = move || {view! {
            //         <div>
            //             "加载中..."
            //         </div>
            //     }}
            // >
            //     <ForEnumerate
            //         each = move || menus.get()
            //         key = |menu| menu.id
            //         children = move |_, menu| {
            //             view! {
            //                 <div>{menu.menu_name}</div>
            //             }
            //         }
            //     />
            // </Suspense>
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
                                        view! {
                                            <div>{menu.menu_name}</div>
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
