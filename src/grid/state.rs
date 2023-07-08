use leptos::IntoView;

#[derive(Clone)]
pub enum Mark {
    Nought,
    Cross,
    Empty,
}

impl ToString for Mark {
    fn to_string(&self) -> String {
        match self {
            Mark::Nought => "o",
            Mark::Cross => "x",
            Mark::Empty => "",
        }
        .to_string()
    }
}

impl IntoView for Mark {
    fn into_view(self, cx: leptos::Scope) -> leptos::View {
        self.to_string().into_view(cx)
    }
}
