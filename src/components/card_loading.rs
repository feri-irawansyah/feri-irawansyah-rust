use leptos::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn CardLoading(delay: Option<i32>, count: Option<i32>) -> impl IntoView {
    let count = count.unwrap_or(2);
    let delay = delay.unwrap_or(0);

    view! {
        {(0..count).map(|i| view! {
            <div class=move || format!("col-md-{} d-flex align-items-stretch", 12 / count) data-aos="zoom-in" data-aos-delay={delay} data-aos-duration=format!("{}", i * 100)>
                <div class="card" aria-hidden="true">
                    <img src="/assets/img/loading.png" class="card-img py-2" alt="Ngga ada"/>
                    <div class="card-body card-img-overlay">
                        <h5 class="card-title placeholder-glow">
                            <span class="placeholder col-6"></span>
                        </h5>
                        <p class="card-text placeholder-glow">
                            <span class="placeholder col-7"></span>
                            <span class="placeholder col-4"></span>
                            <span class="placeholder col-4"></span>
                            <span class="placeholder col-6"></span>
                            <span class="placeholder col-8"></span>
                        </p>
                    </div>
                    <div class="card-footer placeholder-glow">
                        <a class="btn btn-primary w-100 disabled placeholder col-6" aria-disabled="true"></a>
                    </div>
                </div>
            </div>
        }).collect_view()}
    }
}
