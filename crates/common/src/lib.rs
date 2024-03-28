use serde::Deserialize;
use serde::Serialize;
use std::time::Duration;

pub mod simulation;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ServerToApp {
    /// Keep-alive message.
    HeartBeat,

    /// State (server to app).
    SimulationStatus(SimulationStatus),
}
impl ServerToApp {
    pub fn ser(&self) -> String {
        serde_json::to_string(&self).expect("Failed serialization")
    }

    pub fn deser(text: &str) -> Self {
        serde_json::from_str::<Self>(&text).expect("Failed deserialization")
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SimulationStatus {
    pub parameters: simulation::Parameters,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum AppToServer {
    /// Keep-alive message.
    HeartBeat,

    /// Command
    Command(Command),
}
impl AppToServer {
    pub fn ser(&self) -> String {
        serde_json::to_string(&self).expect("Failed serialization")
    }

    pub fn deser(text: &str) -> Self {
        serde_json::from_str::<Self>(&text).expect("Failed deserialization")
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Command {
    ActivatePreset(u32),

    /// Toggle play/pause of simulation
    TogglePlayPause,

    /// Randomize simulation parameters
    RandomizeParameters,

    /// Reset simulation to initial conditions
    ResetSimulation,

    /// Set single parameter
    SetParameter(SetParameter),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum SetParameter {
    AgentSpeed(f32),
    InitialHeading(simulation::InitialHeading),
    InitialCircleRadius(f32),
    DecayStrength(f32),
    DepositStrength(f32),
    SensorAngleDegrees(f32),
    MaxTurnAngleDegrees(f32),
    MaxRandTurnAngleDegrees(f32),
    HighDensityThreshold(f32),
    HighDensitySpeedBoost(f32),
    SensorDistance(f32),
    VertexStretch(f32),
    NumberOfActiveAgents(u32),
    NumberOfTriangles(simulation::NumberOfTriangles),
    EnableRenderTrailMap(bool),
    EnableAgentBounce(bool),
    EnableAgentDeposit(bool),
    EnableAgentRotate(bool),
    EnableAgentRotateLeft(bool),
    EnableAgentRotateRandomly(bool),
    EnableAgentRotateRight(bool),
    EnableColor(bool),
    EnableDecay(bool),
    EnableDiffuse(bool),
}

pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(2);
