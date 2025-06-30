use leptos::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn Badge(experience: i32) -> impl IntoView {
    view! {
        <div class="badge-experience">
            <span class="label">"Experience"</span>
            <span class="value">{format!("{}+ Years", experience)}</span>
        </div>
    }
}
