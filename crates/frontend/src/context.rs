use common::AppToServer;
use std::rc::Rc;
use yew::Reducible;

/// Context provided to the entire component tree.
#[derive(Clone)]
pub struct Context {
    /// Status of the backend simulation.
    pub simulation_status: Option<common::SimulationStatus>,

    /// Channel to send messages to the backend.
    pub app_to_server_tx: futures::channel::mpsc::UnboundedSender<AppToServer>,

    /// Incrementing counter.
    ///
    /// The PartialEq implementation compares this field only.
    nonce: usize,
}
impl Context {
    pub fn new(app_to_server_tx: futures::channel::mpsc::UnboundedSender<AppToServer>) -> Context {
        Context {
            simulation_status: None,
            app_to_server_tx,
            nonce: 0,
        }
    }
}
/// Needs to implement `PartialEq` to be useable in a yew ContextProvider, but
/// so we'll have an ever incrementing nonce and compare that.
impl PartialEq for Context {
    fn eq(&self, other: &Self) -> bool {
        self.nonce == other.nonce
    }
}
impl Reducible for Context {
    type Action = common::SimulationStatus;
    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        Rc::new(Context {
            simulation_status: Some(action),
            app_to_server_tx: self.app_to_server_tx.clone(),
            nonce: self.nonce + 1,
        })
    }
}
