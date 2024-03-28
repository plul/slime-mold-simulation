//! This is the UI component that subscribes to and displays errors.

use super::error_provider::ErrorContext;
use yew::prelude::*;

pub enum Msg {
    ErrorContextUpdated(ErrorContext),
    HideError,
}

pub struct ErrorComponent {
    error: Option<String>,
    _context_listener: ContextHandle<ErrorContext>,
}

impl Component for ErrorComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (error_context, context_listener) = ctx
            .link()
            .context(ctx.link().callback(Msg::ErrorContextUpdated))
            .expect("No Error Context Provided");
        Self {
            error: error_context.value.clone(),
            _context_listener: context_listener,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ErrorContextUpdated(error_context) => {
                log::info!("error context updated");
                self.error = error_context.value.clone();
                true
            }
            Msg::HideError => {
                self.error = None;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_hide = ctx.link().callback(|_| Msg::HideError);
        html! {
            <div>
                { self.view_error(onclick_hide) }
            </div>
        }
    }
}

impl ErrorComponent {
    fn view_error(&self, onclick_hide: Callback<MouseEvent>) -> Html {
        match &self.error {
            Some(error_msg) => {
                html! {
                    <>
                    <div class="row">
                        <h5><i style="margin-right: 1rem" class="fas fa-triangle-exclamation" />{"Error"}</h5>
                        <p>{ error_msg }</p>
                        <button onclick={onclick_hide} class="button-primary">
                            { "Hide" }
                        </button>
                    </div>
                    <hr />
                    </>
                }
            }
            None => html! {},
        }
    }
}
