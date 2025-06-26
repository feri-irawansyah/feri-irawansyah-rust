use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[allow(non_snake_case)]
#[component]
pub fn Slug() -> impl IntoView {
    let params = use_params_map();
    let category = params.with(|p| p.get("category"));
    let slug = params.with(|p| p.get("slug"));

    view! {
        <p>"Kategori: " {category.unwrap_or("None".to_string())}</p>
        <p>"Slug: " {slug.unwrap_or("None".to_string())}</p>
    }
}
