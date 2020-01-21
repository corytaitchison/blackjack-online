use bjo::Model;
use stdweb::web::{document, IParentNode};
use yew::html::Scope;
use yew::App;

fn mount_app(selector: &'static str, app: App<Model>) -> Scope<Model> {
    let element = document().query_selector(selector).unwrap().unwrap();
    app.mount(element)
}

fn main() {
    yew::initialize();
    let app: App<Model> = App::new();
    mount_app(".app", app);
    yew::run_loop();
}
