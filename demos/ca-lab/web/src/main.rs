fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<ca_lab_web::app::App>::new().render();
}
