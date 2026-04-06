use leptos::ev::scroll;
use leptos::prelude::*;

use crate::components::common::{Fallback, If, Then};

use crate::components::header::mobile_header::MobileHeader;
use crate::components::header::pc_header::PcHeader;
use crate::server_fn::menu::get_menus;
use wasm_bindgen::prelude::*;
use web_sys::wasm_bindgen;
use web_sys::window;
/**
 * 系统头部公共导航部分
 */
#[component]
pub fn Headers() -> impl IntoView {
    // 一次性加载菜单资源
    let menu_once = OnceResource::new(get_menus());

    // 是否是移动屏幕
    let is_mobile =
        use_context::<ReadSignal<Option<bool>>>().expect("provide context 中没有监听到屏幕尺寸！");

    // 监听滚动
    let scrolled = RwSignal::new(false);

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
        {
            move || {
                let is_mobile_ready = is_mobile.get();

                view! {
                    <Show
                        when= move || is_mobile_ready.is_some()
                        fallback=move || view! { 
                            <div class="flex items-center justify-center h-[60vh] w-full text-gray-500">
                                "loading data...."
                            </div>
                        }
                    >
                        {
                            move || {
                                match is_mobile_ready {
                                    Some(true) => view! {
                                        <MobileHeader menu_once=menu_once scrolled=scrolled/>
                                    }.into_any(),

                                    Some(false) => view! {
                                        <PcHeader menu_once=menu_once scrolled=scrolled />
                                    }.into_any(),

                                    None => view! {}.into_any()
                                }
                            }
                        }
                    </Show>
                }
            }
        }
    }
}
