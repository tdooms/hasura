use hasura::*;
use serde::{Deserialize, Serialize};
use yew::*;

#[derive(Debug, Serialize, Deserialize, Clone, hasura::Object, hasura::Pk)]
#[object(name = "customers", pk = "c_id")]
pub struct Customer {
    c_id: u64,
    member: bool,
    name: String,
    email: Option<String>,
}

pub async fn test() {
    let customers = QueryBuilder::default()
        .returning(Customer::all())
        .build()
        .unwrap();

    let endpoint = "https://pixeltest.hasura.app/v1/graphql";

    let result = query!(customers).send(endpoint).await.unwrap();
    log::info!("{:?}", result);
}

#[function_component(App)]
pub fn app() -> Html {
    use_effect_with_deps(
        |_| {
            wasm_bindgen_futures::spawn_local(test());
            || ()
        },
        (),
    );

    html! {
        "test"
    }
}

pub fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    Renderer::<App>::new().render();
}
