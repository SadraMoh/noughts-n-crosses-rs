use leptos::*;

use crate::grid::{Mark, Spot};

#[component]
pub fn Grid(cx: Scope) -> impl IntoView {
    let (turn, set_turn) = create_signal(cx, Mark::Nought);
    let (winner, set_winner) = create_signal(cx, Mark::Empty);
    let spots = [create_signal(cx, Mark::Empty); 9];

    let on_check = move |new_mark: Mark| {
        let values = spots.iter().map(|(read, _)| read()).collect::<Vec<_>>();
        if check_for_winner(values) {
            set_winner(turn());
            log!("{} won!", turn().to_string());
            return;
        }

        set_turn.set(turn().opposite());
        log!("{}", new_mark.to_string());
    };

    view! {
      cx,
      <div class="grid">
        {
          spots.iter().map(|spot_signal| {
              view! { cx,
                <Spot spot_signal turn on_check=Some(Box::new(on_check))  />
              }
          })
          .collect::<Vec<_>>();
        }
      </div>
    }
}

fn check_for_winner(marks: Vec<Mark>) -> bool {
    vec![
        // horizontal
        marks.chunks(3).any(|chunk| match chunk {
            [x, y, z] if x == y && y == z && x == z => true,
            _ => false,
        }),
        // vertical
        (0..3)
            .map(|c| (marks[c], marks[c + 3], marks[c + 6]))
            .any(|chunk| match chunk {
                (x, y, z) if x == y && y == z && x == z => true,
                _ => false,
            }),
        // diagonal
        match (marks[0], marks[8], marks[5], marks[2], marks[6]) {
            (x, _, y, _, z) if x == y && y == z && x == z => true,
            (_, x, y, z, _) if x == y && y == z && x == z => true,
            _ => false,
        },
    ]
    .iter()
    .any(|f| *f)
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
