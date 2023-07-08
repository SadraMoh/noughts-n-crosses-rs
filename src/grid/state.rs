use leptos::IntoView;

#[derive(Clone)]
pub enum Mark {
    Nought,
    Cross,
    Empty,
}

impl IntoView for Mark {
    fn into_view(self, cx: leptos::Scope) -> leptos::View {
        match self {
            Mark::Nought => "0",
            Mark::Cross => "x",
            Mark::Empty => "",
        }
        .into_view(cx)
    }
}
