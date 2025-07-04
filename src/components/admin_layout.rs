use leptos::prelude::*;
use leptos_router::components::Outlet;

#[allow(non_snake_case)]
#[component]
pub fn AdminLayout() -> impl IntoView {
    view! {
        <div class="flex flex-row">
            <div class="w-1/6">
                Menu
            </div>
            <div class="w-5/6">
                <Outlet />
            </div>
        </div>
    }
}