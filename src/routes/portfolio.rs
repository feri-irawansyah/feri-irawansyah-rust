use leptos::prelude::*;

use crate::contexts::models::{Portfolio, Tech};

#[allow(non_snake_case)]
#[component]
pub fn Portfolio() -> impl IntoView {

    let navigate = leptos_router::hooks::use_navigate();
    let portfolio = vec![

        Portfolio {
            title: "Snakesystem Onboarding Client",
            category: "frontend",
            description: "Snakesystem Onboarding Client adalah aplikasi web clone dari aplikasi Custoommer Onboarding Client yang saya buat menggunakan React di tempat saya bekerja. Aplikasi ini adalah sebuah form untuk calon nasabah yang ingin mendaftarkan diri untuk melakukan trading di suatu sekuritas",
            image: "/assets/img/portfolio/snakesystem-onboarding-client.jpeg",
            link: "https://sveltekit-onboarding-client.vercel.app",
            techs: vec![
                Tech {
                    name: "Javascript",
                    image: "/assets/img/skills/js.png",
                    link: "https://developer.mozilla.org/en-US/docs/Web/JavaScript"
                },
                Tech {
                    name: "Vite",
                    image: "/assets/img/skills/vite.png",
                    link: "https://vite.dev"
                },
                Tech {
                    name: "Svelte",
                    image: "/assets/img/skills/svelte.png",
                    link: "https://svelte.dev"
                },
            ],
        },
    ];
    
    view! {
        <section id="portfolio" class="portfolio section" data-aos="fade-right">     
            <div class="container section-title" data-aos="slide-right" data-aos-delay="100">
                <h2>Porfolio</h2>
                <p>I am a Software Engineer dedicated to building efficient, scalable, and user-friendly digital solutions. With a strong background in web development, I am used to working with various modern technologies such as JavaScript/TypeScript, Svelte, Rust, and various other frameworks.</p>
            </div>   
             <div class="container" data-aos="slide-right" data-aos-delay="200">
                <div class="row justify-content-start">
                    {portfolio.iter().map(|p| view! {
                        <div class="col-lg-6">
                            <a class="card text-bg-dark" href={p.link} target="_blank">
                                <img src={p.image} class="card-img" alt={p.title}/>
                                <div class="card-img-overlay">
                                    <h5 class="card-title">{p.title}</h5>
                                    <p class="card-text">{p.description}</p>
                                    {p.techs.iter().map(|t| view! {
                                        <div class="card-tech" on:click={
                                            let navigate = navigate.clone();
                                            let link = t.link;
                                            move |_| navigate(&link, Default::default())
                                        }>
                                            <span class="tech">#<small>{t.name}</small><img src={t.image} alt={t.name} /></span>
                                        </div>
                                    }).collect::<Vec<_>>()}
                                </div>
                                <div class="overlay-card"></div>
                            </a>
                        </div>
                    }).collect::<Vec<_>>()}
                </div>
             </div>     
        </section>
    }
}