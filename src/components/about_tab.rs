use leptos::{prelude::*};

#[allow(non_snake_case)]
#[component]
pub fn AboutTab(menu_item: RwSignal<&'static str>) -> impl IntoView {

    view! {
        <ul class="col-12 nav nav-tabs mb-3 mx-3">
            {["Intro", "Experience", "Skills", "Certifications"].iter().map(|item| view! {
                <li class="nav-item">
                    <a class=move || if menu_item.get() == *item { "nav-link active" } else { "nav-link" } on:click=move |_| menu_item.set(*item)>{*item}</a>
                </li>
            }).collect::<Vec<_>>()}
        </ul>
    }
}
