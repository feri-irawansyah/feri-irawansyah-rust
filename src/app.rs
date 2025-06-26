use leptos::prelude::*;
use leptos_sweetalert::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes}, StaticSegment, WildcardSegment
};

use crate::{
    components:: {
        catatan_layout::CatatanLayout,
        list_catatan::ListCatatan
    }, 
    routes::{
        about::About, contact::Contact, home::Home, login::Login, notes::{
            category::Category,
            slug::Slug,
        }, notfound::NotFound, portfolio::Portfolio, services::Services
    }
};

#[allow(non_snake_case)]
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    Swal::init_key_handlers();
    // provide_alert_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/feri-irawansyah.css"/>
        <Stylesheet id="leptos" href="/css/aos.min.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
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

                <p><a href="/">"Home"</a></p>
                <p><a href="/about">"About"</a></p>
                <p><a href="/services">"Services"</a></p>
                <p><a href="/portfolio">"Portfolio"</a></p>
                <p><a href="/catatan">"Blog"</a></p>
                <p><a href="/contact">"Contact"</a></p>

            </main>
            // <SweetAlert/>
        </Router>
    }
}
