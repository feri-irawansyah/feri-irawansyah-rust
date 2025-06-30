use leptos::prelude::*;
use leptos_router::{hooks::use_location, location::Location};

#[allow(non_snake_case)]
#[component]
pub fn MenuList() -> impl IntoView {
    
    let location: Location = use_location();

    view! {
        <div class="menu-list">
            <img src="/assets/img/feri.jpg" alt="feri" class="rounded-circle img-fluid about-img mb-1" />
            <h5 class="fw-bold mb-0">Feri Irawansyah</h5>
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
                @ <strong><a href="https://github.com/feri-irawansyah">Feri Irawansyah</a></strong>.
                <p>All Rights Reserved</p>
                <div class="credits">
                    Powered by <a href="https://leptos.dev">Rust Leptos</a> <i>"❤️"</i>
                </div>
            </div>
        </div>
    }
}