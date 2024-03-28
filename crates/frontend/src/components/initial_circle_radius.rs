use crate::components::slider_control::SliderControl;
use common::AppToServer;
use common::Command;
use common::SetParameter;
use patternfly_yew::components::slider::Step;
use patternfly_yew::components::title::Level;
use patternfly_yew::components::title::Title;
use yew::function_component;
use yew::html;
use yew::use_context;
use yew::use_state;
use yew::Html;

#[function_component]
pub fn InitialCircleRadius() -> Html {
    let ctx = use_context::<crate::Context>().expect("no context found");
    let cfg = ctx.simulation_parameters.as_ref().map(|s| &s.runtime_config);

    let fmt_val = use_state(String::new);

    let min = Step { value: 10.0, label: None };
    let max = Step { value: 2000.0, label: None };
    let value = cfg.map(|c| c.initial_conditions.initial_circle_radius).map(Into::<f64>::into);
    log::info!("value: {value:?}");

    let onchange = {
        let fmt_val = fmt_val.clone();
        move |val: f64| {
            log::info!("onchange: {val}");
            fmt_val.set(format!("{val:.2}"));
        }
    };

    let onchange_debounced = {
        move |val: f64| {
            log::info!("onchange debounced: {val}");
            log::info!("Sending initial circle radius: {val}");
            if let Err(err) = ctx
                .app_to_server_tx
                .unbounded_send(AppToServer::Command(Command::SetParameter(SetParameter::InitialCircleRadius(val as f32))))
            {
                log::error!("Failed to emit app to server command: {err}");
            };
        }
    };

    html! {
        <>
        <Title level={Level::H3}>{"Initial Circle Radius:"} {(*fmt_val).clone()}</Title>
        <SliderControl {min} {max} {value} {onchange} {onchange_debounced}></SliderControl>
        </>
    }
}
