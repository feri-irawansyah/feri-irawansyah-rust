use leptos::prelude::*;
use leptos_sweetalert::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes}, StaticSegment, WildcardSegment
};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    components:: {
        catatan_layout::CatatanLayout,
        menu_list::MenuList
    }, contexts::models::AppState, routes::{
        about::About, contact::Contact, home::Home, login::Login, notes::{
            category::Category, list_catatan::ListCatatan, slug::Slug
        }, notfound::NotFound, portfolio::Portfolio, services::Services
    }
};

#[wasm_bindgen(inline_js = "
    export function initAOS() {
        AOS.init({
            disable: false,
            startEvent: 'DOMContentLoaded', 
            initClassName: 'aos-init',
            animatedClassName: 'aos-animate',
            useClassNames: false,
            disableMutationObserver: false, 
            debounceDelay: 50,
            throttleDelay: 99, 
            
            offset: -9999, 
            delay: 0, 
            duration: 400, 
            easing: 'ease',
            once: false, 
            mirror: false, 
            anchorPlacement: 'top-center',

        });
    }
")]
extern "C" {
    fn initAOS();
    pub fn refreshAOS();
}

#[allow(non_snake_case)]
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    Swal::init_key_handlers();
    let global_state = AppState {
        count: RwSignal::new(0),
        name: RwSignal::new("Feri Irawansyah".to_string()),
        title: RwSignal::new("".to_string()),
    };

    // Register biar bisa dipakai semua komponen
    provide_context(global_state);

    Effect::new(move |_| {
        initAOS(); // ini panggil JS function
    });

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/feri-irawansyah.css"/>
        <Stylesheet id="aos" href="/css/aos.min.css"/>
        <Stylesheet id="icons" href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.13.1/font/bootstrap-icons.min.css"/>
        <Stylesheet id="icons" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/styles/atom-one-dark.css"/>

        // sets the document title
        <Title text="Feri Irawansyah"/>

        // content for this welcome page
        <Router>
            <main data-bs-theme="dark">
                <div class="container-fluid">
                    <div class="row">
                        <div class="col-lg-2 p-0">
                            <MenuList/>
                        </div>
                        <div class="col-lg-10 p-0">
                            <Routes fallback=move || "Not found.">
                                <Route path=StaticSegment("") view=Home/>
                                <Route path=StaticSegment("about") view=About/>
                                <Route path=StaticSegment("services") view=Services/>
                                <Route path=StaticSegment("portfolio") view=Portfolio/>
                                <ParentRoute path=leptos_router::path!("/catatan") view=CatatanLayout>
                                    <Route path=leptos_router::path!("") view=ListCatatan />
                                    <Route path=leptos_router::path!(":category") view=Category />
                                    <Route path=leptos_router::path!(":category/:slug") view=Slug />
                                </ParentRoute>
                                <Route path=StaticSegment("contact") view=Contact/>
                                <Route path=StaticSegment("login") view=Login/>
                                <Route path=WildcardSegment("any") view=NotFound/>
                            </Routes>
                        </div>
                    </div>
                </div>
            </main>
        </Router>
    }
}
