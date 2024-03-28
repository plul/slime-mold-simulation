use common::SetParameter;
use yew::use_context;

pub mod error;
pub mod index;
pub mod initial_heading;
pub mod number_of_active_agents;
pub mod shader_parameters;
pub mod slider_control;

fn bool_from_u32(v: u32) -> bool {
    match v {
        0 => false,
        1 => true,
        _ => panic!("non-bool value: {v}"),
    }
}

pub mod number_of_triangles {
    use super::*;
    use common::AppToServer;
    use common::Command;
    use patternfly_yew::components::form::Form;
    use patternfly_yew::components::form::FormGroup;
    use patternfly_yew::components::form::Radio;
    use patternfly_yew::components::title::Level;
    use patternfly_yew::components::title::Title;
    use yew::html;
    use yew::Callback;

    #[yew::function_component]
    pub(crate) fn NumberOfTriangles() -> yew::Html {
        let ctx = use_context::<crate::Context>().expect("no context found");
        let value: Option<common::simulation::NumberOfTriangles> = ctx.simulation_status.map(|s| s.parameters.number_of_triangles);
        let disabled: bool = value.is_none();
        let options = [
            common::simulation::NumberOfTriangles::Zero,
            common::simulation::NumberOfTriangles::One,
            common::simulation::NumberOfTriangles::Two,
        ]
        .into_iter()
        .map(|n| {
            let checked = value == Some(n);
            (n, checked, callback(&ctx, n))
        });
        yew::html! {
            <>
            <crate::components::Margin/>
            <Title level={Level::H3}>{"Number of triangles"}</Title>
            <Form>
                <FormGroup>
                    <>
                        { for options.map(|(n, checked, cb)| html! { <Radio disabled={disabled} onchange={cb} checked={checked}>{n.to_u32()}</Radio> } ) }
                    </>
                </FormGroup>
            </Form>
            <crate::components::Margin/>
            </>
        }
    }

    fn callback(ctx: &crate::Context, number_of_triangles: common::simulation::NumberOfTriangles) -> Callback<(), ()> {
        let tx = ctx.app_to_server_tx.clone();
        Callback::from(move |()| {
            if let Err(err) = tx.unbounded_send(AppToServer::Command(Command::SetParameter(SetParameter::NumberOfTriangles(
                number_of_triangles,
            )))) {
                log::error!("Failed to emit app to server command: {err}");
            };
        })
    }
}

#[yew::function_component]
pub(crate) fn Margin() -> yew::Html {
    yew::html! {
        <div style="margin-bottom: 2em; margin-top: 2em"/>
    }
}

#[yew::function_component]
pub(crate) fn SwitchPause() -> yew::Html {
    let get_value = yew::Callback::from(|s: common::SimulationStatus| !s.parameters.paused);
    let fn_command = yew::Callback::from(|_val: bool| common::Command::TogglePlayPause);
    yew::html! {
        <my_switch::MySwitch {get_value} {fn_command} description={"Run Simulation"}/>
    }
}

#[yew::function_component]
pub(crate) fn SwitchRenderTrailMap() -> yew::Html {
    let get_value = yew::Callback::from(|s: common::SimulationStatus| bool_from_u32(s.parameters.shader_parameters.bool_enable_render_trail_map));
    let fn_command = yew::Callback::from(|val: bool| common::Command::SetParameter(SetParameter::EnableRenderTrailMap(val)));
    yew::html! {
        <my_switch::MySwitch {get_value} {fn_command} description={"Render Trail Map"}/>
    }
}

#[yew::function_component]
pub(crate) fn SwitchAgentBounce() -> yew::Html {
    let get_value = yew::Callback::from(|s: common::SimulationStatus| bool_from_u32(s.parameters.shader_parameters.bool_enable_agent_bounce));
    let fn_command = yew::Callback::from(|val: bool| common::Command::SetParameter(SetParameter::EnableAgentBounce(val)));
    yew::html! {
        <my_switch::MySwitch {get_value} {fn_command} description={"Agent Bounce"}/>
    }
}

#[yew::function_component]
pub(crate) fn SwitchAgentDeposit() -> yew::Html {
    let get_value = yew::Callback::from(|s: common::SimulationStatus| bool_from_u32(s.parameters.shader_parameters.bool_enable_agent_deposit));
    let fn_command = yew::Callback::from(|val: bool| common::Command::SetParameter(SetParameter::EnableAgentDeposit(val)));
    yew::html! {
        <my_switch::MySwitch {get_value} {fn_command} description={"Agent Deposit"}/>
    }
}

#[yew::function_component]
pub(crate) fn SwitchAgentRotate() -> yew::Html {
    let get_value = yew::Callback::from(|s: common::SimulationStatus| bool_from_u32(s.parameters.shader_parameters.bool_enable_agent_rotate));
    let fn_command = yew::Callback::from(|val: bool| common::Command::SetParameter(SetParameter::EnableAgentRotate(val)));
    yew::html! {
        <my_switch::MySwitch {get_value} {fn_command} description={"Agent Rotate"}/>
    }
}

#[yew::function_component]
pub(crate) fn SwitchAgentRotateLeft() -> yew::Html {
    let get_value = yew::Callback::from(|s: common::SimulationStatus| bool_from_u32(s.parameters.shader_parameters.bool_enable_agent_rotate_left));
    let fn_command = yew::Callback::from(|val: bool| common::Command::SetParameter(SetParameter::EnableAgentRotateLeft(val)));
    yew::html! {
        <my_switch::MySwitch {get_value} {fn_command} description={"Agent Rotate Left"}/>
    }
}

#[yew::function_component]
pub(crate) fn SwitchAgentRotateRandomly() -> yew::Html {
    let get_value =
        yew::Callback::from(|s: common::SimulationStatus| bool_from_u32(s.parameters.shader_parameters.bool_enable_agent_rotate_randomly));
    let fn_command = yew::Callback::from(|val: bool| common::Command::SetParameter(SetParameter::EnableAgentRotateRandomly(val)));
    yew::html! {
        <my_switch::MySwitch {get_value} {fn_command} description={"Agent Rotate Randomly"}/>
    }
}

#[yew::function_component]
pub(crate) fn SwitchAgentRotateRight() -> yew::Html {
    let get_value = yew::Callback::from(|s: common::SimulationStatus| bool_from_u32(s.parameters.shader_parameters.bool_enable_agent_rotate_right));
    let fn_command = yew::Callback::from(|val: bool| common::Command::SetParameter(SetParameter::EnableAgentRotateRight(val)));
    yew::html! {
        <my_switch::MySwitch {get_value} {fn_command} description={"Agent Rotate Right"}/>
    }
}

#[yew::function_component]
pub(crate) fn SwitchColor() -> yew::Html {
    let get_value = yew::Callback::from(|s: common::SimulationStatus| bool_from_u32(s.parameters.shader_parameters.bool_enable_color));
    let fn_command = yew::Callback::from(|val: bool| common::Command::SetParameter(SetParameter::EnableColor(val)));
    yew::html! {
        <my_switch::MySwitch {get_value} {fn_command} description={"Color"}/>
    }
}

#[yew::function_component]
pub(crate) fn SwitchDecay() -> yew::Html {
    let get_value = yew::Callback::from(|s: common::SimulationStatus| bool_from_u32(s.parameters.shader_parameters.bool_enable_decay));
    let fn_command = yew::Callback::from(|val: bool| common::Command::SetParameter(SetParameter::EnableDecay(val)));
    yew::html! {
        <my_switch::MySwitch {get_value} {fn_command} description={"Decay"}/>
    }
}

#[yew::function_component]
pub(crate) fn SwitchDiffuse() -> yew::Html {
    let get_value = yew::Callback::from(|s: common::SimulationStatus| bool_from_u32(s.parameters.shader_parameters.bool_enable_diffuse));
    let fn_command = yew::Callback::from(|val: bool| common::Command::SetParameter(SetParameter::EnableDiffuse(val)));
    yew::html! {
        <my_switch::MySwitch {get_value} {fn_command} description={"Diffuse"}/>
    }
}

#[yew::function_component]
pub(crate) fn AgentSpeed() -> yew::Html {
    let get_min = yew::Callback::from(|_| 0.0);
    let get_max = yew::Callback::from(|_| 5.0);
    let get_value = yew::Callback::from(|s: common::SimulationStatus| s.parameters.shader_parameters.agent_speed);
    let fn_command = yew::Callback::from(|val: f32| SetParameter::AgentSpeed(val));
    let description = String::from("Agent Speed");
    yew::html! {
        <my_slider::MySlider {get_min} {get_max} {get_value} {fn_command} {description}/>
    }
}

#[yew::function_component]
pub(crate) fn DecayStrength() -> yew::Html {
    let get_min = yew::Callback::from(|_| 0.0);
    let get_max = yew::Callback::from(|_| 1.0);
    let get_value = yew::Callback::from(|s: common::SimulationStatus| s.parameters.shader_parameters.decay_strength);
    let fn_command = yew::Callback::from(|val: f32| SetParameter::DecayStrength(val));
    let description = String::from("Decay Strength");
    yew::html! {
        <my_slider::MySlider {get_min} {get_max} {get_value} {fn_command} {description}/>
    }
}

#[yew::function_component]
pub(crate) fn SensorAngleDegrees() -> yew::Html {
    let get_min = yew::Callback::from(|_| 0.0);
    let get_max = yew::Callback::from(|_| 180.0);
    let get_value = yew::Callback::from(|s: common::SimulationStatus| s.parameters.shader_parameters.sensor_angle_degrees);
    let fn_command = yew::Callback::from(|val: f32| SetParameter::SensorAngleDegrees(val));
    let description = String::from("Sensor Angle (degrees)");
    yew::html! {
        <my_slider::MySlider {get_min} {get_max} {get_value} {fn_command} {description}/>
    }
}

#[yew::function_component]
pub(crate) fn SensorDistance() -> yew::Html {
    let get_min = yew::Callback::from(|_| 0.0);
    let get_max = yew::Callback::from(|_| 200.0);
    let get_value = yew::Callback::from(|s: common::SimulationStatus| s.parameters.shader_parameters.sensor_distance);
    let fn_command = yew::Callback::from(|val: f32| SetParameter::SensorDistance(val));
    let description = String::from("Sensor Distance");
    yew::html! {
        <my_slider::MySlider {get_min} {get_max} {get_value} {fn_command} {description}/>
    }
}

#[yew::function_component]
pub(crate) fn VertexStretch() -> yew::Html {
    let get_min = yew::Callback::from(|_| 0.0);
    let get_max = yew::Callback::from(|_| 1.0);
    let get_value = yew::Callback::from(|s: common::SimulationStatus| s.parameters.shader_parameters.vertex_stretch);
    let fn_command = yew::Callback::from(|val: f32| SetParameter::VertexStretch(val));
    let description = String::from("Vertex Stretch");
    yew::html! {
        <my_slider::MySlider {get_min} {get_max} {get_value} {fn_command} {description}/>
    }
}

#[yew::function_component]
pub(crate) fn MaxRandTurnAngleDegrees() -> yew::Html {
    let get_min = yew::Callback::from(|_| 0.0);
    let get_max = yew::Callback::from(|_| 180.0);
    let get_value = yew::Callback::from(|s: common::SimulationStatus| s.parameters.shader_parameters.max_rand_turn_angle_degrees);
    let fn_command = yew::Callback::from(|val: f32| SetParameter::MaxRandTurnAngleDegrees(val));
    let description = String::from("Max Random Turn Angle (degrees)");
    yew::html! {
        <my_slider::MySlider {get_min} {get_max} {get_value} {fn_command} {description}/>
    }
}

#[yew::function_component]
pub(crate) fn MaxTurnAngleDegrees() -> yew::Html {
    let get_min = yew::Callback::from(|_| 0.0);
    let get_max = yew::Callback::from(|_| 180.0);
    let get_value = yew::Callback::from(|s: common::SimulationStatus| s.parameters.shader_parameters.max_turn_angle_degrees);
    let fn_command = yew::Callback::from(|val: f32| SetParameter::MaxTurnAngleDegrees(val));
    let description = String::from("Turn Angle (degrees)");
    yew::html! {
        <my_slider::MySlider {get_min} {get_max} {get_value} {fn_command} {description}/>
    }
}

#[yew::function_component]
pub(crate) fn DepositStrength() -> yew::Html {
    let get_min = yew::Callback::from(|_| 0.0);
    let get_max = yew::Callback::from(|_| 1.0);
    let get_value = yew::Callback::from(|s: common::SimulationStatus| s.parameters.shader_parameters.deposit_strength);
    let fn_command = yew::Callback::from(|val: f32| SetParameter::DepositStrength(val));
    let description = String::from("Deposit Strength");
    yew::html! {
        <my_slider::MySlider {get_min} {get_max} {get_value} {fn_command} {description}/>
    }
}

#[yew::function_component]
pub(crate) fn HighDensitySpeedBoost() -> yew::Html {
    let get_min = yew::Callback::from(|_| 0.0);
    let get_max = yew::Callback::from(|_| 20.0);
    let get_value = yew::Callback::from(|s: common::SimulationStatus| s.parameters.shader_parameters.high_density_speed_boost);
    let fn_command = yew::Callback::from(|val: f32| SetParameter::HighDensitySpeedBoost(val));
    let description = String::from("High Density Speed Boost");
    yew::html! {
        <my_slider::MySlider {get_min} {get_max} {get_value} {fn_command} {description}/>
    }
}

#[yew::function_component]
pub(crate) fn HighDensityThreshold() -> yew::Html {
    let get_min = yew::Callback::from(|_| 0.0);
    let get_max = yew::Callback::from(|_| 1.0);
    let get_value = yew::Callback::from(|s: common::SimulationStatus| s.parameters.shader_parameters.high_density_threshold);
    let fn_command = yew::Callback::from(|val: f32| SetParameter::HighDensityThreshold(val));
    let description = String::from("High Density Threshold");
    yew::html! {
        <my_slider::MySlider {get_min} {get_max} {get_value} {fn_command} {description}/>
    }
}

#[yew::function_component]
pub(crate) fn InitialCircleRadius() -> yew::Html {
    let get_min = yew::Callback::from(|_| 10.0);
    let get_max = yew::Callback::from(|_| 2000.0);
    let get_value = yew::Callback::from(|s: common::SimulationStatus| s.parameters.initial_conditions.initial_circle_radius);
    let fn_command = yew::Callback::from(|val: f32| SetParameter::InitialCircleRadius(val));
    let description = String::from("Initial Circle Radius");
    yew::html! {
        <my_slider::MySlider {get_min} {get_max} {get_value} {fn_command} {description}/>
    }
}

#[yew::function_component]
pub(crate) fn InitialConditions() -> yew::Html {
    use patternfly_yew::components::title::Level;
    use patternfly_yew::components::title::Title;
    yew::html! {
        <>
        <Title level={Level::H2}>{"Initial Conditions"}</Title>
        <hr />

        <initial_heading::InitialHeading />
        <InitialCircleRadius />

        </>
    }
}

mod my_switch {
    use patternfly_yew::components::switch::Switch;
    use patternfly_yew::components::title::Level;
    use patternfly_yew::components::title::Title;

    #[derive(yew::Properties, PartialEq)]
    pub struct Props {
        #[prop_or_default]
        pub description: Option<String>,
        pub get_value: yew::Callback<common::SimulationStatus, bool>,
        pub fn_command: Option<yew::Callback<bool, common::Command>>,
    }

    #[yew::function_component]
    pub(crate) fn MySwitch(props: &Props) -> yew::Html {
        let ctx = yew::use_context::<crate::Context>().expect("no context found");
        let status = ctx.simulation_status;

        let Props {
            description,
            get_value,
            fn_command,
        } = props;

        let Some(&status) = status.as_ref() else {
            return yew::html! {
                <>
                <p>{"no status..."}</p>
                </>
            };
        };

        let checked = get_value.emit(status);
        let state = if checked { "Enabled" } else { "Disabled" };

        let onchange = {
            let fn_command = fn_command.as_ref().map(Clone::clone);
            move |val: bool| {
                log::info!("onchange fired");
                if let Some(fn_command) = fn_command.as_ref() {
                    let command: common::Command = fn_command.emit(val);
                    if let Err(err) = ctx.app_to_server_tx.unbounded_send(common::AppToServer::Command(command)) {
                        log::error!("Failed to emit app to server command: {err}");
                    };
                }
            }
        };

        yew::html! {
        <>
            <crate::components::Margin/>
            if let Some(d) = description {
                <Title level={Level::H3}>{ d }</Title>
            }
            <Switch {checked} {onchange} label={state}></Switch>
            <crate::components::Margin/>
        </>
        }
    }
}

mod my_slider {
    #[derive(yew::Properties)]
    pub struct Props {
        #[prop_or_default]
        pub description: Option<String>,
        pub get_min: yew::Callback<common::SimulationStatus, f32>,
        pub get_max: yew::Callback<common::SimulationStatus, f32>,
        pub get_value: yew::Callback<common::SimulationStatus, f32>,
        pub fn_command: Option<yew::Callback<f32, common::SetParameter>>,
    }
    impl PartialEq for Props {
        fn eq(&self, _other: &Self) -> bool {
            false
        }
    }

    #[yew::function_component]
    pub(crate) fn MySlider(props: &Props) -> yew::Html {
        let Props {
            get_min,
            get_max,
            get_value,
            fn_command,
            description,
        } = props;
        let description = description.to_owned();

        let value = yew::use_state_eq(|| None::<f32>);

        let ctx = yew::use_context::<crate::Context>().expect("no context found");
        let status = ctx.simulation_status;

        // Debounce incoming context values
        {
            let value = value.clone();
            let get_value = get_value.clone();
            let incoming_value = status.map(|s| get_value.emit(s));
            yew_hooks::use_debounce_effect_with_deps(
                move || {
                    value.set(status.map(|s| get_value.emit(s)));
                },
                150,
                incoming_value,
            );
        }

        let Some(&status) = status.as_ref() else {
            return yew::html! {
                <>
                <p>{"no status..."}</p>
                </>
            };
        };

        // If we don't have a value yet and the context does, don't wait for the debounce then. Set it immediately:
        if (*value).is_none() {
            let val = get_value.emit(status);
            value.set(Some(val));
        }

        let Some(val) = *value else {
            return yew::html! {
                <>
                <p>{"waiting for value..."}</p>
                </>
            };
        };

        log::debug!(
            "{}: ctx value: {}, our value: {:?}",
            description.clone().unwrap_or_default(),
            get_value.emit(status),
            val
        );

        let min = get_min.emit(status);
        let max = get_max.emit(status);
        // let value = get_value.emit(status);

        use patternfly_yew::components::slider::Step;
        let min = Step {
            value: min as f64,
            label: None,
        };
        let max = Step {
            value: max as f64,
            label: None,
        };

        let onchange = {
            let fn_command = fn_command.as_ref().map(Clone::clone);
            let value = value.clone();
            move |val: f32| {
                log::info!("onchange: {val}");
                value.set(Some(val));

                if let Some(fn_command) = fn_command.as_ref() {
                    let set_parameter: common::SetParameter = fn_command.emit(val);
                    log::info!("Sending value: {val}");

                    if let Err(err) = ctx
                        .app_to_server_tx
                        .unbounded_send(common::AppToServer::Command(common::Command::SetParameter(set_parameter)))
                    {
                        log::error!("Failed to emit app to server command: {err}");
                    };
                }
            }
        };

        use crate::components::slider_control::SliderControl;
        use patternfly_yew::components::title::Level;
        use patternfly_yew::components::title::Title;

        let fmt_val = if let Some(val) = *value { format!("{val:.2}") } else { String::new() };

        yew::html! {
        <>
            if let Some(d) = description {
                <Title level={Level::H3}>{ format!("{d}: {fmt_val}") }</Title>
            }

            <p>
                <SliderControl {min} {max} value={val} {onchange}></SliderControl>
            </p>
        </>
        }
    }
}
