use leptos::prelude::*;
use crate::api::handlers::auth_handler::{login_handler, LoginRequest};

#[allow(non_snake_case)]
#[component]
pub fn Login() -> impl IntoView {
    // state untuk username & password
    let email = RwSignal::new("".to_string());
    let password = RwSignal::new("".to_string());
    let message = RwSignal::new("".to_string());

    println!("email: {}, password: {}", email.get(), password.get());

    // handler tombol login
    let on_submit = Action::new(move |_: &()| {
        let email = email.get();
        let password = password.get();
        
        async move {
            match login_handler(LoginRequest { email, password }).await {
                Ok(res) => {
                    println!("res: {:#?}", res);
                    message.set(res.message);
                },
                Err(err) => {
                    println!("err: {:#?}", err);
                    message.set(format!("Error: {:?}", err));
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
            <button on:click=move |_| { on_submit.dispatch(()); }>
                "Login"
            </button>
            <p>{message.get()}</p>
        </div>
    }
}