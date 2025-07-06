use gloo_net::http::Request;
use leptos::{leptos_dom::logging::console_log, prelude::*, task::spawn_local};
use serde::Deserialize;

use crate::app::BACKEND_URL;

#[derive(Clone, Debug, Deserialize)]
pub struct ColumnData {
    pub data: Vec<Columns>,
}


#[derive(Clone, Debug, Deserialize)]
pub struct Columns {
    pub field: String,
    pub formatter: Option<String>,
    pub title: String,
    pub sortable: bool,
    #[serde(rename = "filterControl")]
    pub filter_control: String,
}

#[allow(non_snake_case)]
#[component]
pub fn Table(table: String, data: RwSignal<Vec<serde_json::Value>>, loading_data: RwSignal<bool>, page: RwSignal<i32>, limit: i32, total: RwSignal<i32>, refresh: impl Fn(i32) + 'static) -> impl IntoView {
    let columns = RwSignal::new(vec![]);
    let (loading, set_loading) = signal(false);

    let get_table_object = move |tablename: String| {
        let url = format!("{}/data/header?tablename={}", BACKEND_URL, tablename);

        spawn_local(async move {
            set_loading(true);
            if let Ok(response) = Request::get(&url).send().await {
                if let Ok(header) = response.json::<ColumnData>().await {
                    columns.set(header.data);
                }
            }
            set_loading(false);
        });
    };

    Effect::new(move |_| {
        get_table_object(table.clone());
    });

    view! {
        <table class="table table-striped">
            <thead>
                <tr>
                    {move || columns.get().iter().map(|column| view! {
                        <th>{column.title.clone()}</th>
                    }).collect_view()}
                </tr>
            </thead>
            <tbody>
                <Show when=move || !loading_data.get() fallback=|| view! { <span>loading...</span> }>
                    {move || {
                        data.get().iter().map(|item| {
                            view! {
                                <tr>
                                    {
                                        columns.get().iter().map(|col| {
                                            let value = item.get(&col.field).unwrap_or(&serde_json::Value::Null);
                                            view! {
                                                <td>{value.to_string()}</td>
                                            }
                                        }).collect_view()
                                    }
                                </tr>
                            }
                        }).collect_view()
                    }}
                </Show>
            </tbody>
        </table>
        <nav class=move || if total.get() == limit { "" } else { "d-none" }>
            <ul class="pagination justify-content-end pagination-sm">
                <li class=format!("page-item {}", if page.get() == 1 { "disabled" } else { "" })>
                    <button class="page-link" on:click=move |_| page.set(page.get() - 1)>
                        <i class="bi bi-caret-left-fill"></i>
                    </button>
                </li>
                {
                    let total_pages = (total.get() as f64 / limit as f64).ceil() as i32;
                    (1..=total_pages).map(|i| {
                        view! {
                            <li class=format!("page-item {}", if page.get() == i { "active" } else { "" })>
                                <button class="page-link" on:click=move |_| page.set(i)>{i}</button>
                            </li>
                        }
                    }).collect_view()
                }
                <li class=format!("page-item {}", if page.get() * limit >= total.get().try_into().unwrap() { "disabled" } else { "" })>
                    <button class="page-link" on:click=move |_| page.set(page.get() + 1)>
                        <i class="bi bi-caret-right-fill"></i>
                    </button>
                </li>
            </ul>
        </nav>
    }
}

