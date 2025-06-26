use leptos::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn Home() -> impl IntoView {

    view! {
        <section id="hero" class="hero section"  data-aos="zoom-out">             
            <img src="/assets/img/hero-bg.jpeg" alt="" />
            <div class="container-fluid">
                <div class="row justify-content-center">
                    <div class="col-lg-9">
                        <h2>Feri Irawansyah</h2>
                        <p>Programmer 
                        </p>
                        <div class="tech-marquee-wrapper">
                            <div class="tech-marquee">
                                // Ulang dua kali untuk efek infinite
                                <div class="tech-marquee-content">
                                    <img src="/assets/img/skills/rust.png" alt="Rust" />
                                    <img src="/assets/img/skills/javascript.png" alt="JavaScript" />
                                    <img src="/assets/img/skills/ts.png" alt="TypeScript" />
                                    <img src="/assets/img/skills/html.png" alt="HTML" />
                                    <img src="/assets/img/skills/css.png" alt="CSS" />
                                    <img src="/assets/img/skills/svelte.png" alt="Svelte" />
                                    <img src="/assets/img/skills/wasm.png" alt="WASM" />
                                </div>
                                <div class="tech-marquee-content">
                                    <img src="/assets/img/skills/rust.png" alt="Rust" />
                                    <img src="/assets/img/skills/javascript.png" alt="JavaScript" />
                                    <img src="/assets/img/skills/ts.png" alt="TypeScript" />
                                    <img src="/assets/img/skills/html.png" alt="HTML" />
                                    <img src="/assets/img/skills/css.png" alt="CSS" />
                                    <img src="/assets/img/skills/svelte.png" alt="Svelte" />
                                    <img src="/assets/img/skills/wasm.png" alt="WASM" />
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}