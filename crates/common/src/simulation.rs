use crate::SetParameter;
use serde::Deserialize;
use serde::Serialize;
use smart_default::SmartDefault;
use std::cmp::min;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, TypedBuilder)]
pub struct Parameters {
    /// Number of ticks of the simulation to target per second.
    #[builder(default = 60.0)]
    pub target_ticks_per_second: f32,

    /// Number of agents the buffer is initialized with
    #[builder(default = 500_000)]
    pub number_of_agents: u32,

    #[builder(default)]
    pub number_of_triangles: NumberOfTriangles,

    #[builder(default)]
    pub initial_conditions: InitialConditions,

    pub shader_parameters: ShaderParameters,

    #[builder(default = false)]
    pub paused: bool,
}
impl Parameters {
    pub fn toggle_paused(&mut self) {
        self.paused = !self.paused;
    }

    pub fn set(&mut self, set_parameter: crate::SetParameter) {
        match set_parameter {
            SetParameter::AgentSpeed(val) => self.shader_parameters.agent_speed = val,
            SetParameter::InitialHeading(val) => self.initial_conditions.initial_heading = val,
            SetParameter::InitialCircleRadius(val) => {
                self.initial_conditions.initial_circle_radius = val;
            }
            SetParameter::DecayStrength(val) => {
                self.shader_parameters.decay_strength = val;
            }
            SetParameter::DepositStrength(val) => {
                self.shader_parameters.deposit_strength = val;
            }
            SetParameter::SensorAngleDegrees(val) => {
                self.shader_parameters.sensor_angle_degrees = val;
            }
            SetParameter::MaxTurnAngleDegrees(val) => {
                self.shader_parameters.max_turn_angle_degrees = val;
            }
            SetParameter::MaxRandTurnAngleDegrees(val) => {
                self.shader_parameters.max_rand_turn_angle_degrees = val;
            }
            SetParameter::HighDensityThreshold(val) => {
                self.shader_parameters.high_density_threshold = val;
            }
            SetParameter::HighDensitySpeedBoost(val) => {
                self.shader_parameters.high_density_speed_boost = val;
            }
            SetParameter::SensorDistance(val) => {
                self.shader_parameters.sensor_distance = val;
            }
            SetParameter::VertexStretch(val) => {
                self.shader_parameters.vertex_stretch = val;
            }
            SetParameter::NumberOfActiveAgents(val) => {
                let val = min(self.number_of_agents, val);
                self.shader_parameters.number_of_active_agents = val;
            }
            SetParameter::NumberOfTriangles(val) => self.number_of_triangles = val,
            SetParameter::EnableRenderTrailMap(val) => self.shader_parameters.bool_enable_render_trail_map = if val { 1 } else { 0 },
            SetParameter::EnableAgentBounce(val) => self.shader_parameters.bool_enable_agent_bounce = val as u32,
            SetParameter::EnableAgentDeposit(val) => self.shader_parameters.bool_enable_agent_deposit = val as u32,
            SetParameter::EnableAgentRotate(val) => self.shader_parameters.bool_enable_agent_rotate = val as u32,
            SetParameter::EnableAgentRotateLeft(val) => self.shader_parameters.bool_enable_agent_rotate_left = val as u32,
            SetParameter::EnableAgentRotateRandomly(val) => self.shader_parameters.bool_enable_agent_rotate_randomly = val as u32,
            SetParameter::EnableAgentRotateRight(val) => self.shader_parameters.bool_enable_agent_rotate_right = val as u32,
            SetParameter::EnableColor(val) => self.shader_parameters.bool_enable_color = val as u32,
            SetParameter::EnableDecay(val) => self.shader_parameters.bool_enable_decay = val as u32,
            SetParameter::EnableDiffuse(val) => self.shader_parameters.bool_enable_diffuse = val as u32,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, TypedBuilder, Serialize, Deserialize, bytemuck::Zeroable, bytemuck::NoUninit)]
pub struct ShaderParameters {
    /// Agent base speed
    #[builder(default = 1.0)]
    pub agent_speed: f32,

    #[builder(default = 1)]
    pub bool_enable_agent_bounce: u32,

    #[builder(default = 1)]
    pub bool_enable_agent_deposit: u32,

    #[builder(default = 1)]
    pub bool_enable_agent_rotate: u32,

    #[builder(default = 1)]
    pub bool_enable_agent_rotate_left: u32,

    #[builder(default = 1)]
    pub bool_enable_agent_rotate_randomly: u32,

    #[builder(default = 1)]
    pub bool_enable_agent_rotate_right: u32,

    #[builder(default = 1)]
    pub bool_enable_color: u32,

    #[builder(default = 1)]
    pub bool_enable_decay: u32,

    #[builder(default = 1)]
    pub bool_enable_diffuse: u32,

    #[builder(default = 1)]
    pub bool_enable_render_trail_map: u32,

    /// Horizontal dimension of the simulation canvas
    pub canvas_width: u32,

    /// Vertical dimension of the simulation canvas
    pub canvas_height: u32,

    /// Number of active agents
    #[builder(default = 500_000)]
    pub number_of_active_agents: u32,

    /// Vertex Stretch
    #[builder(default = 1.0)]
    pub vertex_stretch: f32,

    /// Decay strength
    #[builder(default = 0.32)]
    pub decay_strength: f32,

    /// Angle for left and right sensors.
    #[builder(default = 54.2)]
    pub sensor_angle_degrees: f32,

    /// Max angle to turn when left or right sensor dictates turn direction.
    #[builder(default = 29.15)]
    pub max_turn_angle_degrees: f32,

    /// Max angle to turn when turning in a random direction.
    #[builder(default = 1.38)]
    pub max_rand_turn_angle_degrees: f32,

    /// Threshold for forced turn due to high agent density (as measured indirectly by deposit strength)
    #[builder(default = 0.56)]
    pub high_density_threshold: f32,

    /// Speed boost for agents in areas of high agent density (as measured indirectly by deposit strength)
    #[builder(default = 0.85)]
    pub high_density_speed_boost: f32,

    /// Deposit strength
    #[builder(default = 0.03)]
    pub deposit_strength: f32,

    /// Sensor distance
    #[builder(default = 13.8)]
    pub sensor_distance: f32,
}

impl ShaderParameters {
    pub fn randomize(&mut self) {
        use rand::Rng as _;
        let mut rng = rand::thread_rng();

        self.decay_strength = rng.gen_range(0.001..0.5);
        self.sensor_angle_degrees = rng.gen_range(1.0..120.0);
        self.max_turn_angle_degrees = rng.gen_range(1.0..60.0);
        self.max_rand_turn_angle_degrees = rng.gen_range(0.0..10.0);
        self.high_density_speed_boost = rng.gen_range(0.0..20.0);
        self.high_density_threshold = rng.gen_range(0.50..1.0);
        self.deposit_strength = rng.gen_range(0.001..0.03);
        self.sensor_distance = rng.gen_range(5.0..14.0);
    }
}

#[derive(Debug, Clone, Copy, SmartDefault, Serialize, Deserialize, PartialEq)]
pub struct InitialConditions {
    /// Radius of circle in which agents are initially distributed
    #[default = 200.0]
    pub initial_circle_radius: f32,

    /// Initial agent direction
    pub initial_heading: InitialHeading,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum InitialHeading {
    #[default]
    Inward,
    Outward,
    Random,
}
impl std::fmt::Display for InitialHeading {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InitialHeading::Inward => write!(f, "Inward"),
            InitialHeading::Outward => write!(f, "Outward"),
            InitialHeading::Random => write!(f, "Random"),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum NumberOfTriangles {
    Zero,
    One,
    #[default]
    Two,
}
impl NumberOfTriangles {
    pub fn to_u32(&self) -> u32 {
        match self {
            NumberOfTriangles::Zero => 0,
            NumberOfTriangles::One => 1,
            NumberOfTriangles::Two => 2,
        }
    }
}
