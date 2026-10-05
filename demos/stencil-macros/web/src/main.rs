fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<stencil_macros_web::app::App>::new().render();
}
