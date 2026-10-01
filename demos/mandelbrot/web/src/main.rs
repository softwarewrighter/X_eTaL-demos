fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<mandelbrot_web::app::App>::new().render();
}
