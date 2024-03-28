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
pub fn InitialHeading() -> Html {
    let ctx = use_context::<crate::Context>().expect("no context found");

    let on_inward = callback(&ctx, common::simulation::InitialHeading::Inward);
    let on_outward = callback(&ctx, common::simulation::InitialHeading::Outward);
    let on_random = callback(&ctx, common::simulation::InitialHeading::Random);

    let value: Option<common::simulation::InitialHeading> = ctx.simulation_status.map(|s| s.parameters.initial_conditions.initial_heading);
    let disabled: bool = value.is_none();

    let inward_checked: bool = value.map(|v| matches!(v, common::simulation::InitialHeading::Inward)).unwrap_or_default();
    let outward_checked: bool = value
        .map(|v| matches!(v, common::simulation::InitialHeading::Outward))
        .unwrap_or_default();
    let random_checked: bool = value.map(|v| matches!(v, common::simulation::InitialHeading::Random)).unwrap_or_default();

    html! {
        <>
        <Title level={Level::H3}>{"Initial Heading"}</Title>

        <Form>
            <FormGroup>
                <Radio name="Inward" disabled={disabled} onchange={on_inward} checked={inward_checked}>{"Inward"}</Radio>
                <Radio name="Outward" disabled={disabled} onchange={on_outward} checked={outward_checked}>{"Outward"}</Radio>
                <Radio name="Random" disabled={disabled} onchange={on_random} checked={random_checked}>{"Random"}</Radio>
            </FormGroup>
        </Form>
        </>
    }
}

fn callback(ctx: &crate::Context, initial_heading: common::simulation::InitialHeading) -> Callback<(), ()> {
    let tx = ctx.app_to_server_tx.clone();
    Callback::from(move |()| {
        if let Err(err) = tx.unbounded_send(AppToServer::Command(Command::SetParameter(SetParameter::InitialHeading(initial_heading)))) {
            log::error!("Failed to emit app to server command: {err}");
        };
    })
}
