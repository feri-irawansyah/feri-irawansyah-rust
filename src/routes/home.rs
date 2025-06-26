use leptos::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn Home() -> impl IntoView {

    view! {
        <section id="hero" class="hero section"  data-aos="zoom-out">             
            <img src="/assets/img/hero-bg.jpeg" alt="" />
            <div class="container">
                <div class="row justify-content-center">
                    <div class="col-lg-12">
                        <h2>Feri Irawansyah</h2>
                        <p>Programmer 
                        </p>
                    </div>
                    <div class="col-lg-12">
                        
                    </div>
                </div>
            </div>
        </section>
    }
}