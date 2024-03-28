use std::rc::Rc;
use yew::prelude::*;

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct Error {
    // The counter field is a trick so each emission by the reducer is unique.
    // Yew will by default try to minimize the number of re-renders by diffing the new virtual DOM against the previous version.
    // So this makes sure that downstream subscribers to this context are notified.
    _counter: usize,

    pub value: Option<String>,
}

impl Reducible for Error {
    type Action = String;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Error {
            _counter: self._counter + 1,
            value: Some(action),
        }
        .into()
    }
}

pub type ErrorContext = UseReducerHandle<Error>;

#[derive(Properties, Debug, PartialEq)]
pub struct ErrorProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn ErrorProvider(props: &ErrorProviderProps) -> Html {
    let err = use_reducer(Default::default);

    html! {
        <ContextProvider<ErrorContext> context={err}>
            {props.children.clone()}
        </ContextProvider<ErrorContext>>
    }
}
