use leptos::prelude::*;

use crate::dto::menu_dto::MenuDto;

#[component]
pub fn MobileHeader(
    menu_once: OnceResource<Result<Vec<MenuDto>, ServerFnError>>,
    scrolled: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <div>"这是移动屏幕的 header."</div>
    }
}
