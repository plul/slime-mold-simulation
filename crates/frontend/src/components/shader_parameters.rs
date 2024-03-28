use patternfly_yew::components::title::Level;
use patternfly_yew::components::title::Title;
use yew::function_component;
use yew::html;
use yew::Html;

#[function_component]
pub fn ShaderParameters() -> Html {
    html! {
        <>
        <Title level={Level::H2}>{"Shader Parameters"}</Title>
        <crate::components::Margin/>

        <crate::components::VertexStretch/>

        <crate::components::Margin/>

        <crate::components::number_of_active_agents::NumberOfActiveAgents/>

        <crate::components::Margin/>

        <crate::components::AgentSpeed/>

        <crate::components::Margin/>

        <crate::components::DecayStrength/>
        <crate::components::DepositStrength/>

        <crate::components::Margin/>

        <crate::components::SensorAngleDegrees/>
        <crate::components::MaxTurnAngleDegrees/>
        <crate::components::MaxRandTurnAngleDegrees/>

        <crate::components::Margin/>

        <crate::components::SensorDistance/>

        <crate::components::Margin/>

        <crate::components::HighDensityThreshold/>
        <crate::components::HighDensitySpeedBoost/>

        <crate::components::Margin/>

        <crate::components::SwitchRenderTrailMap/>
        <crate::components::SwitchAgentBounce/>
        <crate::components::SwitchAgentDeposit/>
        <crate::components::SwitchAgentRotate/>
        <crate::components::SwitchAgentRotateLeft/>
        <crate::components::SwitchAgentRotateRandomly/>
        <crate::components::SwitchAgentRotateRight/>
        <crate::components::SwitchColor/>
        <crate::components::SwitchDecay/>
        <crate::components::SwitchDiffuse/>
        </>
    }
}
