use leptos::{either::Either, prelude::*};
use leptos_router::components::{Outlet, A};

use crate::components::common::{Fallback, If, Then};
use crate::components::mobile_header::MobileHeader;
use crate::components::pc_header::PcHeader;
use crate::{components::icons::icons::SearchIcon, server_fn::menu::get_menus};
use wasm_bindgen::prelude::*;
use web_sys::wasm_bindgen;
use web_sys::window;
/**
 * 系统头部公共导航部分
 */
#[component]
pub fn HomePage() -> impl IntoView {
    // 一次性加载菜单资源
    let menu_once = OnceResource::new(get_menus());

    // 是否是移动屏幕
    let is_mobile =
        use_context::<ReadSignal<bool>>().expect("provide context 中没有监听到屏幕尺寸！");

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
        <If condition=is_mobile.into()>
                // The `If` component always expects a `Then` child for `then_slot`
                <Then slot:then><MobileHeader menu_once=menu_once /></Then>

                <Fallback slot>
                    <PcHeader menu_once=menu_once scrolled=scrolled />
                </Fallback>
            </If>

    }
}
