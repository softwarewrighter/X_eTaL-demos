fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<reaction_diffusion_web::app::App>::new().render();
}
