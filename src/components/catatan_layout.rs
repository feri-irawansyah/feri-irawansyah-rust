use leptos::prelude::*;
use leptos_router::{components::Outlet, hooks::use_params_map};

use crate::contexts::models::AppState;

#[allow(non_snake_case)]
#[component]
pub fn CatatanLayout() -> impl IntoView {
    let params = use_params_map();
    let category = Memo::new(move |_| {
        params.with(|p| p.get("category"))
    });
    let slug = Memo::new(move |_| {
        params.with(|p| p.get("slug"))
    });

    let state = expect_context::<AppState>();

    Effect::new(move |_| {
        if slug.get().is_none() {
            state.title.set("".to_string());
        }
    });

    view! {
        <section id="catatan" class="catatan section" data-aos="fade-left">

            <div class="container">
                <Show
                    when=move || { category.get().is_some() }
                    fallback=|| view! { <span></span> }
                >
                    <Show
                        when=move || { !slug.get().is_some() }
                        fallback=move || {
                            let cat = category.get().clone().unwrap_or("".to_string());
                            view! { 
                                <a class="btn text-start back" href={format!("/catatan/{}", cat)}>
                                    <i class="bi bi-arrow-left-circle me-2"></i>Kembali
                                </a>
                            }
                        }
                    >
                        <a class="btn text-start back" href="/catatan">
                            <i class="bi bi-arrow-left-circle me-2"></i>Kembali
                        </a>
                    </Show>
                </Show>
            </div>
            
            <div class="container section-title" data-aos="fade-left" data-aos-delay="100">
                
                <h2>Catatan {move || state.name.get()}</h2>
                <Show
                    when=move || !state.title.get().is_empty()
                    fallback=|| view! {
                        <p>
                            "Catatan gue tutorial, wawasan teknologi, opini gajelas, kadang membingungkan, dan ide - ide tentang teknologi yang disusun untuk memicu ide dan terkadang memecahkan masalah kadang juga engga."
                        </p>
                    }
                >
                    <h5 class="text-uppercase fw-bold">{move || state.title.get()}</h5>
                    <hr/>
                </Show>
            </div>

            <div class="container" data-aos="fade-up" data-aos-delay="200">
    
                <Outlet/>

            </div>
        </section>
    }
}
