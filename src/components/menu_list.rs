use leptos::{leptos_dom::logging::console_log, prelude::*, task::spawn_local};
use leptos_router::{components::Outlet, hooks::use_location, location::Location};

use crate::{contexts::models::AppState, middleware::session::check_session};

#[allow(non_snake_case)]
#[component]
pub fn MenuList() -> impl IntoView {
    
    let location: Location = use_location();
    let state = expect_context::<AppState>();

    Effect::new(move |_| {
        spawn_local(async move { 
            state.loading.set(true);
            let response = check_session().await;
            match response {
                Ok(session) => {
                    state.session.set(session);
                }
                Err(error) => {
                    console_log(format!("Error: {:#?}", error).as_str());
                }
            }
            state.loading.set(false);
        });
    });

    view! {
        <div class="menu-list col-2 p-0 scroll-custom">
            <img src="/assets/img/feri.jpg" alt="feri" class="rounded-circle img-fluid about-img mb-1" />
            <h5 class="fw-bold mb-0">{move || state.name.get()}<img class="real-image" src="/assets/img/real.png" alt="feri" /></h5>
            <p class="mt-0">Software Engineer From Indonesia</p>
            <ul class="list-unstyled">
                <li class:active=move || (location.pathname)() == "/"><a href="/"><i class="bi bi-house"></i> <span>Home</span></a></li>
                <li class:active=move || (location.pathname)() == "/about"><a href="/about"><i class="bi bi-person"></i> <span>About</span></a></li>
                <li class:active=move || (location.pathname)() == "/portfolio"><a href="/portfolio"><i class="bi bi-journal-code"></i> <span>Portfolio</span></a></li>
                <li class:active=move || (location.pathname)().contains("catatan")><a href="/catatan"><i class="bi bi-journal-text"></i> <span>Catatan</span></a></li>
                <li class:active=move || (location.pathname)() == "/services"><a href="/services"><i class="bi bi-briefcase"></i> <span>Services</span></a></li>
                <li class:active=move || (location.pathname)() == "/contact"><a href="/contact"><i class="bi bi-envelope"></i> <span>Contact</span></a></li>
            </ul>
            <div class="copyright">
                @ <strong><a href="https://github.com/feri-irawansyah">{move || state.name.get()}</a></strong>.
                <p>All Rights Reserved</p>
                <div class="credits">
                    Powered by <a target="_blank" href="https://leptos.dev">Rust Leptos</a> <i>"❤️"</i>
                </div>
            </div>
        </div>

        <div class="col-10 p-0 content scroll-custom">
            <Show when=move || state.session.get().usernid != 0 fallback=|| view! { <span></span>} >
                <a href="/admin" class="btn btn-outline-primary btn-sm to-dashboard">
                    <i class="bi bi-house-door-fill me-2"></i><span>Dashboard</span>
                </a>
            </Show>
            <Outlet />
        </div>
    }
}