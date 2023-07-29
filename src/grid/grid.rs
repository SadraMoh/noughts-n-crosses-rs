use leptos::*;

use crate::grid::{Mark, Spot};

#[component]
pub fn Grid(cx: Scope) -> impl IntoView {
    let (turn, set_turn) = create_signal(cx, Mark::Nought);
    let (winner, set_winner) = create_signal(cx, Mark::Empty);

    let (spots, _) = create_signal(
        cx,
        (0..9)
            .map(|_| create_signal(cx, Mark::Empty))
            .collect::<Vec<_>>(),
    );

    view! {
      cx,
      <div class="container">
        <div class=move || format!("winner {} {}", winner().to_string(), if winner() == Mark::Empty { "hidden" } else { "" })>
            {move || winner().to_string()} " is the winner!"
        </div>
        <div class="grid">
            {
                spots().iter().map(move |spot_signal| {
                    let on_check = Box::new(move |_| {
                        let marks = &spots().iter().map(|(read, _)| read()).collect::<Vec<_>>()[0..9];

                        if check_for_winner(marks, turn()) {
                            set_winner(turn());
                            return;
                        }

                        set_turn.update(|turn| *turn = turn.opposite());
                    });

                    view! { cx,
                        <Spot spot_signal=*spot_signal turn on_check />
                    }
                })
                .collect::<Vec<_>>()
            }
        </div>
      </div>
    }
}

fn check_for_winner(marks: &[Mark], turn: Mark) -> bool {
    const WIN_SCENARIOS: [[usize; 3]; 8] = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];

    WIN_SCENARIOS.iter().any(|scenario| match scenario {
        [x, y, z] if marks[*x] == turn && marks[*y] == turn && marks[*z] == turn => true,
        _ => false,
    })
}
