use leptos::IntoView;

#[derive(Clone, PartialEq)]
pub enum Mark {
    Nought,
    Cross,
    Empty,
}

impl Mark {
    pub fn opposite(&self) -> Self {
        match self {
            Mark::Nought => Mark::Cross,
            Mark::Cross => Mark::Nought,
            Mark::Empty => Mark::Empty,
        }
    }
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
