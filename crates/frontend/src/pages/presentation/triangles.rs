use patternfly_yew::components::title::Level;
use patternfly_yew::components::title::Title;
use yew::function_component;
use yew::html;
use yew::Html;

#[function_component]
pub fn Triangles() -> Html {
    html! {
        <>
            <Title level={Level::H2}>{"Triangles"}</Title>

            <crate::components::number_of_triangles::NumberOfTriangles/>

            <crate::components::VertexStretch/>

            <crate::components::SwitchRenderTrailMap/>
        </>
    }
}
