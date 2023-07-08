use leptos::*;

use crate::grid::{Mark, Spot};

#[component]
pub fn Grid(cx: Scope) -> impl IntoView {
    let spots = (0..9)
        .map(|_| create_signal::<Mark>(cx, Mark::Empty))
        .map(|signal| {
            view! { cx,
              <Spot spot_signal=signal />
            }
        })
        .collect::<Vec<_>>();

    view! {
      cx,
      <div class="grid">
        {spots}
      </div>
    }
}

// #[component]
// pub fn Grid(cx: Scope) -> impl IntoView {
//     let initial_spots = (0..9)
//         .map(|id| (id, create_signal(cx, id)))
//         .collect::<Vec<_>>();

//     let (spots, set_spots) = create_signal(cx, initial_spots);

//     view! {
//         cx,
//         <div>"This is the grid component"</div>
//         <For
//         each=spots
//         key= |spot| spot.0 // id
//         view=move |cx, (id, (spot, set_spot)) | {
//             view! { cx,
//               <div>{spot}</div>
//               <button on:click=move |_| set_spot.update(|n| *n += 1)>"+"</button>
//             }
//         }
//     />
//       }
// }
