fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<frontend::Application>::new().render();
}
