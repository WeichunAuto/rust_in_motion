use leptos::prelude::*;

use crate::{constant::SUMMARY_DIR, server_fn::about_me::update_summary};

// 用于对 about_me 的 Summary 更新
#[component]
pub fn UpdateSummary() -> impl IntoView {
    // 无参数 Action
    let update_summary_action = Action::new(move |&()| async move { update_summary().await });
    view! {
        <p>{format!("将读取 Markdown 文件位置：{}", {SUMMARY_DIR})}</p>
        <button
            class="px-4 py-2 rounded bg-black text-white"
            on:click=move |_| {update_summary_action.dispatch(());}
        >
            "更新简介"
        </button>

        // pending 状态时
        <Show when=move||update_summary_action.pending().get()>
            <p>"更新中...."</p>
        </Show>

        // 成功 或 失败后的结果展示
        <Show when=move || update_summary_action.value().get().is_some()>
        {
            move || {
                let rt = update_summary_action.value().get();
                match rt {
                    Some(Ok(true)) => view! { <p class="text-green-600">"更新成功！"</p> }.into_any(),
                    Some(Ok(false)) => view! { <p class="text-red-600">"更新失败，发生错误！"</p> }.into_any(),
                    Some(Err(e)) => view! { <p class="text-red-600">{format!("❌ 错误: {}", e)}</p> }.into_any(),
                    None => view! { <p class="text-red-600">"未知错误咯。。。"</p> }.into_any(),
                }
            }
        }
        </Show>
    }
}
