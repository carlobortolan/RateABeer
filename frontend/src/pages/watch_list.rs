use crate::{api::watch::get_watches, routes::Route};
use yew::{function_component, html, use_effect_with_deps, use_state, Callback, Html, MouseEvent};
use yew_router::history::History;
use yew_router::prelude::use_history;

#[function_component(WatchList)]
pub fn watch_list() -> Html {
    #[allow(clippy::redundant_closure)]
    let watches = use_state(|| Vec::new());
    let has_error = use_state(|| false);
    let history = use_history().expect("history to be available");
    let row_click = |id: i32| -> Callback<MouseEvent> {
        let history = history.clone();
        Callback::once(move |_: yew::MouseEvent| {
            history.push(Route::WatchDetail {
                watch_id: id.to_string(),
            })
        })
    };

    if *has_error {
        history.push(Route::NotFound)
    }

    {
        let watches = watches.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let result = get_watches().await;
                    match result {
                        Ok(resp) => watches.set(resp),
                        // We could check the error and respond differently.
                        Err(_) => has_error.set(true),
                    };
                });
                || ()
            },
            (),
        );
    }

    html! {
        <div class="mt-4 w-75 mx-auto">
          <div class="d-flex flex-column">
            <h1>{ "Watch List" }</h1>
            <table class="table shadow-sm">
                <colgroup>
                    <col width="45%" />
                    <col width="20%" />
                    <col width="10%" />
                    <col width="25%" />
                </colgroup>
                <thead>
                    <th scope="col">{"Name"}</th>
                    <th scope="col">{"Reference"}</th>
                    <th scope="col">{"Rating"}</th>
                    <th scope="col">{"Style"}</th>
                </thead>
                <tbody>
                {
                    watches.clone().iter().map(|watch| {

                    html!{
                        <tr class="pointer" onclick={row_click(watch.id)}>
                            <th scope="row">{&watch.name}</th>
                            <td >{&watch.reference}</td>
                            <td >{format!("{:.2}", &watch.average_rating)}</td>
                            <td >{&watch.style}</td>
                        </tr>
                    }
                }).collect::<Html>()}
                </tbody>
            </table>
          </div>
        </div>
    }
}
