use leptos::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn ListCatatan() -> impl IntoView {
    
    view! {
        <div>
            <h2>"List Semua Kategori"</h2>
            <ul>
                <li><a href="/catatan/backend">Backend</a></li>
                <li><a href="/catatan/frontend">Frontend</a></li>
                <li><a href="/catatan/fullstack">Fullstack</a></li>
            </ul>
        </div>
    }
}