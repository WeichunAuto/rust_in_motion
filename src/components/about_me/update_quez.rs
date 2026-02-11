use leptos::{either::Either, logging::log, prelude::*};
use leptos_router::hooks::use_query_map;

use crate::server_fn::about_me::update_answer_by_quezid;

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

    // Effect::watch(
    //     move || quez_id.get(),
    //     move |id, _, _| {
            
    //     },
    //     true,
    // );

    // 更新指定 quez 的答案；
    let update_answer = Action::new(move|id: &i32| {
        let id = *id;
        async move {
            let rt = update_answer_by_quezid(id).await;
            match rt {
                Ok(is_success) => {
                    if is_success {
                        leptos::logging::log!("update is successful. now update the view");
                        
                    } else {
                        leptos::logging::log!("update not successful.");
                    }
                }
                Err(_) => {
                    leptos::logging::log!("error happened");
                }
            }
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
                    {
                        update_answer.dispatch(quez_id.get());
                    }
                    <h2>"这里更新quez."</h2>
                })
            }
        }

    }
}
