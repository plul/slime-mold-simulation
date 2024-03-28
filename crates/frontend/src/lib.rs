use crate::context::Context;
use components::index::Index;
use components::shader_parameters::ShaderParameters;
use components::InitialConditions;
use patternfly_yew::components::backdrop::BackdropViewer;
use patternfly_yew::components::nav::Nav;
use patternfly_yew::components::nav::NavExpandable;
use patternfly_yew::components::nav::NavList;
use patternfly_yew::components::nav::NavRouterItem;
use patternfly_yew::components::page::Page;
use patternfly_yew::components::page::PageSection;
use patternfly_yew::components::page::PageSectionGroup;
use patternfly_yew::components::page::PageSidebar;
use patternfly_yew::components::toast::ToastViewer;
use patternfly_yew::components::toolbar::GroupVariant;
use patternfly_yew::components::toolbar::Toolbar;
use patternfly_yew::components::toolbar::ToolbarContent;
use patternfly_yew::components::toolbar::ToolbarElementModifier;
use patternfly_yew::components::toolbar::ToolbarGroup;
use patternfly_yew::components::toolbar::ToolbarItem;
use patternfly_yew::core::WithBreakpointExt as _;
use yew::function_component;
use yew::html;
use yew::html_nested;
use yew::use_callback;
use yew::use_reducer;
use yew::use_state;
use yew::Children;
use yew::ContextProvider;
use yew::Html;
use yew::Properties;
use yew_nested_router::Router;
use yew_nested_router::Scope;
use yew_nested_router::Switch;
use yew_nested_router::Target;

mod components;
mod context;
mod hooks;
mod services;

mod pages;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Target)]
pub enum AppRoute {
    #[default]
    Index,
    Parameters(Parameters),
    Presentation(Presentation),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Target)]
pub enum Parameters {
    InitialConditions,
    ShaderParameters,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Target)]
pub enum Presentation {
    Triangles,
    ParticleModel,
}

fn switch_app_route(target: AppRoute) -> Html {
    let parameters = |target: Parameters| match target {
        Parameters::InitialConditions => html! { <InitialConditions /> },
        Parameters::ShaderParameters => html! { <ShaderParameters /> },
    };

    let presentation = |target: Presentation| match target {
        Presentation::Triangles => html! { <pages::presentation::triangles::Triangles /> },
        Presentation::ParticleModel => html! { <pages::presentation::particle_model::ParticleModel /> },
    };

    match target {
        AppRoute::Index => {
            html! {
                <AppPage>
                    <Index/>
                </AppPage>
            }
        }
        AppRoute::Parameters(_) => {
            html!(
                <AppPage>
                    <Scope<AppRoute, Parameters> mapper={AppRoute::mapper_parameters}>
                        <Switch<Parameters> render={parameters}/>
                    </Scope<AppRoute, Parameters>>
                </AppPage>
            )
        }
        AppRoute::Presentation(_) => {
            html!(
                <AppPage>
                    <Scope<AppRoute, Presentation> mapper={AppRoute::mapper_presentation}>
                        <Switch<Presentation> render={presentation}/>
                    </Scope<AppRoute, Presentation>>
                </AppPage>
            )
        }
    }
}

#[function_component]
pub fn Application() -> Html {
    let (app_to_server_tx, app_to_server_rx) = futures::channel::mpsc::unbounded::<common::AppToServer>();

    // Initialize reducible simulation status state
    let context_reducer_handle = use_reducer(|| Context::new(app_to_server_tx));
    // Establish a callback to update the state
    let callback = {
        let dispatcher = context_reducer_handle.dispatcher();
        move |status: common::SimulationStatus| dispatcher.dispatch(status)
    };

    // Start websocket service, which will call the callback whenever it gets a new state from the server
    use_state(|| services::websocket::start(callback, app_to_server_rx));

    let context = (*context_reducer_handle).clone();

    html! {
        <ContextProvider<Context> context={context}>
        <BackdropViewer>
            <ToastViewer>
                <Router<AppRoute> default={AppRoute::Index}>
                    <Switch<AppRoute> render={switch_app_route} />
                </Router<AppRoute>>
            </ToastViewer>
        </BackdropViewer>
        </ContextProvider<Context>>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct PageProps {
    children: Children,
}
#[function_component]
fn AppPage(props: &PageProps) -> Html {
    let sidebar = html_nested! {
        <PageSidebar>
            <Nav>
                <NavList>
                    <NavExpandable title="Presentation" expanded = true>
                        <NavRouterItem<AppRoute> to={AppRoute::Presentation(Presentation::Triangles)}>{"Triangles"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Presentation(Presentation::ParticleModel)}>{"Particle Model"}</NavRouterItem<AppRoute>>
                    </NavExpandable>
                    <NavExpandable title="Parameters" expanded = true>
                        <NavRouterItem<AppRoute> to={AppRoute::Parameters(Parameters::InitialConditions)}>{"InitialConditions"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Parameters(Parameters::ShaderParameters)}>{"ShaderParameters"}</NavRouterItem<AppRoute>>
                    </NavExpandable>
                </NavList>
            </Nav>
        </PageSidebar>
    };

    let onthemeswitch = use_callback((), |state, ()| match state {
        true => gloo_utils::document_element().set_class_name("pf-v5-theme-dark"),
        false => gloo_utils::document_element().set_class_name(""),
    });

    let tools = html!(
        <Toolbar full_height=true>
            <ToolbarContent>
                <ToolbarGroup
                    modifiers={ToolbarElementModifier::Right.all()}
                    variant={GroupVariant::IconButton}
                >
                    <ToolbarItem>
                        <patternfly_yew::prelude::Switch onchange={onthemeswitch} label="Dark Theme" />
                    </ToolbarItem>
                </ToolbarGroup>
            </ToolbarContent>
        </Toolbar>
    );

    html! {
        <Page {sidebar} {tools}>
            <PageSectionGroup>
                <PageSection>
                    { for props.children.iter() }
                </PageSection>
            </PageSectionGroup>
        </Page>
    }
}
