use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::{contexts::models::AppState};

#[allow(non_snake_case)]
#[component]
pub fn AdminLayout() -> impl IntoView {
    let state = expect_context::<AppState>();
    let is_open = RwSignal::new(true);

    view! {
        <Show when=move || state.session.get().usernid != 0 fallback=|| view! { <span></span>} >
            <div class="container-fluid admin-layout" data-aos="fade-left">
                <div class="d-flex">
                    <div class=move || {
                            if is_open.get() {
                                "sidebar"
                            } else {
                                "sidebar collapsed"
                            }
                        }>
                        <ul>
                            <li class="logo"><a href="/">
                                <img src="/assets/img/logo-ss.png" alt="feri" class="rounded-circle img-fluid about-img mb-1" /> 
                                <h5>Feri Irawansyah <img class="real-image" src="/assets/img/real.png" alt="feri" /></h5>
                                <p>Software Engineer</p>
                            </a></li>
                            <li><a href="/admin"><i class="bi bi-grid"></i> <span>Dashboard</span></a></li>
                            <li><a href="/admin/user"><i class="bi bi-person"></i> <span>User Management</span></a></li>
                            <li><a href="/admin/notes-management"><i class="bi bi-journal-code"></i><span>Notes Management</span></a></li>
                            <li><a href="/admin/settings"><i class="bi bi-gear"></i> <span>Settings</span></a></li>
                        </ul>
                    </div>
                    <div class=move || {
                            if is_open.get() {
                                "main-area"
                            } else {
                                "main-area expanded"
                            }
                        }>
                        <nav class="navbar">
                            <div class="container-fluid">
                                <div class="navbar-brand">
                                    <button class="menu-toggle" on:click=move |_| is_open.set(!is_open.get())><i class="bi bi-list"></i></button>
                                    <h5 class="fw-bold mb-0">Snakesystem Admin Area</h5>
                                </div>
                                <div class="navbar-nav">
                                    <a class="nav-link"><i class="bi bi-bell"></i></a>
                                    <a class="nav-link"><i class="bi bi-person"></i></a>
                                    <a class="nav-link"><i class="bi bi-box-arrow-right"></i></a>
                                </div>
                            </div>
                        </nav>
                        <div class="content">
                            <Outlet />
                        </div>
                    </div>
                </div>
            </div>
        </Show>
    }
}