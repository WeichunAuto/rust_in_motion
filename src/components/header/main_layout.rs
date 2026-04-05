use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::components::header::headers::Headers;

#[component]
pub fn MainLayout() -> impl IntoView {
    view! {
        <div class="relative min-h-screen">
            // Header 单独存在（scroll 只影响这里）
            <Headers/>

            // Outlet 独立（不会被 scroll 影响）
            <Outlet/>
        </div>
    }
}