use crate::grid::Mark;
use leptos::{ReadSignal, WriteSignal};

pub type SpotProp = (ReadSignal<Mark>, WriteSignal<Mark>);
