use leptos::prelude::*;
use chrono::Local;
use gloo_timers::callback::Interval;

#[allow(non_snake_case)]
#[component]
pub fn Clock() -> impl IntoView {
    // Signal waktu sekarang
    let (time, set_time) = signal(format_now());

    // Update setiap detik
    Effect::new(move || {
        Interval::new(1000, move || {
            set_time.set(format_now());
        }).forget(); // keep interval alive
    });

    // View-nya
    view! {
        <div class="clock">
            <strong>{move || time.get()}</strong>
        </div>
    }
}

fn format_now() -> String {
    let now = Local::now();
    now.format("%B %d, %Y at %I:%M:%S %p").to_string()
}