use leptos::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn Contact() -> impl IntoView {
    
    view! {
        <section id="contact" class="contact section" data-aos="fade-right">     
            <div class="container section-title" data-aos="slide-right" data-aos-delay="100">
                <h2>Contact</h2>
                <p>I am a Software Engineer dedicated to building efficient, scalable, and user-friendly digital solutions. With a strong background in web development, I am used to working with various modern technologies such as JavaScript/TypeScript, Svelte, Rust, and various other frameworks.</p>
            </div>   
             <div class="container" data-aos="slide-right" data-aos-delay="200">
                Halo
             </div>     
        </section>
    }
}