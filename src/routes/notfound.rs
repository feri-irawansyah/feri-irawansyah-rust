use leptos::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    let go_back_and_reload = move |_| {
        // Kembali ke halaman sebelumnya
        window().history().unwrap().back().ok();
    };

    view! {
        <section id="notfound" class="notfound section">
            <div class="container section-title" data-aos="zoom-in">
                <h2>404</h2>
                <p>Page Not Found</p>
                <button class="btn btn-primary mt-3" on:click=go_back_and_reload>
                    <i class="bi bi-arrow-left-circle me-2"></i>Kembali
                </button>
            </div>
            <div class="container" data-aos="zoom-in" data-aos-delay="100">
                <img class="img-fluid" src="https://cdn.dribbble.com/users/285475/screenshots/2083086/dribbble_1.gif" alt="404" />
            </div>
        </section>
    }
}