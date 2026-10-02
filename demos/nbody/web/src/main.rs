fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<nbody_web::app::App>::new().render();
}
