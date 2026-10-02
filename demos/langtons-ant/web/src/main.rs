fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<langtons_ant_web::app::App>::new().render();
}
