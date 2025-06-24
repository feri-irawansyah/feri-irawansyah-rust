pub mod app;
pub mod api {
    pub mod handlers {
        pub mod auth_handler;
    }

    pub mod services {
        pub mod auth_service;
    }

    pub mod middleware {
        pub mod connection;
        pub mod crypto;
        pub mod session;
    }
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
    }
}

pub mod components {
    pub mod catatan_layout;
    pub mod list_catatan;
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
