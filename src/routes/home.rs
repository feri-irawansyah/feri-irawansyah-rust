use gloo_net::http::Request;
use leptos::{prelude::*, task::spawn_local};
use wasm_bindgen::JsCast;
use leptos::web_sys::HtmlImageElement;
use crate::{app::BACKEND_URL, components::card_loading::CardLoading, contexts::{index::format_wib_date, models::{AppState, Notes, NotesData}}};

#[allow(non_snake_case)]
#[component]
pub fn Home() -> impl IntoView {

    let state: AppState = expect_context::<AppState>();
    let notes: RwSignal<Vec<Notes>> = RwSignal::new(vec![]);
    let (loading, set_loading) = signal(false);
    let current_page: RwSignal<i32> = RwSignal::new(1);
    let limit = 3;

    let fetch_notes = move |page: i32| {
        let offset = (page - 1) * limit;
        let url = format!(
            "{}/data/table?tablename=notes&offset={}&limit={}&nidkey=notes_id",
            BACKEND_URL,
            offset,
            limit
        );

        spawn_local(async move {
            set_loading(true);
            if let Ok(response) = Request::get(&url).send().await {
                if let Ok(data) = response.json::<NotesData>().await {
                    notes.set(data.rows);
                }
            }
            set_loading(false);
        });
    };

    Effect::new(move |_| {
        fetch_notes(current_page.get());
    });

    view! {
        <section id="hero" class="hero section"  data-aos="zoom-in">             
            <img src="/assets/img/hero-bg.jpeg" alt="" />
            <div class="container" data-aos="fade-up" data-aos-delay="100">
                <div class="row justify-content-center">
                    <div class="col-lg-12 mb-3" data-aos="slide-right" data-aos-delay="200">
                        <h2>Hi, "I'm" <span class="ms-3 text-primary">Feri</span></h2>
                        <p>Programmer 
                        </p>
                    </div>
                    <div class="col-lg-11">
                        <div class="d-flex flex-row justify-content-between" data-aos="slide-right" data-aos-delay="300">
                            <h4 class="fw-bold">Latest <span class="text-primary">Notes</span></h4>
                            <a class="btn see-all" href="/catatan">See All <i class="bi bi-arrow-right"></i></a>
                        </div>
                        <div class="row mb-3" data-aos="slide-right" data-aos-delay="300">
                            <Show when=move || !loading.get() fallback=|| view! { <CardLoading delay={Some(300)} count={Some(3)}/> }>
                                {move || {
                                    let notes_clone = notes.get().clone();
                                    {notes_clone.iter().enumerate().map(|(i, note)| view! {
                                        <div class="col-lg-4 col-md-6 d-flex align-items-stretch" data-aos="fade-up" data-aos-delay={format!("{}", i * 200)} data-aos-duration="1000">
                                            <a class="card text-center" href=format!("/catatan/{}/{}", note.category.clone(), note.slug.clone())>
                                                <img src=format!("/assets/img/notes/{}.png", note.slug.clone())
                                                    alt={note.title.clone()}
                                                    on:error=move |e: leptos::ev::ErrorEvent| {
                                                        if let Some(target) = e.target() {
                                                            if let Ok(img) = target.dyn_into::<HtmlImageElement>() {
                                                                img.set_src("/assets/img/notes/default.jpg");
                                                            }
                                                        }
                                                    }
                                                class="card-img rounded py-1"/>
                                                <div class="card-img-overlay">
                                                    <div class="hashtag">
                                                    {
                                                        let list_hashtag = note.hashtag.clone().unwrap_or(vec!["".to_string()]);
                                                        list_hashtag.iter().map(|hashtag| view! {
                                                            <span>#{hashtag.clone()}</span>
                                                        }).collect_view()
                                                    }
                                                    </div>
                                                    <h5 class="card-title text-start text-uppercase">{note.title.clone()}</h5>
                                                    <p class="card-text text-start">{note.description.clone()}</p>
                                                </div>
                                                <div class="card-footer text-body-secondary">
                                                    <div class="d-flex justify-content-between">
                                                        <div class="d-flex gap-1 author">
                                                            <img class="rounded-circle" src="/assets/img/logo-ss.png" style="width: 1.5rem; height: 1.5rem;"/>
                                                            <span>{move || state.name.get()}</span>
                                                        </div>
                                                        <small class="text-white">{format_wib_date(&note.last_update)}</small>
                                                    </div>
                                                </div>
                                            </a>
                                        </div>
                                    }).collect_view()}
                                }}
                            </Show>
                        </div>
                        // <div class="row" data-aos="slide-right" data-aos-delay="300">
                        //     <h4 class="fw-bold">My <span class="text-primary">Tech Stack</span></h4>
                        //     <div class="card">
                        //         <SkillMarquee/>
                        //     </div>
                        // </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct Skill {
    image_src: String,
}

#[derive(Debug, Deserialize)]
struct SkillResponse {
    rows: Vec<Skill>,
}

#[allow(non_snake_case)]
#[component]
pub fn SkillMarquee() -> impl IntoView {

    let skills = RwSignal::new(vec![]);

    Effect::new(move |_| {
        spawn_local(async move {
            if let Ok(response) = Request::get("https://snakesystem-api.shuttle.app/api/v1/data/table?tablename=skills&offset=0&limit=50&nidkey=skill_id").send().await {
                if let Ok(data) = response.json::<SkillResponse>().await {
                    skills.set(data.rows);
                }
            }
        });
    });

    view! {
        <div class="marquee-custom">
            {move || {
                let skills_clone = skills.get().clone();
                {skills_clone.iter().map(|skill| view! {
                    <div class="marquee-text">
                        <img src=format!("/assets/{}", skill.image_src.clone())/>
                    </div>
                }).collect_view()}
            }}
        </div>

    }
}
