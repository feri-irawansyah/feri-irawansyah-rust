use gloo_net::http::Request;
use leptos::{prelude::*, task::spawn_local};
use leptos_router::hooks::use_params_map;
use crate::{app::BACKEND_URL, components::card_loading::CardLoading, contexts::{index::format_wib_date, models::{AppState, Notes, NotesData}}};
use wasm_bindgen::JsCast;
use leptos::web_sys::HtmlImageElement;

#[allow(non_snake_case)]
#[component]
pub fn Category() -> impl IntoView {
    let params = use_params_map();
    let category = Memo::new(move |_| {
        params.with(|p| p.get("category"))
    });
    let notes: RwSignal<Vec<Notes>> = RwSignal::new(vec![]);
    let (total, set_total) = signal(0);
    let (current_page, set_current_page) = signal(1);
    let (loading, set_loading) = signal(false);
    let state = expect_context::<AppState>();

    let limit = 4;

    let filter = serde_json::json!({
        // "category": category.get().unwrap_or_else(|| "".to_string())
    });

    let fetch_notes = move |page: i32| {
        let offset = (page - 1) * limit;
        let url = format!(
            "{}/data/table?tablename=notes&offset={}&limit={}&nidkey=notes_id&filter={}",
            BACKEND_URL,
            offset,
            limit,
            urlencoding::encode(&filter.to_string())
        );

        spawn_local(async move {
            set_loading(true);
            if let Ok(response) = Request::get(&url).send().await {
                if let Ok(data) = response.json::<NotesData>().await {
                    notes.set(data.rows);
                    set_total(data.total);
                }
            }
            set_loading(false);
        });
    };

    Effect::new(move |_| {
        fetch_notes(current_page.get());
    });

    view! { 
        <div class="row category" data-aos="slide-left">
            <Show
                when=move || { loading.get() == false }
                fallback=|| view! { <CardLoading delay={Some(0)} count={Some(3)} /> }
            >
                <Show
                    when=move || { !notes.get().is_empty() }
                    fallback=|| view! { <h1>No notes available</h1> }
                >
                    <div class="row list-notes">
                        {move || {
                            let notes_clone = notes.get().clone();
                            {notes_clone.iter().enumerate().map(|(i, note)| view! {
                                <div class="col-lg-4 col-md-6 d-flex align-items-stretch"  data-aos="fade-up" data-aos-delay={format!("{}", i * 200)} data-aos-duration="1000">
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
                                                <div class="d-flex gap-2">
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
                    </div>

                    // Pagination
                    <nav class=move || if total.get() as i32 <= limit { "d-none" } else { "pagination-container" }>
                        <ul class="pagination justify-content-end">
                            <li class=format!("page-item {}", if current_page.get() == 1 { "disabled" } else { "" })>
                                <button class="page-link" on:click=move |_| set_current_page(current_page.get() - 1)>
                                    <i class="bi bi-caret-left-fill"></i>
                                </button>
                            </li>
                            {
                                let total_pages = (total.get() as f64 / limit as f64).ceil() as i32;
                                (1..=total_pages).map(|i| {
                                    view! {
                                        <li class=format!("page-item {}", if current_page.get() == i { "active" } else { "" })>
                                            <button class="page-link" on:click=move |_| set_current_page(i)>{i}</button>
                                        </li>
                                    }
                                }).collect_view()
                            }
                            <li class=format!("page-item {}", if current_page.get() * limit >= total.get().try_into().unwrap() { "disabled" } else { "" })>
                                <button class="page-link" on:click=move |_| set_current_page(current_page.get() + 1)>
                                    <i class="bi bi-caret-right-fill"></i>
                                </button>
                            </li>
                        </ul>
                    </nav>
                </Show>
            </Show>
        </div>
     }
}