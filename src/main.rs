//Bug: CSS classes not applied correctly on initial fullstack build
// The desired output is achieved without fullstack. On full stack it is achieved after a hot-patch
// Try editing the p tag which says "Superfluous"
// The input should have a red box-shadow when the page is first opened

use dioxus::prelude::*;

use crate::components::input::Input;

const COMPONENTS: Asset = asset!("/assets/dx-components-theme.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut store: Store<Option<u32>> = use_store(|| None);
    rsx! {
       document::Link { rel: "stylesheet", href: COMPONENTS}
       div{
            SomeInput {
                store,
                error_condition: move |str: String|{
                    match str.parse::<u32>(){
                        Ok(val) => Some(val),
                        _ => None,
                    }
                }
             }
             //Change the below and the desired behavior appears
             p{
                 "Superfluous"
             }
       }
    }
}

mod components;

#[component]
fn SomeInput<T: 'static + Clone>(
    store: Store<Option<T>>,
    error_condition: Callback<String, Option<T>>,
) -> Element {
    let class = match *store.read() {
        Some(_) => "input correct",
        None => "input errored",
    };
    rsx! {
        Input{
            class,
            oninput: move |evt: FormEvent|{
                store.set(error_condition.call(evt.value()));
                info!("{class}")
            }
        }
    }
}
