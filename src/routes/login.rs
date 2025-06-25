use leptos::prelude::*;
use leptos_sweetalert::{Swal, SwalIcon, SwalOptions};

#[allow(non_snake_case)]
#[component]
pub fn Login() -> impl IntoView {
    // state untuk username & password
    let email = RwSignal::new("".to_string());
    let password = RwSignal::new("".to_string());

    // handler tombol login
    let on_submit = Action::new(move |_: &()| {
        let email = email.get();
        let password = password.get();
        
        async move {

            if email == "admin" && password == "admin" {
                Swal::fire(SwalOptions {
                    title: "This is a title",
                    text: "This is some text",
                    icon: SwalIcon::SUCCESS,
                    confirm_button_text: "LETS GO",
                    show_cancel_button: true,
                    show_deny_button: true,
                    ..SwalOptions::default()
                });
            } else {
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
    });


    view! {
        <form on:submit=move |e| {
            e.prevent_default();
            on_submit.dispatch(());
        }>
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
            <button type="submit">
                "Login"
            </button>
        </form>
    }
}