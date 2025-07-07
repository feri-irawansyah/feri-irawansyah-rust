use gloo_net::http::Request;
use leptos::{prelude::*, task::spawn_local};

use crate::{app::BACKEND_URL, directives::table::Table};

#[allow(non_snake_case)]
#[component]
pub fn NotesManagement() -> impl IntoView {

    let notes: RwSignal<Vec<serde_json::Value>> = RwSignal::new(vec![serde_json::Value::Array(vec![])]);
    let notes_page: RwSignal<i32> = RwSignal::new(1);
    let notes_total: RwSignal<usize> = RwSignal::new(0);
    let notes_loading: RwSignal<bool> = RwSignal::new(false);
    let notes_limit = 4;

    let fetch_notes = move |page: i32| {
        let offset = (page - 1) * notes_limit;
        let url = format!(
            "{}/data/table?tablename=notes&offset={}&limit={}&nidkey=notes_id",
            BACKEND_URL,
            offset,
            notes_limit
        );

        spawn_local(async move {
            notes_loading.set(true);
            if let Ok(response) = Request::get(&url).send().await {
                if let Ok(data) = response.json::<serde_json::Value>().await {
                    notes.set(data["rows"].as_array().unwrap_or(&vec![]).to_vec());
                    notes_total.set(data["total"].as_u64().unwrap_or(10) as usize);
                }
            }
            notes_loading.set(false);
        });
    };

    Effect::new(move |_| {
        fetch_notes(notes_page.get());
    });

    view! {
        <div class="notes-management">
            <div class="row">
                <div class="col-12">
                    <div class="card">
                        <div class="card-header">
                            Notes Management
                        </div>
                        <div class="card-body">
                            <Table
                                table="notes".to_string() 
                                data=notes.clone() 
                                page=notes_page
                                total=notes_total
                                limit=notes_limit
                                refresh=fetch_notes
                                loading_data=notes_loading/>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}