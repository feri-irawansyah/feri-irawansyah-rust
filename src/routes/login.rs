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
        <div class="container" data-aos="fade-left">
            <div class="row justify-content-center">
                <div class="col-md-8">
                    <div class="card">
                        <div class="card-header">Login</div>

                        <div class="card-body">
                            <form on:submit=move |e| {
                                e.prevent_default();
                                on_submit.dispatch(());
                            }>
                                <div class="mb-3">
                                    <label class="form-label">"Email address"</label>
                                    <input type="text" class="form-control" placeholder="Username" on:input=move |e| email.set(event_target_value(&e))/>
                                </div>
                                <div class="mb-3">
                                    <label class="form-label">"Password"</label>
                                    <input type="password" class="form-control" placeholder="Password" on:input=move |e| password.set(event_target_value(&e))/>
                                </div>
                                <button type="submit" class="btn btn-primary">
                                    "Login"
                                </button>
                            </form>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}