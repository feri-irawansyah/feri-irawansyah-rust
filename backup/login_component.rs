use leptos::{leptos_dom::logging::console_log, prelude::*};
use crate::{api::{handlers::auth_handler::login_handler, models::auth_model::LoginRequest}};
use leptos_sweetalert::{Swal, SwalIcon, SwalOptions};

#[allow(non_snake_case)]
#[component]
pub fn Login() -> impl IntoView {
    // state untuk username & password
    let email = RwSignal::new("".to_string());
    let password = RwSignal::new("".to_string());
    let message = RwSignal::new("".to_string());

    // handler tombol login
    let on_submit = Action::new(move |_: &()| {
        let email = email.get();
        let password = password.get();
        
        async move {
            
            match login_handler(LoginRequest { email, password }).await {

                Ok(res) => {
                    Swal::fire(SwalOptions {
                        title: "This is a title",
                        text: "This is some text",
                        icon: SwalIcon::SUCCESS,
                        confirm_button_text: "LETS GO",
                        show_cancel_button: true,
                        show_deny_button: true,
                        ..SwalOptions::default()
                    });
                    // show_alert("Login Berhasil", &res.message, &"success".to_string());
                    message.set(res.message);
                },
                Err(err) => {
                    // show_alert("Login Gagal", &err.to_string(), &"error".to_string());
                    // let err_parsed: serde_json::Value = serde_json::from_str(&err.to_string()).unwrap();
                    // console_log(&err_parsed["message"].to_string());
                    let err_str = format!("{:?}", err);

                    console_log(&err_str);

                    if let Some(json_str) = err_str.strip_prefix("ServerError(\"").and_then(|s| s.strip_suffix("\")")) {
                        console_log(json_str);
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(json_str) {
                            console_log(format!("JSON: {:?}", json["message"].as_str().unwrap_or("Unknown error")).as_str());
                            let message_err = json["message"].as_str().unwrap_or("Unknown error");
                            let error_detail = json["error"].as_str().unwrap_or("No details");

                            message.set(format!("Message: {message_err}, Error: {error_detail}"));
                        } else {
                            message.set("Failed to parse server error".to_string());
                        }
                    } else {
                        message.set("Invalid server error format".to_string());
                    }
                    // message.set(format!("Error: {:?}", err));
                    Swal::fire(SwalOptions {
                        title: "This is a title",
                        text: "This is some text",
                        icon: SwalIcon::ERROR,
                        confirm_button_text: "LETS GO",
                        show_cancel_button: true,
                        show_deny_button: true,
                        ..SwalOptions::default()
                    });
                }
            }
        }
    });


    view! {
        <div>
            <input
                type="text"
                placeholder="Username"
                on:input=move |e| email.set(event_target_value(&e))
            />
            <input
                type="password"
                placeholder="Password"
                on:input=move |e| password.set(event_target_value(&e))
            />
            <button on:click=move |_| { 
                console_log("tombol di click");
                on_submit.dispatch(());
             }>
                "Login"
            </button>
            <p>{move || message.get()}</p>
        </div>
    }
}