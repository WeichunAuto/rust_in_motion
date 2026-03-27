// 解决 action workflow 中编译阶段的递归深度限制的问题
#![recursion_limit = "512"]

pub mod app;

#[cfg(feature = "ssr")] // 整个 backend 模块只在server使用
pub mod entity;

#[cfg(feature = "ssr")] // 整个 backend 模块只在server使用
pub mod config;

#[cfg(feature = "ssr")] // 整个 backend 模块只在server使用
pub mod state;

pub mod components;
pub mod constant;
pub mod dto;
pub mod server_fn;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
