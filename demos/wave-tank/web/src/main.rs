fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<wave_tank_web::app::App>::new().render();
}
