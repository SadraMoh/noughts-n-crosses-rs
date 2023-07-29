use leptos::*;

use crate::grid::Mark;

use super::state::SpotProp;

#[component]
pub fn Spot(
    cx: Scope,
    spot_signal: SpotProp,
    turn: ReadSignal<Mark>,
    winner: ReadSignal<Mark>,
    on_check: Box<dyn Fn(Mark)>,
) -> impl IntoView {
    let (spot, set_spot) = spot_signal;

    let check = move |_| {
        // do not allow clicking when there already is a winner
        if winner() != Mark::Empty {
            return;
        }

        // do not allow clicking on pre-occupied spots
        if spot() != Mark::Empty {
            return;
        }

        let new_mark = turn();
        set_spot(new_mark.clone());

        (on_check)(Mark::Cross)
    };

    view! {
      cx,
      <div>
        <button class=move || spot().to_string() on:click=check>{spot}</button>
      </div>
    }
}
