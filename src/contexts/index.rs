use leptos::{leptos_dom::logging::console_log, prelude::*};
use chrono::{DateTime, Datelike, FixedOffset, NaiveDate, Timelike, Utc};
use serde_json::Value;

use crate::leptos_ignore;

use crate::{contexts::models::{AppState, ModalState}, directives::table::Columns};

pub fn hitung_usia(tanggal_lahir: &str) -> Option<i32> {
    // Parse tanggal lahir, formatnya "YYYY-MM-DD"
    let birth_date = NaiveDate::parse_from_str(tanggal_lahir, "%Y-%m-%d").ok()?;
    let today = Utc::now().date_naive();

    let mut usia = today.year() - birth_date.year();

    // Jika ulang tahun belum lewat, kurangi 1
    if (today.month(), today.day()) < (birth_date.month(), birth_date.day()) {
        usia -= 1;
    }

    Some(usia)
}

pub fn format_wib_date(date_str: &str) -> String {
    // Parse input sebagai UTC
    let utc_datetime = DateTime::parse_from_rfc3339(date_str)
        .unwrap_or_else(|_| Utc::now().into());

    // Konversi ke WIB (UTC+7)
    let wib_offset = FixedOffset::east_opt(7 * 3600).unwrap();
    let wib_datetime = utc_datetime.with_timezone(&wib_offset);

    let weekday = wib_datetime.format("%a"); // Sen, Sel, Rab, dst
    let month = wib_datetime.format("%b");   // Jan, Feb, dst
    let day = wib_datetime.day();
    let year = wib_datetime.year();

    format!(
        "{} {} {}, {}",
        weekday,
        month,
        day,
        year
    )
}

pub fn format_wib_datetime(date_str: &str) -> String {
    // Parse input sebagai UTC
    let utc_datetime = DateTime::parse_from_rfc3339(date_str)
        .unwrap_or_else(|_| Utc::now().into());

    // Konversi ke WIB (UTC+7)
    let wib_offset = FixedOffset::east_opt(7 * 3600).unwrap();
    let wib_datetime = utc_datetime.with_timezone(&wib_offset);

    // Format 12 jam
    let hour = wib_datetime.hour();
    let hour12 = if hour % 12 == 0 { 12 } else { hour % 12 };
    let minutes = wib_datetime.minute();
    let is_pm = hour >= 12;

    let weekday = wib_datetime.format("%a"); // Sen, Sel, Rab, dst
    let month = wib_datetime.format("%b");   // Jan, Feb, dst
    let day = wib_datetime.day();
    let year = wib_datetime.year();

    format!(
        "{:02}:{:02} {} WIB, {} {} {}, {}",
        hour12,
        minutes,
        if is_pm { "PM" } else { "AM" },
        weekday,
        month,
        day,
        year
    )
}

pub fn error_message(err: &Option<Value>) -> String {
    match err {
        Some(Value::String(s)) => s.clone(),

        Some(Value::Object(map)) => {
            if let Some(Value::String(msg)) = map.get("message") {
                msg.clone()
            } else {
                serde_json::to_string(map).unwrap_or("Unknown object error".to_string())
            }
        }

        Some(Value::Array(arr)) => {
            arr.iter()
                .filter_map(|v| v.as_str())
                .collect::<Vec<&str>>()
                .join(", ")
        }

        _ => "Unknown error".to_string(),
    }
}

leptos_ignore! {
    
    pub fn format_cell_value(item: &serde_json::Value, col: &Columns,
    // ) -> Option<leptos::prelude::View<leptos::html::HtmlElement<Td, Vec<AnyAttribute>, (leptos::html::HtmlElement<leptos::html::Span, Vec<AnyAttribute>, (String,)>,)>>> {
    ) -> Option<View<impl IntoView>> {
    
        let state: ModalState = expect_context::<ModalState>();
    
        let on_click = move |_| {
            // kosong, atau bisa nanti diisi
        };
    
        match col.field.as_str() {
            "notes_id" | "tsv" => {
                let string = "hidden";
    
                Some(view! {
                    <td class="d-none".to_string()>
                        <a href="#".to_string()  on:click=on_click class="invisible".to_string() data-bs-toggle="".to_string() data-bs-target="".to_string()>{string.to_string()}</a>
                    </td>
                })
            }
    
            "content" => {
                let value = item.get(&col.field)?.as_str().unwrap_or("").to_string();
                let title = item.get("title")?.as_str().unwrap_or("").to_string();
    
                
                let get_url = move |value: &str| {
                    state.note_url.set(Some(value.to_string()));
                    state.title.set(title.to_string());
                    console_log(&value);
                };
                
                Some(view! {
                    <td class="".to_string()>
                        <a href="#".to_string() on:click=move |_| { get_url(&value); } class="".to_string() data-bs-toggle="modal".to_string() data-bs-target="#note-content".to_string()>{value.clone()}</a>
                    </td>
                })
            }
    
            "last_update" => {
                let value = item.get(&col.field)?.as_str().unwrap_or("");
                let formatted = format!("🕒 {}", format_wib_datetime(value));
                Some(view! {
                    <td class="".to_string()>
                        <a href="#".to_string() on:click=on_click class="text-muted".to_string() data-bs-toggle="".to_string() data-bs-target="".to_string()>{formatted}</a>
                    </td>
                })
            }
    
            "category" => {
                let status = item.get(&col.field)?.as_str().unwrap_or("-");
                let class = format!("badge {}", match status {
                    "fullstack" => "bg-success text-dark",
                    "backend" => "bg-warning text-dark",
                    "frontend" => "bg-info text-dark",
                    "series" => "bg-danger text-dark",
                    _ => "bg-secondary text-dark",
                });
                Some(
                    view! {
                        <td class="".to_string()>
                            <a href="#".to_string() on:click=on_click class=class.to_string() data-bs-toggle="".to_string() data-bs-target="".to_string()>{status.to_string()}</a>
                        </td>
                    }
                )
            }
    
            _ => {
                let value = item.get(&col.field)?.as_str().unwrap_or("-");
                Some(
                    view! {
                        <td class="".to_string()>
                            <a href="#".to_string() on:click=on_click class="text-muted".to_string() data-bs-toggle="".to_string() data-bs-target="".to_string()>{value.to_string()}</a>
                        </td>
                    }
                )
            }
        }
    }
}    
