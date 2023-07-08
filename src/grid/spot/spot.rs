use leptos::*;

use crate::grid::Mark;

use super::state::SpotProp;

#[component]
pub fn Spot(cx: Scope, spot_signal: SpotProp) -> impl IntoView {
    let (spot, set_spot) = spot_signal;

    let toggle = move |_| {
        set_spot(match spot() {
            Mark::Empty => Mark::Nought,
            Mark::Nought => Mark::Cross,
            Mark::Cross => Mark::Nought,
        })
    };

    view! {
      cx,
      <div>
        <button class=move || spot().to_string() on:click=toggle>{spot}</button>
      </div>
    }
}
