use leptos_router::{components::A, hooks::use_params, params::Params};

use leptos::prelude::*;
// use leptos_shadcn_ui::{Button, Card, Input};

use crate::components::{about_me::about_me::AboutMeSector, common::{ElseIf, Fallback, If, Then}};
// use leptos_shadcn_button::Button;
// use leptos_shadcn_input::Input;

// 不同栏目切换的URL路径参数
#[derive(Params, PartialEq)]
struct SectorParams {
    sector: Option<String>,
}

#[component]
pub fn Sectors() -> impl IntoView {
    let params = use_params::<SectorParams>();
    let sector = RwSignal::new(move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|p| p.sector.clone())
            .unwrap_or_default()
    });

    let is_project = Signal::derive(move || sector.get()() == "Projects");
    let is_about_me = Signal::derive(move || sector.get()() == "About_Me");
    let is_robotics = Signal::derive(move || sector.get()() == "Robotics");
    let is_web = Signal::derive(move || sector.get()() == "Web");
    let is_tool = Signal::derive(move || sector.get()() == "Tools");
    view! {
        <div>
            <If condition=is_project>
                // The `If` component always expects a `Then` child for `then_slot`
                <Then slot:then><ProjectsSector /></Then>
                <ElseIf slot condition=is_about_me><AboutMeSector /></ElseIf>
                <ElseIf slot condition=is_robotics><p>"这是 robotics 栏目"</p></ElseIf>
                <ElseIf slot condition=is_web><p>"这是 Web 栏目"</p></ElseIf>
                <ElseIf slot condition=is_tool><p>"这是 Tools 栏目"</p></ElseIf>
                <Fallback slot><p>"进入了fallback分支！"</p></Fallback>
            </If>
        </div>
    }
}

#[component]
fn ProjectsSector() -> impl IntoView {
    view! {
        <div class="space-y-4 h-[1200px]">
            "this is a project content."
        </div>
    }
}
