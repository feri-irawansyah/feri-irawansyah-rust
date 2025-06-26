use leptos::prelude::*;
use alert_rs::{leptos::Alert, IconType, Position};
use crate::directives::alert_context::{AlertState};

#[allow(non_snake_case)]
#[component]
pub fn SweetAlert() -> impl IntoView {
    let ctx = use_context::<AlertState>().expect("Alert context not found");

    let (icons, set_icons) = signal(IconType::Info);

    match ctx.icon.get().as_str() {
        "question" => set_icons(IconType::Question),
        "success" => set_icons(IconType::Success),
        "error" => set_icons(IconType::Error),
        "warning" => set_icons(IconType::Warning),
        "info" => set_icons(IconType::Info),
        _ => set_icons(IconType::Info),
    }

    view! {
        <Alert
            title={"Alert Title"}
            body={"This is an alert message"}
            show_alert={(ctx.show.read_only(), ctx.show.write_only())}
            icon_class={"flex justify-center"}
            timeout={10000}
            confirm_button_text={"Okay"}
            cancel_button_text={"Cancel"}
            confirm_button_class={"bg-green-500 text-white rounded"}
            cancel_button_class={"bg-red-500 text-white rounded"}
            show_confirm_button={true}
            show_cancel_button={true}
            show_close_button={true}
            on_confirm={Callback::from(move || {})}
            on_cancel={Callback::from(move || {})}
            position={Position::Center}
            icon_type={icons.get()}
            alert_class={"flex items-center text-center justify-center bg-gray-800 text-white border border-gray-600"}
            title_class={"dark:text-white"}
            body_class={"dark:text-gray-300"}
            icon_color={""}
            icon_width={"50"}
        />
    }
}
