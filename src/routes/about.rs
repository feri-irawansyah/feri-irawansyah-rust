use leptos::prelude::*;

use crate::{components::{badge::Badge, list_skill::ListSkill}, contexts::{index::hitung_usia, models::Skill}};

#[allow(non_snake_case)]
#[component]
pub fn About() -> impl IntoView {

    let usia = hitung_usia("2000-06-09").unwrap_or(0);
    let experience = hitung_usia("2021-02-04").unwrap_or(0);
    
    let skills = vec![
        Skill {
            name: "Rust",
            description: "Bahasa pemrograman sistem aman dan cepat.",
            experience: "3 Tahun",
            level: "Advanced",
        },
        Skill {
            name: "Actix",
            description: "Framework web performa tinggi untuk Rust.",
            experience: "2 Tahun",
            level: "Intermediate",
        },
    ];

    let selected_skill = RwSignal::new(Some(skills[0].clone()));

    
    view! {
        <section id="about" class="about section" data-aos="fade-right">

            <div class="container section-title" data-aos="slide-right" data-aos-delay="100">
                <h2>About Me</h2>
                <p>I am a Software Engineer dedicated to building efficient, scalable, and user-friendly digital solutions. With a strong background in web development, I am used to working with various modern technologies such as JavaScript/TypeScript, Svelte, Rust, and various other frameworks.</p>
            </div>
            <div class="container" data-aos="slide-right" data-aos-delay="200">

                <div class="row justify-content-start">
                    <div class="col-lg-8 content" data-aos="slide-right" data-aos-delay="300">
                        <h2>Software Engineer & Developer.</h2>
                        <hr />
                        <Badge experience={experience} />
                        <div class="row mt-3">
                            <div class="col-lg-6">
                            <ul>
                                <li><i class="bi bi-chevron-right"></i> <strong>Birth Date:</strong> <span>09 Juni 2000</span></li>
                                <li><i class="bi bi-chevron-right"></i> <strong>Website:</strong> <span>www.feri-irawansyah.github.io</span></li>
                                <li><i class="bi bi-chevron-right"></i> <strong>Phone:</strong> <span>+62 82323443535</span></li>
                                <li><i class="bi bi-chevron-right"></i> <strong>City:</strong> <span>DKI Jakarta, Indonesia</span></li>
                            </ul>
                            </div>
                            <div class="col-lg-6">
                            <ul>
                                <li><i class="bi bi-chevron-right"></i> <strong>Age:</strong> <span>{usia}</span></li>
                                <li><i class="bi bi-chevron-right"></i> <strong>Degree:</strong> <span>Sarjana Teknik Informatika</span></li>
                                <li><i class="bi bi-chevron-right"></i> <strong>Email:</strong> <span>feryirawansyah09@gmail.com</span></li>
                                <li><i class="bi bi-chevron-right"></i> <strong>Freelance:</strong> <span>Available</span></li>
                            </ul>
                        </div>
                        <div class="col-lg-12">
                             <Show when=move || selected_skill.get().is_some()>
                                {move || {
                                    selected_skill.get().map(|skill| {
                                        view! {
                                            <div class="card mt-3">
                                                <div class="card-body">
                                                    <h5 class="card-title">{skill.name}</h5>
                                                    <p>{skill.description}</p>
                                                    <ul>
                                                        <li>{format!("Experience: {}", skill.experience)}</li>
                                                        <li>{format!("Level: {}", skill.level)}</li>
                                                    </ul>
                                                </div>
                                            </div>
                                        }
                                    })
                                }}
                            </Show>
                        </div>
                    </div>
                </div>
                <div class="col-lg-4 content" data-aos="slide-right" data-aos-delay="100">
                    <h2>My Skills.</h2>
                    <hr />
                    <ListSkill data={skills} selected_skill={selected_skill} />
                </div>
            </div>
        </div>
     </section>

    }
}