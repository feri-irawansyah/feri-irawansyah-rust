use leptos::web_sys;
use leptos::{prelude::*, task::spawn_local};
use leptos_router::hooks::use_navigate;
use leptos_sweetalert::{Swal, SwalIcon, SwalOptions};
use crate::contexts::index::error_message;
use crate::{app::BACKEND_URL, contexts::models::{LoginRequest, SuccessResponse, ErrorResponse}};

#[allow(non_snake_case)]
#[component]
pub fn Login() -> impl IntoView {
    // state untuk username & password
    let email = RwSignal::new("".to_string());
    let password = RwSignal::new("".to_string());
    let navigate = use_navigate();

    // handler tombol login
    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let email = email.get();
        let password = password.get();
        let navigate = navigate.clone();

        spawn_local(async move {
            let login_request = LoginRequest {
                email: email.clone(),
                password: password.clone(),
            };

            let url = format!("{}/auth/login", BACKEND_URL);
            let res = gloo_net::http::Request::post(&url)
                .header("Content-Type", "application/json")
                .credentials(web_sys::RequestCredentials::Include)
                .body(serde_json::to_string(&login_request).unwrap());

            match res {
                Ok(res) => {
                    let response = res.send().await.unwrap();
                    if response.status() == 200 {
                        let _success_response: SuccessResponse<String> = response.json().await.unwrap();
                        navigate("/admin", Default::default());
                        
                    } else {
                        let error: ErrorResponse = response.json::<ErrorResponse>().await.unwrap();
                        let error_msg: &'static str = Box::leak(error_message(&error.error).into_boxed_str());
                        Swal::fire(SwalOptions {
                            title: "Login failed",
                            text: error_msg,
                            icon: SwalIcon::ERROR,
                            confirm_button_text: "OK",
                            ..Default::default()
                        });
                    }
                }
                Err(_) => {
                    Swal::fire(SwalOptions {
                        title: "Network Error",
                        text: "Tidak bisa terhubung ke server",
                        icon: SwalIcon::ERROR,
                        confirm_button_text: "OK",
                        ..Default::default()
                    });
                }
            }
        });
    };


    view! {
        <div class="container login" data-aos="fade-left">
            <div class="card">
                <div class="card-header">
                    <i class="bi bi-person-check"></i>
                    <h5 class="card-title">"Login"</h5>
                    <p>Feri Irawansyah profile admin area</p>
                </div>

                <div class="card-body">
                    <form on:submit=on_submit>
                        <div class="mb-3">
                            <label class="form-label">"Email address"</label>
                            <input type="text" class="form-control" placeholder="Email" on:input=move |e| email.set(event_target_value(&e))/>
                        </div>
                        <div class="mb-3">
                            <label class="form-label">"Password"</label>
                            <input type="password" class="form-control" placeholder="Password" on:input=move |e| password.set(event_target_value(&e))/>
                        </div>
                        <button type="submit" class="btn btn-primary w-100">
                            "Login"
                        </button>
                        <a class="btn btn-link w-100" href="/"><i class="bi bi-house me-2"></i><span>Back to Home</span></a>
                    </form>
                </div>
            </div>
        </div>
    }
}