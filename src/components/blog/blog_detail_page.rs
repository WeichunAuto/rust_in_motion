use leptos::prelude::*;
use leptos_router::{hooks::use_params, params::Params};


use crate::{dto::blog_response_dto::BlogResponsetDto, server_fn::blog::load_blog_by_id};

// 博客详情页-博客ID参数
#[derive(Params, PartialEq)]
struct BlogIdParams {
    blog_id: Option<i32>,
}

#[component]
pub fn BlogDetailPage() -> impl IntoView {
    let params = use_params::<BlogIdParams>();
    let blog_id = params
            .read()
            .as_ref()
            .ok()
            .and_then(|p| p.blog_id.clone())
            .unwrap_or_default();

    let blog_resource = OnceResource::new(load_blog_by_id(blog_id));

    view! {
        {
            
            move || {
                if let Some(Ok(blog_dto)) = blog_resource.get() {
                    view! {
                        <Suspense fallback=move|| view! {<p>"blog detail page is loading..."</p>}>
                            <div>{blog_dto.get_blog_title()}</div>
                        </Suspense>
                    }.into_any()
                    
                } else {
                    view! {
                        <div>"no data."</div>
                    }.into_any()
                    
                }
            }
        }
        
        
    }
}