use gloo_net::http::Request;
use leptos::{leptos_dom::logging::console_log, prelude::*, task::spawn_local};
use leptos_router::hooks::use_params_map;
use wasm_bindgen::JsCast;
use leptos::web_sys::HtmlImageElement;
use crate::{app::BACKEND_URL, components::card_loading::CardLoading, contexts::{index::format_wib_date, models::{AppState, Note, NoteData}}, directives::markdown::MarkdownFromUrl};

#[allow(non_snake_case)]
#[component]
pub fn Slug() -> impl IntoView {
    let params = use_params_map();
    let slug = params.with(|p| p.get("slug"));
    let content: RwSignal<Option<String>> = RwSignal::new(None);
    let note: RwSignal<Note> = RwSignal::new(Note::new());
    let state = expect_context::<AppState>();
    let (loading, set_loading) = signal(false);

    let slug_name = slug.clone().unwrap_or("".to_string());

    Effect::new(move |_| {

        let url = format!(
            "{}/library/get-library/{}",
            BACKEND_URL,
            slug_name
        );

        spawn_local(async move {
            set_loading(true);
            if let Ok(response) = Request::get(&url).send().await {
                if let Ok(data) = response.json::<NoteData>().await {
                    note.set(data.data);
                    state.title.set(note.get().title.clone());
                    content.set(Some(note.get().content.clone()));
                } else {
                    console_log(format!("Error parsing JSON: {:?}", response.status()).as_str());
                }
            }
            set_loading(false);
        });
    });

    view! {
        <Show
            when=move || { !loading.get() }
            fallback=|| view! { <CardLoading count=Some(1) delay=Some(0) /> }
        >
            <Show
                when=move || { note.get().content.is_empty() }
                fallback=move || {
                    view! { 
                        <div class="author d-flex flex-row align-items-start justify-content-start w-100">
                            <img src="/assets/img/logo-ss.png" class="mb-3 rounded-circle" width="50px" alt=""/>
                            <div class="flex-column">
                                <a class="text-decoration-none text-muted" href="https://github.com/feri-irawansyah" target="_blank">
                                {move || state.name.get().to_string()} <img src="/assets/img/real.png" width="20px" alt=""/></a>
                                <p class="text-muted">{format_wib_date(&note.get().last_update)}</p>
                            </div>
                        </div>
                        <div class="w-100"  data-aos="fade-up" data-aos-duration="1000">
                            <div class="image-content d-flex justify-content-center" >
                                <img class="img-fluid rounded" src={format!("/assets/img/notes/{}.png", note.get().slug)} alt={note.get().title} 
                                    on:error=move |e: leptos::ev::ErrorEvent| {
                                        if let Some(target) = e.target() {
                                            if let Ok(img) = target.dyn_into::<HtmlImageElement>() {
                                                img.set_src("/assets/img/notes/default.jpg");
                                            }
                                        }
                                    }/>
                            </div>
                            <div class="markdown-body">
                                <MarkdownFromUrl url={content}/>
                            </div>
                        </div>
                    }
                }
            >
                <h1>"Slug not found"</h1>
            </Show>
        </Show>
        
    }
}
