use leptos::prelude::*;

use crate::components::clock::Clock;

#[allow(non_snake_case)]
#[component]
pub fn Contact() -> impl IntoView {
    
    view! {
        <section id="contact" class="contact section" data-aos="fade-right">     
            <div class="container section-title" data-aos="slide-right" data-aos-delay="100">
                <h2>Contact Me</h2>
                <p>I am a Software Engineer dedicated to building efficient, scalable, and user-friendly digital solutions. With a strong background in web development, I am used to working with various modern technologies such as JavaScript/TypeScript, Svelte, Rust, and various other frameworks.</p>
            </div>   
             <div class="container" data-aos="slide-right" data-aos-delay="200">
                <div class="row">
                    <div class="col-12 mb-3 d-flex gap-2 justify-content-start">
                        <a class="btn btn-success" href="https://wa.me/6282323443535" target="_blank">
                            <i class="bi bi-whatsapp"></i> WhatsApp
                        </a>
                        <a class="btn btn-primary" href="https://wa.me/6282323443535" target="_blank">
                            <i class="bi bi-linkedin"></i> Linkedin
                        </a>
                        <a class="btn btn-info" href="https://wa.me/6282323443535" target="_blank">
                            <i class="bi bi-twitter"></i> Twitter
                        </a>
                        <a class="btn btn-dark" href="https://wa.me/6282323443535" target="_blank">
                            <i class="bi bi-github"></i> Github
                        </a>
                        <a class="btn btn-danger" href="https://wa.me/6282323443535" target="_blank">
                            <i class="bi bi-instagram"></i> Instagram
                        </a>
                    </div>
                    <div class="col-lg-6 mb-3">
                        <h4><i class="bi bi-alarm"></i> Availability</h4>
                        <div class="row">
                            <div class="col-12 mb-3">
                                <div class="card p-3">
                                    <p class="card-text"><i class="bi bi-clock"></i> Jakarta Indonesia</p>
                                    <Clock/>
                                </div>
                            </div>
                            <div class="col-12 mb-3">
                                <div class="card p-3">
                                    <div class="alert alert-primary d-flex align-items-center" role="alert">
                                        <i class="bi bi-info-circle me-2"></i>
                                        <div>
                                            Response time: I typically respond within 24-48 hours.
                                        </div>
                                    </div>

                                    <div class="d-flex justify-content-between">
                                        <span>Monday - Friday</span>
                                        <div class="alert alert-primary p-1 d-flex align-items-center" role="alert">
                                            <div>
                                                09:00 - 17:00
                                            </div>
                                        </div>
                                    </div>
                                    <div class="d-flex justify-content-between">
                                        <span>Saturday</span>
                                        <div class="alert alert-primary p-1 d-flex align-items-center" role="alert">
                                            <div>
                                                By Appointment
                                            </div>
                                        </div>
                                    </div>
                                    <div class="d-flex justify-content-between">
                                        <span>Sunday</span>
                                        <div class="alert alert-primary p-1 d-flex align-items-center" role="alert">
                                            <div>
                                                Rest Day
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="col-lg-6 mb-3">
                    <h4><i class="bi bi-envelope"></i> Contact Me</h4>
                        <form class="card p-3">
                            <div class="mb-3">
                                <label for="exampleInputEmail1" class="form-label">Email address</label>
                                <input type="email" class="form-control" id="exampleInputEmail1" aria-describedby="emailHelp"/>
                            </div>
                            <div class="mb-3">
                                <label for="exampleInputPassword1" class="form-label">Password</label>
                                <input type="password" class="form-control" id="exampleInputPassword1"/>
                            </div>
                            <div class="mb-3">
                                <label for="exampleFormControlTextarea1" class="form-label">Message</label>
                                <textarea class="form-control" id="exampleFormControlTextarea1" rows="3"></textarea>
                            </div>
                            <button type="submit" class="btn btn-primary">Submit</button>
                        </form>
                    </div>
                </div>
             </div>     
        </section>
    }
}