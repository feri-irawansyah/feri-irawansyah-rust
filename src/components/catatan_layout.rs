use leptos::prelude::*;
use leptos_router::{components::Outlet, hooks::use_params_map};

#[allow(non_snake_case)]
#[component]
pub fn CatatanLayout() -> impl IntoView {
    let params = use_params_map();
    let category = params.with(|p| p.get("category"));

    view! {
        <h1>"Category: " {category.unwrap_or("None".to_string())}</h1>
        // Ini kayak <Outlet/> di React Router
        <Outlet/>
    }
}
