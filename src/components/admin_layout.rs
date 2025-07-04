use leptos::{leptos_dom::logging::console_log, prelude::*, task::spawn_local};
use leptos_router::{components::Outlet, hooks::use_navigate};

use crate::{contexts::models::AppState, middleware::session::{check_session, SessionData, SessionResponse}};

#[allow(non_snake_case)]
#[component]
pub fn AdminLayout() -> impl IntoView {
    let state = expect_context::<AppState>();
    let session = RwSignal::new(SessionResponse { data: SessionData::new() });
    Effect::new(move |_| {
        let navigate = use_navigate();
        spawn_local(async move { 
            state.loading.set(true);
            let response = check_session().await;
            match response {
                Ok(session) => {
                    console_log(format!("Session: {:#?}", session).as_str());
                }
                Err(error) => {
                    console_log(format!("Error: {:#?}", error).as_str());
                    navigate("/login", Default::default());
                }
            }
            state.loading.set(false);
        });
    });
    view! {
        <Show when=move || session.get().data.user_id != 0 fallback=|| view! { <span></span>} >
            <div class="container-fluid">
                <div class="row">
                    Menu
                </div>
                <div class="row">
                    <Outlet />
                </div>
            </div>
        </Show>
    }
}