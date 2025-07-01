use leptos::{leptos_dom::logging::console_log, prelude::*};

use crate::{components::{about_tab::AboutTab, list_skill::ListSkill}, contexts::{index::hitung_usia, models::Skill}};

#[allow(non_snake_case)]
#[component]
pub fn About() -> impl IntoView {

    let menu_item = RwSignal::new("Intro");
    
    view! {
        <section id="about" class="about section" data-aos="fade-right">

            <div class="container section-title" data-aos="slide-right" data-aos-delay="100">
                <h2>About Me</h2>
            </div>
            <div class="container" data-aos="slide-right" data-aos-delay="200">

                <div class="row justify-content-start">
                    <AboutTab menu_item={menu_item} />
                    <div class="col-12 content">
                        {move || {
                            match menu_item.get() {
                                "Intro" => view! { <Intro/> }.into_any(),
                                "Experience" => view! { <Experience/> }.into_any(),
                                "Skills" => view! { <Skills /> }.into_any(),
                                "Certifications" => view! { <h2>Certifications</h2> }.into_any(),
                                _ => view! { <h2>Intro</h2> }.into_any(),
                            }
                        }}
                    </div>
                </div>
            </div>
        </section>

    }
}

#[allow(non_snake_case)]
#[component]
pub fn Intro() -> impl IntoView {

    let usia = hitung_usia("2000-06-09").unwrap_or(0);

    view! {
        <div data-aos="slide-right">
            <h2>Software Engineer & Developer.</h2>
            <p>"Hi, I'm Feri, commonly known as snakesystem —a Rust & C# Programmer, AI Engineer and Software Engineer. Based in West Java, Indonesia. I create Web-based applications, RESTfull API, Desktop using Rust and C# focusing on performance, clean code and scalable."</p>
            <p>"Personally, I am a child of odd-job laborers. I have worked since I was in elementary school to pay for school, helped by my parents' endless prayers. This has shaped me to have determination, focus, and discipline, the same energy that I pour into every line of code."</p>
            <p>"What's next? I am fully involved in the world of technology to overcome challenges and needs in the world using technology by developing Tech Snake System as an application development organization."</p>
            <p>"Have a crazy idea or just curious about technology? Contact me, let's build something great together!"</p>
            <hr/>
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
            </div>
        </div>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn Experience() -> impl IntoView {

    let experience = hitung_usia("2021-02-04").unwrap_or(0);

    view! {
        <div data-aos="slide-right" class="experience">
            <div class="row">
                <div class="col-12">
                    <div class="card">
                        <div class="d-flex justify-content-between">
                            <div class="flex-column experience-left">
                                <div class="d-flex align-items-center">
                                    <i class="bi bi-briefcase"></i>
                                    <div class="flex-column">
                                        <h5>Curriculum Vitae</h5>
                                        <p>"Access my updated curriculum vitae"</p>
                                    </div>
                                </div>
                                <div class="alert alert-success d-flex align-items-center" role="alert">
                                    <i class="bi bi-bookmark-check"></i>
                                    <div>
                                        {format!("{}+ Years", experience)} Experience
                                    </div>
                                </div>
                            </div>
                            <div class="flex-column experience-right">
                                <button class="btn" type="button"><i class="bi bi-eye"></i> Preview</button>
                                <button class="btn" type="button"><i class="bi bi-download"></i> Download</button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn Skills() -> impl IntoView {
    
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
        <div data-aos="slide-right">
            <div class="row">
                <div class="col-12">
                    <div class="card">
                        <div class="d-flex justify-content-between">
                            <div class="flex-column">
                                <i class="bi bi-briefcase"></i>
                            </div>
                            <div class="flex-column">
                                <button class="btn btn-primary" type="button" data-bs-toggle="collapse" data-bs-target="#collapseExperience" aria-expanded="false" aria-controls="collapseExperience"></button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}