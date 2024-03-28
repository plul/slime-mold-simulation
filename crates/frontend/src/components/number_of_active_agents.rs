use common::AppToServer;
use common::Command;
use common::SetParameter;
use patternfly_yew::components::form::Form;
use patternfly_yew::components::form::FormGroup;
use patternfly_yew::components::form::Radio;
use patternfly_yew::components::title::Level;
use patternfly_yew::components::title::Title;
use yew::function_component;
use yew::html;
use yew::use_context;
use yew::Callback;
use yew::Html;

#[function_component]
pub fn NumberOfActiveAgents() -> Html {
    let ctx = use_context::<crate::Context>().expect("no context found");

    let value: Option<u32> = ctx.simulation_status.map(|s| s.parameters.shader_parameters.number_of_active_agents);

    let disabled: bool = value.is_none();

    let options = [1_u32, 10, 100, 1000, 10000, 100000, 500000].into_iter().map(|n| {
        let checked = value == Some(n);
        (n, checked, callback(&ctx, n))
    });

    html! {
        <>
        <Title level={Level::H3}>{format!("Number of active agents: {}", value.unwrap_or_default())}</Title>

        <Form>
            <FormGroup>
                <>
                    { for options.map(|(n, checked, cb)| html! { <Radio disabled={disabled} onchange={cb} checked={checked}>{n.to_string()}</Radio> } ) }
                </>
            </FormGroup>
        </Form>

        <Slider />
        </>
    }
}

fn callback(ctx: &crate::Context, n: u32) -> Callback<(), ()> {
    let tx = ctx.app_to_server_tx.clone();
    Callback::from(move |()| {
        if let Err(err) = tx.unbounded_send(AppToServer::Command(Command::SetParameter(SetParameter::NumberOfActiveAgents(n)))) {
            log::error!("Failed to emit app to server command: {err}");
        };
    })
}

#[yew::function_component]
fn Slider() -> yew::Html {
    let get_min = yew::Callback::from(|_| 0.0);
    let get_max = yew::Callback::from(|s: common::SimulationStatus| s.parameters.number_of_agents as f32);
    let get_value = yew::Callback::from(|s: common::SimulationStatus| s.parameters.shader_parameters.number_of_active_agents as f32);
    let fn_command = yew::Callback::from(|val: f32| SetParameter::NumberOfActiveAgents(val.floor() as u32));
    yew::html! {
        <super::my_slider::MySlider {get_min} {get_max} {get_value} {fn_command}/>
    }
}
