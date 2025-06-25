use leptos::prelude::*;

#[derive(Clone)]
pub struct AlertState {
    pub show: RwSignal<bool>,
    pub title: RwSignal<String>,
    pub message: RwSignal<String>,
    pub icon: RwSignal<String>,
}

pub fn provide_alert_context() {
    let alert = AlertState {
        show: RwSignal::new(false),
        title: RwSignal::new("".into()),
        message: RwSignal::new("".into()),
        icon: RwSignal::new("info".into()),
    };
    provide_context(alert);
}

// Function yang bisa dipanggil dari mana saja
pub fn show_alert(title: &str, message: &str, icon: &str) {
    let ctx: AlertState = use_context::<AlertState>().expect("Alert context not found");
    ctx.title.set(title.to_string());
    ctx.message.set(message.to_string());
    ctx.icon.set(icon.to_string());
    ctx.show.set(true);
}
