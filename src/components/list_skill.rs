use leptos::prelude::*;
use crate::contexts::models::Skill;

#[allow(non_snake_case)]
#[component]
pub fn ListSkill(data: Vec<Skill>, selected_skill: RwSignal<Option<Skill>>) -> impl IntoView {

    view! {
        <div class="list-skill list-group">
           {data.iter().map(|skill| {
                let skill_clone = skill.clone();
                view! {
                    <a href="#"
                        class="list-group-item list-group-item-action"
                        on:click=move |_| {
                            if selected_skill.get() == Some(skill_clone.clone()) {
                                selected_skill.set(None);
                            } else {
                                selected_skill.set(Some(skill_clone.clone()));
                            }
                        }
                    >
                        <div class="d-flex justify-content-between">
                            <div class="col-lg-8">
                                <h5>{skill.name}</h5>
                                <p>{skill.description}</p>
                                <small>{skill.experience}</small>
                            </div>
                            <div class="col-lg-4 p-3">
                                <img src=format!("/assets/img/skills/{}.png", skill.name.to_lowercase()) alt=skill.name />
                            </div>
                        </div>
                    </a>
                }
            }).collect::<Vec<_>>()}
        </div>

        // disini
    }
}