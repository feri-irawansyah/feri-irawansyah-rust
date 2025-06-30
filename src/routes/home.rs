use leptos::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn Home() -> impl IntoView {

    view! {
        <section id="hero" class="hero section"  data-aos="zoom-in">             
            <img src="/assets/img/hero-bg.jpeg" alt="" />
            <div class="container" data-aos="zoom-in" data-aos-delay="100">
                <div class="row justify-content-center">
                    <div class="col-lg-12">
                        <h2>Feri Irawansyah</h2>
                        <p>Programmer 
                        </p>
                    </div>
                    <div class="col-lg-8">
                        <h5>Latest Project</h5>
                        <div class="row">
                            <div class="card">
                                <a class="card-body">
                                    <h5 class="card-title"><i class="bi bi-journal-code"></i></h5>
                                    <h6 class="card-subtitle mb-2 text-body-secondary">Card subtitle</h6>
                                    <p class="card-text">Some quick example text to build on the card title and make up the bulk of the cards content.</p>
                                </a>
                            </div>
                            <div class="card">
                                <div class="card-body">
                                    <h5 class="card-title"><i class="bi bi-journal-code"></i></h5>
                                    <h6 class="card-subtitle mb-2 text-body-secondary">Card subtitle</h6>
                                    <p class="card-text">Some quick example text to build on the card title and make up the bulk of the cards content.</p>
                                </div>
                            </div>
                            <div class="card">
                                <div class="card-body">
                                    <h5 class="card-title"><i class="bi bi-journal-code"></i></h5>
                                    <h6 class="card-subtitle mb-2 text-body-secondary">Card subtitle</h6>
                                    <p class="card-text">Some quick example text to build on the card title and make up the bulk of the cards content.</p>
                                </div>
                            </div>
                        </div>
                        <h5>Latest Blog</h5>
                        <div class="row">
                            <div class="card">
                                <div class="card-body">
                                    <h5 class="card-title"><i class="bi bi-journal-code"></i></h5>
                                    <h6 class="card-subtitle mb-2 text-body-secondary">Card subtitle</h6>
                                    <p class="card-text">Some quick example text to build on the card title and make up the bulk of the cards content.</p>
                                </div>
                            </div>
                            <div class="card">
                                <div class="card-body">
                                    <h5 class="card-title"><i class="bi bi-journal-code"></i></h5>
                                    <h6 class="card-subtitle mb-2 text-body-secondary">Card subtitle</h6>
                                    <p class="card-text">Some quick example text to build on the card title and make up the bulk of the cards content.</p>
                                </div>
                            </div>
                            <div class="card">
                                <div class="card-body">
                                    <h5 class="card-title"><i class="bi bi-journal-code"></i></h5>
                                    <h6 class="card-subtitle mb-2 text-body-secondary">Card subtitle</h6>
                                    <p class="card-text">Some quick example text to build on the card title and make up the bulk of the cards content.</p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}