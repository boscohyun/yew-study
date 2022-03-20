use yew::start_app;
mod seonbi;

fn main() {
    tracing_wasm::set_as_global_default();
    start_app::<seonbi::Seonbi>();
}
