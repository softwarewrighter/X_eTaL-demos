fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<eigencube_web::app::App>::new().render();
}
