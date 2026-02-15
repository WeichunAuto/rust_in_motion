use leptos::{either::Either, logging::log, prelude::*};
use leptos_router::hooks::use_query_map;

use crate::{constant::QUEZ_ANSWER_DIR, server_fn::about_me::update_answer_by_quezid};

// 这个页面用于对 quez 答案的 更新；
#[component]
pub fn UpdateQuez() -> impl IntoView {
    let queries = use_query_map();

    // 使用响应式 Memo
    let quez_id = Memo::new(move |_| {
        queries
            .read()
            .get("id")
            .and_then(|id_str| id_str.parse::<i32>().ok())
            .unwrap_or(-1) // 待更新的 quez id, 不存在时默认-1
    });

    // 更新指定 quez 的答案；
    let update_answer = Action::new(move |id: &i32| {
        let id = *id;
        async move {
            update_answer_by_quezid(id).await

            // match rt {
            //     Ok(is_success) => {
            //         if is_success {
            //             leptos::logging::log!("update is successful. now update the view");
            //         } else {
            //             leptos::logging::log!("update not successful.");
            //         }
            //     }
            //     Err(_) => {
            //         leptos::logging::log!("error happened");
            //     }
            // }
        }
    });

    view! {
        {
            move || if quez_id.get() == -1 {
                Either::Left(view! {
                    <h2 class="w-screen text-center">"待更新的 quez 不存在！"</h2>
                })
            } else {
                Either::Right(view! {
                    <p>{format!("将读取 Markdown 文件位置：{}answer_for_quez_", QUEZ_ANSWER_DIR)}{quez_id.get()}".md"</p>
                    <button
                        class="px-4 py-2 rounded bg-black text-white"
                        on:click=move |_| {update_answer.dispatch(quez_id.get());}
                    >
                        "更新答案"
                    </button>

                    // pending 状态时
                    <Show when=move||update_answer.pending().get()>
                        <p>"更新中...."</p>
                    </Show>

                    // 成功 或 失败后的结果展示
                    <Show when=move || update_answer.value().get().is_some()>
                    {
                        move || {
                            let rt = update_answer.value().get();
                            match rt {
                                Some(Ok(true)) => view! { <p class="text-green-600">"更新成功！"</p> }.into_any(),
                                Some(Ok(false)) => view! { <p class="text-red-600">"更新失败，quez ID 不存在或发生错误！"</p> }.into_any(),
                                Some(Err(e)) => view! { <p class="text-red-600">{format!("❌ 错误: {}", e)}</p> }.into_any(),
                                None => view! { <p class="text-red-600">"未知错误咯。。。"</p> }.into_any(),
                            }
                        }
                    }
                    </Show>
                })
            }
        }

    }
}
