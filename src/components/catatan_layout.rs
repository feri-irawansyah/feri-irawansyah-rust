use leptos::prelude::*;
use leptos_router::{components::Outlet, hooks::use_params_map, params::ParamsMap};

#[allow(non_snake_case)]
#[component]
pub fn CatatanLayout() -> impl IntoView {
    let params: Memo<ParamsMap> = use_params_map();

    // Memo yang akan update ketika route param berubah
    let category = Memo::new(move |_| {
        params.with(|p| p.get("category").unwrap_or("None".to_string()))
    });

    view! {
        <h1>"Category: " {move || category.get()}</h1>
        <Outlet/>
    }
}
