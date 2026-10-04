fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<fourier_epicycles_web::app::App>::new().render();
}
