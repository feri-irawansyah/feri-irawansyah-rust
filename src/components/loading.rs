use leptos::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn LoadingScreen(visible: RwSignal<bool>) -> impl IntoView {
    view! {
        <Show when=move || visible.get()>
            <div class="loading-screen fade-in">
                <div class="spinner"></div>
                <span>Please wait...</span>
            </div>
        </Show>
    }
}