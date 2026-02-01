use leptos::prelude::*;

/**
 * 定义 IF Then ElseIf 组件
 */
#[component]
pub fn If(
    condition: Signal<bool>,
    // Component slot, should be passed through the <Then slot> syntax
    then: Then,
    #[prop(default=vec![])] else_if: Vec<ElseIf>,
    #[prop(optional)] fallback: Option<Fallback>,
) -> impl IntoView {
    move || {
        if condition.get() {
            (then.children)().into_any()
        } else if let Some(else_if) = else_if.iter().find(|i| i.condition.get()) {
            (else_if.children)().into_any()
        } else if let Some(fallback) = &fallback {
            (fallback.children)().into_any()
        } else {
            ().into_any()
        }
    }
}

#[slot]
pub struct Then {
    children: ChildrenFn,
}

#[slot]
pub struct ElseIf {
    condition: Signal<bool>,
    children: ChildrenFn,
}

#[slot]
pub struct Fallback {
    children: ChildrenFn,
}
