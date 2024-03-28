use patternfly_yew::components::slider::Slider;
use patternfly_yew::components::slider::SnapMode;
use patternfly_yew::components::slider::Step;
use yew::function_component;
use yew::html;
use yew::use_effect_with;
use yew::use_state;
use yew::Callback;
use yew::Html;
use yew::Properties;
use yew_hooks::use_debounce;
use yew_hooks::use_mut_latest;

#[derive(Clone, PartialEq, Properties)]
pub struct SliderControlProperties {
    /// The minimum value.
    pub min: Step,
    /// The maximum value.
    pub max: Step,

    /// The initial value.
    #[prop_or_default]
    pub value: Option<f32>,

    /// Flag to hide the label.
    #[prop_or_default]
    pub hide_labels: bool,

    /// The precision of the value label.
    #[prop_or(2)]
    pub label_precision: usize,

    #[prop_or_default]
    pub ticks: Vec<Step>,

    /// A callback reporting every change.
    #[prop_or_default]
    pub onchange: Callback<f32>,

    /// A callback reporting changes, passed through a trailing throttle.
    #[prop_or_default]
    pub onchange_debounced: Callback<f32>,

    /// Milliseconds to throttle
    #[prop_or(200)]
    pub debounce_millis: u32,

    #[prop_or_default]
    pub snap_mode: SnapMode,
}

#[function_component]
pub fn SliderControl(props: &SliderControlProperties) -> Html {
    // Keep track of internal value, updated in realtime
    let inner_value = use_state(|| props.value);

    {
        let value = inner_value.clone();
        use_effect_with(props.value, move |&val: &Option<f32>| {
            value.set(val);
        });
    }

    let onchange_ref = use_mut_latest(props.onchange.clone());
    let onchange_debounced_ref = use_mut_latest(props.onchange_debounced.clone());

    let props_value = props.value;

    let debounce = {
        let inner_value = inner_value.clone();
        use_debounce(
            move || {
                let Some(inner_value) = *inner_value else { return };

                // Only emit values once we have an input value
                let Some(prop_value) = props_value else {
                    return;
                };
                // ... and the slider value is different from input value
                if prop_value != inner_value {
                    let onchange_debounced_ref = onchange_debounced_ref.current();
                    let onchange_debounced = onchange_debounced_ref.borrow();
                    onchange_debounced.emit(inner_value);
                }
            },
            props.debounce_millis,
        )
    };

    let onchange = {
        let inner_value = inner_value.clone();
        yew::Callback::from(move |val: f64| {
            let val = val as f32;

            inner_value.set(Some(val));
            debounce.run();

            let onchange_ref = onchange_ref.current();
            let onchange = onchange_ref.borrow();
            onchange.emit(val);
        })
    };

    let value = inner_value.map(f64::from);

    html! {
        <>
        <Slider suppress_initial_change=true min={props.min.clone()} max={props.max.clone()} {value} {onchange}></Slider>
        </>
    }
}
