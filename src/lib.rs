pub mod app;
pub mod contexts {
    pub mod index;
    pub mod models;
}
pub mod routes {
    pub mod home;
    pub mod notfound;
    pub mod about;
    pub mod services;
    pub mod portfolio;
    pub mod contact;
    pub mod login;
    pub mod notes {
        pub mod category;
        pub mod slug;
        pub mod list_catatan;
    }
}

pub mod components {
    pub mod catatan_layout;
    pub mod sweet_alert;
    pub mod menu_list;
    pub mod badge;
    pub mod list_skill;
    pub mod markdown;
}

pub mod directives {
    pub mod alert_context;
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
