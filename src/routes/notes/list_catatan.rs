use leptos::prelude::*;

use crate::contexts::models::Category;

#[allow(non_snake_case)]
#[component]
pub fn ListCatatan() -> impl IntoView {

    let categories = vec![
        Category {
            name: "backend",
            image: "/assets/img/backend.png",
            title: "Backend",
            desc: "Intip catatan tentang aplikasi backend dengan performa tinggi dan modern sekalian nyobain.",
            list_tech: &[
                "rest", "graphql", "rust", "actix-web", "bun", "hono", "nodejs", "nestjs", "laravel-api", ".net",
            ],
        },
        Category {
            name: "frontend",
            image: "/assets/img/frontend.png",
            title: "Frontend",
            desc: "Lu mungkin bakal suka liat catatan tentang tampilan aplikasi yang oke dan interaktif.",
            list_tech: &[
                "svelte/sveltekit", "react/nextjs", "sass", "tailwind", "bootstrap", "tauri", "handlebars", "flutter",
            ],
        },
        Category {
            name: "fullstack",
            image: "/assets/img/fullstack.png",
            title: "Fullstack",
            desc: "Kombinasi backend dan frontend dalam satu catatan mungkin bisa jadi inspirasi baru bang.",
            list_tech: &[
                "graphql", "prisma", "session", "handlebars", "jwt", "architecture", "blade", "laravel", "web-socket",
            ],
        },
        Category {
            name: "random",
            image: "/assets/img/random.png",
            title: "Random",
            desc: "Jangan ngoding mulu bang ntar pingsan kita, kita juga manusia bukan robot. Robot aja bisa error.",
            list_tech: &[
                "internet-information-service", "nginx", "laragon", "gajelas", "tapi", "bisa", "error", "olahraga", "makan",
            ],
        },
        Category {
            name: "series",
            image: "/assets/img/series.png",
            title: "Series",
            desc: "Kita bacotin teknologi yang ada dimuka bumi dalam konteks pebelajaran.",
            list_tech: &[
                "internet-information-service", "nginx", "laragon", "gajelas", "tapi", "bisa", "error", "olahraga", "makan",
            ],
        },
    ];
     
    view! {
        <div class="row" data-aos="slide-left">
            {categories.into_iter().enumerate().map(|(i, cat)| {
                view! {
                    <div class="col-md-4 my-3" data-aos="fade-up" data-aos-delay={format!("{}", i * 100)}>
                        <a class="card p-3 text-decoration-none h-100" href=format!("/catatan/{}", cat.name)>
                            <img src={cat.image} class="card-img-top" alt={cat.title}/>
                            <div class="card-body">
                                <h4 class="card-title">{cat.title}</h4>
                                <p class="card-text">{cat.desc}</p>
                            </div>
                            <div class="card-footer">
                                <div class="d-flex flex-row gap-2 flex-wrap">
                                    {cat.list_tech.iter().map(|tech| {
                                        view! {
                                            <span class="badge bg-primary">#{*tech}</span>
                                        }
                                    }).collect_view()}
                                </div>
                            </div>
                        </a>
                    </div>
                }
            }).collect_view()}
        </div>
    }
}