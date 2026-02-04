use leptos::prelude::*;

#[component]
pub fn SearchIcon(class: &'static str) -> impl IntoView {
    view! {
        <svg class=format!("{} border border-red-500 box-border", class) fill="#D6D5D5" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" preserveAspectRatio="xMinYMin"><path d="M8 14A6 6 0 1 0 8 2a6 6 0 0 0 0 12zm6.32-1.094l3.58 3.58a1 1 0 1 1-1.415 1.413l-3.58-3.58a8 8 0 1 1 1.414-1.414z"/>
        </svg>
    }
}
