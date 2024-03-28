use patternfly_yew::components::title::Level;
use patternfly_yew::components::title::Title;
use yew::function_component;
use yew::html;
use yew::Html;

#[function_component]
pub fn ParticleModel() -> Html {
    html! {
        <>
            <Title level={Level::H2}>{"Particle Model"}</Title>

            <crate::components::SwitchRenderTrailMap/>
            <crate::components::SwitchAgentDeposit/>
            <crate::components::SwitchPause/>
            <crate::components::SwitchAgentBounce/>

            <crate::components::number_of_active_agents::NumberOfActiveAgents/>

            <crate::components::SwitchDecay/>
            <crate::components::SwitchDiffuse/>
            <crate::components::SwitchAgentRotate/>

            <crate::components::DecayStrength/>
            <crate::components::DepositStrength/>

            <crate::components::Margin/>
            <hr />
            <crate::components::Margin/>

            <crate::components::AgentSpeed/>
            <crate::components::SensorDistance/>
            <crate::components::SensorAngleDegrees/>
            <crate::components::MaxTurnAngleDegrees/>

            <crate::components::Margin/>
            <hr />
            <crate::components::Margin/>

            <crate::components::SwitchColor/>

            <crate::components::Margin/>
            <hr />
            <crate::components::Margin/>

            <crate::components::SwitchAgentRotateLeft/>
            <crate::components::SwitchAgentRotateRandomly/>
            <crate::components::SwitchAgentRotateRight/>

            <crate::components::Margin/>
            <hr />
            <crate::components::Margin/>

            <crate::components::HighDensityThreshold/>
            <crate::components::HighDensitySpeedBoost/>
            <crate::components::MaxRandTurnAngleDegrees/>

        </>
    }
}
