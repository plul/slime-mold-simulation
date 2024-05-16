use common::simulation::InitialHeading;
use common::simulation::NumberOfTriangles;
use common::simulation::Parameters;
use common::simulation::ShaderParameters;
use common::Command;
use common::SimulationStatus;
use futures::SinkExt as _;
use futures::StreamExt;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;
use tracing_subscriber::EnvFilter;
use type_toppings::ResultExt;
use wgpu::util::DeviceExt as _;
use winit::dpi::PhysicalSize;
use winit::event::DeviceEvent;
use winit::event::ElementState;
use winit::event::Event;
use winit::event::KeyEvent;
use winit::event::StartCause;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::Key;
use winit::keyboard::NamedKey;

mod cli;
mod resolution;

fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .without_time()
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .init();

    // Parse CLI
    let cli = <cli::Cli as clap::Parser>::parse();

    // Determine window size
    let (width, height) = if let Some(crate::resolution::Resolution { width, height }) = cli.resolution {
        // Resolution set on CLI
        (width, height)
    } else {
        // Fallback
        (512, 512)
    };

    // Create event loop
    let event_loop = winit::event_loop::EventLoop::new().unwrap_or_report();

    // Simulation parameters
    let (simulation_parameters_tx, simulation_parameters_rx) = {
        // Initialize parameters
        let mut simulation_parameters = Parameters::builder()
            .shader_parameters(ShaderParameters::builder().canvas_width(width).canvas_height(height).build())
            .build();
        if let Some(agents) = cli.agents {
            simulation_parameters.number_of_agents = agents;
            simulation_parameters.shader_parameters.number_of_active_agents = agents;
        }

        if cli.presentation {
            simulation_parameters.paused = true;
            simulation_parameters.number_of_triangles = NumberOfTriangles::One;
            simulation_parameters.shader_parameters.bool_enable_render_trail_map = 0;
            simulation_parameters.shader_parameters.number_of_active_agents = 1;
            simulation_parameters.shader_parameters.vertex_stretch = 0.0;
            simulation_parameters.initial_conditions.initial_heading = InitialHeading::Random;
            simulation_parameters.shader_parameters.decay_strength = 0.02;
            simulation_parameters.shader_parameters.deposit_strength = 1.0;
            simulation_parameters.shader_parameters.bool_enable_agent_bounce = 0;
            simulation_parameters.shader_parameters.bool_enable_agent_deposit = 0;
            simulation_parameters.shader_parameters.bool_enable_agent_rotate = 0;
            simulation_parameters.shader_parameters.bool_enable_agent_rotate_left = 1;
            simulation_parameters.shader_parameters.bool_enable_agent_rotate_randomly = 1;
            simulation_parameters.shader_parameters.bool_enable_agent_rotate_right = 1;
            simulation_parameters.shader_parameters.bool_enable_color = 0;
            simulation_parameters.shader_parameters.bool_enable_decay = 0;
            simulation_parameters.shader_parameters.bool_enable_diffuse = 0;

            // no dispersion
            simulation_parameters.shader_parameters.max_rand_turn_angle_degrees = 0.0;
            simulation_parameters.shader_parameters.high_density_threshold = 1.0;
            simulation_parameters.shader_parameters.high_density_speed_boost = 0.0;
        }

        tokio::sync::watch::channel(simulation_parameters)
    };

    // User input
    let (user_input_tx, user_input_rx) = std::sync::mpsc::channel();

    let tokio_runtime = tokio::runtime::Runtime::new().unwrap();

    let state = {
        let gpu: Gpu = tokio_runtime.block_on(request_gpu());

        let parameters = { *simulation_parameters_rx.borrow() };
        let resources: Resources = create_resources(&gpu.device, &parameters);

        let surface_texture_format: wgpu::TextureFormat = surface_texture_format();
        let pipelines: Pipelines = create_pipelines(&gpu.device, surface_texture_format, &resources);

        let gpu_mutex = Arc::new(std::sync::Mutex::new(()));
        Arc::new(State {
            gpu,
            resources,
            pipelines,
            gpu_mutex,
        })
    };

    // Launch websocket server
    let ws_addr = cli.ws_addr;
    let tcp_listener_join_handle = tokio_runtime.spawn({
        let simulation_status_rx = simulation_parameters_rx.clone();
        let user_input_tx = user_input_tx.clone();
        async move {
            tracing::info!("Binding TCP listener on {ws_addr}");
            let tcp_listener = tokio::net::TcpListener::bind(&ws_addr)
                .await
                .expect_or_report_with(|| format!("Failed to bind TCP socket to {ws_addr}"));
            tracing::info!("TCP listener listening on: {ws_addr}");
            while let Ok((stream, addr)) = tcp_listener.accept().await {
                let channels = WebSocketHandlerChannels {
                    simulation_parameters_rx: simulation_status_rx.clone(),
                    app_to_server_tx: user_input_tx.clone(),
                };
                tokio::spawn(handle_ws_connection(stream, addr, channels));
            }
        }
    });

    // Spawn thread to handle user input
    let user_input_handling_thread = {
        let state = Arc::clone(&state);
        std::thread::spawn(move || {
            loop {
                match user_input_rx.recv().unwrap() {
                    Command::ActivatePreset(preset) => {
                        let mut p = { *simulation_parameters_tx.borrow() };
                        p.shader_parameters = ShaderParameters::builder()
                            .canvas_width(p.shader_parameters.canvas_width)
                            .canvas_height(p.shader_parameters.canvas_height)
                            .build();

                        match preset {
                            1 => {
                                simulation_parameters_tx.send_replace(p);
                            }
                            2 => {
                                p.shader_parameters.sensor_distance = 55.0;
                                p.shader_parameters.bool_enable_diffuse = 0;
                                p.initial_conditions.initial_heading = InitialHeading::Outward;
                                // no forced dispersion
                                p.shader_parameters.max_rand_turn_angle_degrees = 0.0;
                                p.shader_parameters.high_density_threshold = 1.0;
                                p.shader_parameters.high_density_speed_boost = 0.0;

                                simulation_parameters_tx.send_replace(p);
                            }
                            3 => {
                                p.shader_parameters.sensor_distance = 4.0;
                                p.shader_parameters.bool_enable_diffuse = 0;
                                p.initial_conditions.initial_heading = InitialHeading::Outward;
                                // no forced dispersion
                                p.shader_parameters.max_rand_turn_angle_degrees = 0.0;
                                p.shader_parameters.high_density_threshold = 1.0;
                                p.shader_parameters.high_density_speed_boost = 0.0;

                                simulation_parameters_tx.send_replace(p);
                            }
                            4 => {
                                p.shader_parameters.bool_enable_diffuse = 0;
                                p.shader_parameters.bool_enable_agent_rotate_left = 0;
                                p.shader_parameters.decay_strength = 0.02;
                                p.shader_parameters.sensor_angle_degrees = 65.0;
                                p.shader_parameters.max_turn_angle_degrees = 60.0;

                                // no forced dispersion
                                p.shader_parameters.max_rand_turn_angle_degrees = 0.0;
                                p.shader_parameters.high_density_threshold = 1.0;
                                p.shader_parameters.high_density_speed_boost = 0.0;

                                simulation_parameters_tx.send_replace(p);
                            }
                            _ => {
                                tracing::warn!("Unhandled preset: {preset}");
                            }
                        }
                    }
                    Command::TogglePlayPause => {
                        simulation_parameters_tx.send_modify(Parameters::toggle_paused);
                    }
                    Command::RandomizeParameters => {
                        simulation_parameters_tx.send_modify(|p| p.shader_parameters.randomize());
                    }
                    Command::ResetSimulation => {
                        let simulation_parameters = { *simulation_parameters_tx.borrow() };
                        reset_simulation(&state, &simulation_parameters);
                    }
                    Command::SetParameter(val) => {
                        tracing::debug!("Setting parameter: Before");
                        simulation_parameters_tx.send_modify(|p| p.set(val));
                        tracing::debug!("Setting parameter: After");
                    }
                }
            }
        })
    };

    // Spawn thread to watch for changes to shader parameters and sync them to the corresponding GPU buffer.
    let sync_shader_parameters_task: tokio::task::JoinHandle<_> = {
        let state = Arc::clone(&state);
        let mut simulation_parameters_rx = simulation_parameters_rx.clone();
        tokio_runtime.spawn(async move {
            loop {
                simulation_parameters_rx.changed().await.unwrap();
                let p: Parameters = { *simulation_parameters_rx.borrow_and_update() };
                sync_shader_parameters(&state, &p);
            }
        })
    };

    // Spawn thread to drive the simulation forward by dispatching GPU commands at e.g. 60 UPS
    let simulation_driver_thread = {
        let simulation_parameters_rx = simulation_parameters_rx.clone();
        let state = Arc::clone(&state);
        std::thread::spawn(move || {
            loop {
                let simulation_parameters = { *simulation_parameters_rx.borrow() };

                if !simulation_parameters.paused {
                    tick(&state, &simulation_parameters);
                }

                let nanoseconds_per_tick = 1_000_000_000.0 / simulation_parameters.target_ticks_per_second;
                std::thread::sleep(Duration::from_nanos(nanoseconds_per_tick as u64));
            }
        })
    };

    let event_loop_context = EventLoopContext {
        state,
        simulation_parameters_rx,
        tokio_runtime,
        user_input_tx,
    };
    run_event_loop(event_loop, event_loop_context);
}

struct WebSocketHandlerChannels {
    simulation_parameters_rx: tokio::sync::watch::Receiver<common::simulation::Parameters>,
    app_to_server_tx: std::sync::mpsc::Sender<common::Command>,
}

async fn handle_ws_connection(raw_stream: tokio::net::TcpStream, addr: SocketAddr, channels: WebSocketHandlerChannels) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    // Split websocket read and write halves
    let (mut server_to_app_tx, app_to_server_rx) = ws_stream.split();

    // Deserialize app to server messages
    let app_to_server_rx = app_to_server_rx.filter_map(|r| async {
        match r {
            Ok(msg) => match msg {
                tokio_tungstenite::tungstenite::Message::Binary(data) => {
                    tracing::error!("text instead of binary received: {} bytes", data.len());
                    None
                }
                tokio_tungstenite::tungstenite::Message::Text(text) => {
                    let item = common::AppToServer::deser(&text);
                    Some(item)
                }
                tokio_tungstenite::tungstenite::Message::Ping(_) => None,
                tokio_tungstenite::tungstenite::Message::Pong(_) => None,
                tokio_tungstenite::tungstenite::Message::Close(_) => None,
                tokio_tungstenite::tungstenite::Message::Frame(_) => None,
            },
            Err(err) => {
                tracing::error!("WS error: {err}");
                None
            }
        }
    });

    // Turn channels into streams
    let simulation_parameters_rx = tokio_stream::wrappers::WatchStream::new(channels.simulation_parameters_rx);

    // Heartbeat
    let heartbeat = {
        let interval = tokio::time::interval(common::HEARTBEAT_INTERVAL);
        tokio_stream::wrappers::IntervalStream::new(interval).map(|_| common::ServerToApp::HeartBeat)
    };

    // Map to ServerToApp
    let simulation_parameters_rx = simulation_parameters_rx.map(|parameters| common::ServerToApp::SimulationStatus(SimulationStatus { parameters }));

    // Map streams to common type
    #[derive(Debug)]
    enum Item {
        ServerToApp(common::ServerToApp),
        AppToServer(common::AppToServer),
    }
    let simulation_status_rx = simulation_parameters_rx.map(Item::ServerToApp);
    let heartbeat = heartbeat.map(Item::ServerToApp);
    let app_to_server_rx = app_to_server_rx.map(Item::AppToServer);

    // Merge streams
    let mut streams = futures::stream::select_all([heartbeat.boxed(), simulation_status_rx.boxed(), app_to_server_rx.boxed()]);

    while let Some(item) = streams.next().await {
        match item {
            Item::ServerToApp(msg) => {
                let serialized = msg.ser();
                if let Err(err) = server_to_app_tx.send(tokio_tungstenite::tungstenite::Message::Text(serialized)).await {
                    tracing::error!("Cannot send to client. Quitting handler: {err}");
                    break;
                }
            }
            Item::AppToServer(msg) => match msg {
                common::AppToServer::HeartBeat => {
                    tracing::trace!("Received heartbeat from: {addr}");
                }
                common::AppToServer::Command(command) => {
                    if let Err(err) = channels.app_to_server_tx.send(command) {
                        tracing::error!("Error sending on channel: {err}");
                    };
                }
            },
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Agent {
    position: [f32; 2],
    velocity: [f32; 2],
}
impl Agent {
    fn new_with_random_start_position(simulation_parameters: &Parameters) -> Self {
        let middle = [
            (simulation_parameters.shader_parameters.canvas_width / 2) as f32,
            (simulation_parameters.shader_parameters.canvas_height / 2) as f32,
        ];
        let in_circle = random_point_in_circle(simulation_parameters.initial_conditions.initial_circle_radius);
        let position = [middle[0] + in_circle[0], middle[1] + in_circle[1]];
        let dir = match simulation_parameters.initial_conditions.initial_heading {
            InitialHeading::Inward => normalize(vector_from_a_to_b(position, middle)),
            InitialHeading::Outward => normalize(vector_from_a_to_b(middle, position)),
            InitialHeading::Random => random_normalized_vector(),
        };

        let velocity = dir;
        // TODO: Do I still want this?
        // let velocity = scale(dir, random_number(0.3, 0.5));

        Agent { position, velocity }
    }
}

fn random_normalized_vector() -> [f32; 2] {
    use rand::Rng as _;
    let mut rng = rand::thread_rng();

    // Randomly pick an angle between 0 and 2π.
    use std::f32::consts::PI;
    let theta: f32 = rng.gen_range(0.0..2.0 * PI);

    // Convert the angle to x and y coordinates.
    [theta.cos(), theta.sin()]
}

/// Seriously - shouldn't this just be `b - a`? When am I going to get around to writing a proper 2D Vector struct...
fn vector_from_a_to_b(a: [f32; 2], b: [f32; 2]) -> [f32; 2] {
    [b[0] - a[0], b[1] - a[1]]
}

fn normalize(vec: [f32; 2]) -> [f32; 2] {
    let magnitude = (vec[0].powi(2) + vec[1].powi(2)).sqrt();

    // Check for zero magnitude to prevent division by zero
    if magnitude == 0.0 {
        return [0.0; 2];
    }

    [vec[0] / magnitude, vec[1] / magnitude]
}

fn random_number(from: f32, to: f32) -> f32 {
    rand::Rng::gen_range(&mut rand::thread_rng(), from..=to)
}

fn random_point_in_circle(radius: f32) -> [f32; 2] {
    use rand::Rng as _;
    let mut rng = rand::thread_rng();

    // Randomly pick an angle between 0 and 2π.
    use std::f32::consts::PI;
    let theta: f32 = rng.gen_range(0.0..2.0 * PI);

    // Generate a uniformly distributed random radius inside the circle.
    let r: f32 = (rng.gen::<f32>()).sqrt() * radius;

    // Convert to Cartesian coordinates.
    [r * theta.cos(), r * theta.sin()]
}

fn scale(vec: [f32; 2], factor: f32) -> [f32; 2] {
    [vec[0] * factor, vec[1] * factor]
}

struct CreateWindowConfiguration {
    width: u32,
    height: u32,
}
fn create_winit_window(event_loop: &ActiveEventLoop, config: CreateWindowConfiguration) -> winit::window::Window {
    let CreateWindowConfiguration { width, height } = config;
    let window_size = winit::dpi::PhysicalSize::<u32> { width, height };
    let window_attributes = winit::window::Window::default_attributes()
        .with_title("Slime Mold Simulation")
        .with_inner_size(window_size)
        .with_min_inner_size(window_size)
        .with_theme(Some(winit::window::Theme::Dark));
    event_loop.create_window(window_attributes).unwrap()
}

async fn create_window_surface<'w>(gpu: &Gpu, window: winit::window::Window) -> WindowSurface<'w> {
    let window = Arc::new(window);
    let surface = gpu.instance.create_surface(Arc::clone(&window)).unwrap();
    let texture_format = surface_texture_format();

    let surface_caps = surface.get_capabilities(&gpu.adapter);

    // Configure surface
    let PhysicalSize { width, height } = window.inner_size();
    let surface_configuration = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: texture_format,
        width,
        height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: wgpu::CompositeAlphaMode::Opaque,
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };
    surface.configure(&gpu.device, &surface_configuration);

    WindowSurface {
        surface,
        window,
        texture_format,
    }
}

struct Gpu {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

async fn request_gpu() -> Gpu {
    let instance = wgpu::Instance::default();
    let adapter = request_adapter(&instance, None).await;
    let (device, queue) = request_device(&adapter).await;

    Gpu {
        instance,
        adapter,
        device,
        queue,
    }
}

async fn request_adapter(wgpu_instance: &wgpu::Instance, compatible_surface: Option<&wgpu::Surface<'_>>) -> wgpu::Adapter {
    wgpu_instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface,
            force_fallback_adapter: false,
        })
        .await
        .expect("no adapter")
}

async fn request_device(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
    adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: adapter.limits(),
                label: None,
            },
            None,
        )
        .await
        .unwrap_or_report()
}

struct Resource {
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    bind_group_layout: wgpu::BindGroupLayout,
}

fn create_shader_context(device: &wgpu::Device, simulation_parameters: &Parameters) -> Resource {
    let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Shader Context Buffer"),
        contents: bytemuck::cast_slice(&[simulation_parameters.shader_parameters]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Shader Context Bind Group Layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::COMPUTE | wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: wgpu::BufferSize::new(std::mem::size_of::<ShaderParameters>() as u64),
            },
            count: None,
        }],
    });
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Shader Context Bind Group"),
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        }],
    });
    Resource {
        buffer,
        bind_group,
        bind_group_layout,
    }
}

fn initial_agent_distribution(simulation_parameters: &Parameters) -> Vec<Agent> {
    (0..simulation_parameters.number_of_agents)
        .map(|_| Agent::new_with_random_start_position(simulation_parameters))
        .collect()
}

fn reset_simulation(state: &State, simulation_parameters: &Parameters) {
    // Reset agents
    tracing::info!("Redistributing agents");
    let agents = initial_agent_distribution(simulation_parameters);
    tracing::info!("Writing agent buffer to GPU");
    state
        .gpu
        .queue
        .write_buffer(&state.resources.agents.buffer, 0, bytemuck::cast_slice(&agents));

    // Reset chemo-attractants
    tracing::info!("Writing chemo_attractants buffer to GPU");
    let canvas_resolution = simulation_parameters.shader_parameters.canvas_width * simulation_parameters.shader_parameters.canvas_height;
    let black_canvas: Vec<f32> = vec![0.0; canvas_resolution as usize];
    state
        .gpu
        .queue
        .write_buffer(&state.resources.deposit_layer.buffer, 0, bytemuck::cast_slice(&black_canvas));
}

fn create_agents(device: &wgpu::Device, simulation_parameters: &Parameters) -> Resource {
    let agents = initial_agent_distribution(simulation_parameters);

    let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Agent Buffer"),
        contents: bytemuck::cast_slice(&agents),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Agent Bind Group Layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: wgpu::BufferSize::new(u64::from(simulation_parameters.number_of_agents) * std::mem::size_of::<Agent>() as u64),
            },
            count: None,
        }],
    });
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Agent Bind Group"),
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        }],
    });

    Resource {
        buffer,
        bind_group,
        bind_group_layout,
    }
}

fn create_deposit_layer(device: &wgpu::Device, simulation_parameters: &Parameters) -> Resource {
    let canvas_resolution = simulation_parameters.shader_parameters.canvas_width * simulation_parameters.shader_parameters.canvas_height;

    // Start with a black canvas
    let init: Vec<f32> = vec![0.0; canvas_resolution as usize];

    let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Deposit Layer Buffer"),
        contents: bytemuck::cast_slice(&init),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Deposit Layer Bind Group Layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::COMPUTE | wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: wgpu::BufferSize::new((usize::try_from(canvas_resolution).unwrap() * std::mem::size_of::<f32>()) as u64),
            },
            count: None,
        }],
    });
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Deposit Layer Bind Group"),
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        }],
    });

    Resource {
        buffer,
        bind_group,
        bind_group_layout,
    }
}

struct Resources {
    shader_context: Resource,
    agents: Resource,
    deposit_layer: Resource,
}

fn create_resources(device: &wgpu::Device, simulation_parameters: &Parameters) -> Resources {
    let shader_context = create_shader_context(device, simulation_parameters);
    let agents = create_agents(device, simulation_parameters);
    let deposit_layer = create_deposit_layer(device, simulation_parameters);
    Resources {
        shader_context,
        agents,
        deposit_layer,
    }
}

struct Pipelines {
    agent_sense_move_deposit: wgpu::ComputePipeline,
    diffuse_and_decay: wgpu::ComputePipeline,
    render_pipeline: wgpu::RenderPipeline,
}

fn create_pipelines(device: &wgpu::Device, surface_format: wgpu::TextureFormat, resources: &Resources) -> Pipelines {
    let shader_source = wgpu::ShaderSource::Wgsl(include_str!("../shaders.wgsl").into());
    let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader Module"),
        source: shader_source,
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Pipeline layout"),
        bind_group_layouts: &[
            &resources.shader_context.bind_group_layout,
            &resources.agents.bind_group_layout,
            &resources.deposit_layer.bind_group_layout,
        ],
        push_constant_ranges: &[],
    });

    let agent_sense_move_deposit = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Agent Compute Pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader_module,
        entry_point: "agent_sense_move_deposit",
    });

    let diffuse_and_decay = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Diffuse and Decay Compute Pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader_module,
        entry_point: "diffuse_and_decay",
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader_module,
            entry_point: "vertex_main",
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader_module,
            entry_point: "fragment_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: surface_format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            ..Default::default()
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    });

    Pipelines {
        agent_sense_move_deposit,
        diffuse_and_decay,
        render_pipeline,
    }
}

fn surface_texture_format() -> wgpu::TextureFormat {
    wgpu::TextureFormat::Bgra8UnormSrgb
}

struct WindowSurface<'w> {
    window: Arc<winit::window::Window>,
    surface: wgpu::Surface<'w>,

    #[allow(dead_code)]
    texture_format: wgpu::TextureFormat,
}

fn create_compute_pass<'encoder, 'pipeline, 'resources>(
    command_encoder: &'encoder mut wgpu::CommandEncoder,
    pipeline: &'pipeline wgpu::ComputePipeline,
    resources: &'resources Resources,
    label: &str,
) -> wgpu::ComputePass<'encoder>
where
    'pipeline: 'encoder,
    'resources: 'encoder,
{
    let mut compute_pass = command_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
        label: Some(label),
        timestamp_writes: None,
    });
    compute_pass.set_pipeline(pipeline);
    compute_pass.set_bind_group(0, &resources.shader_context.bind_group, &[]);
    compute_pass.set_bind_group(1, &resources.agents.bind_group, &[]);
    compute_pass.set_bind_group(2, &resources.deposit_layer.bind_group, &[]);
    compute_pass
}

fn create_render_pass<'encoder, 'texture_view, 'resources, 'pipelines>(
    command_encoder: &'encoder mut wgpu::CommandEncoder,
    texture_view: &'texture_view wgpu::TextureView,
    resources: &'resources Resources,
    pipelines: &'pipelines Pipelines,
) -> wgpu::RenderPass<'encoder>
where
    'texture_view: 'encoder,
    'resources: 'encoder,
    'pipelines: 'encoder,
{
    let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: texture_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::default()),
                store: wgpu::StoreOp::Store,
            },
        })],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
    });

    render_pass.set_pipeline(&pipelines.render_pipeline);
    render_pass.set_bind_group(0, &resources.shader_context.bind_group, &[]);
    render_pass.set_bind_group(1, &resources.agents.bind_group, &[]);
    render_pass.set_bind_group(2, &resources.deposit_layer.bind_group, &[]);
    render_pass
}

#[allow(dead_code)]
fn average_fps(duration: Duration, frames_rendered: u64) -> f64 {
    let seconds = duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9;
    frames_rendered as f64 / seconds
}

/// Execute one tick of the simulation
fn tick(state: &State, simulation_parameters: &Parameters) {
    tracing::debug!("SIMULATION TICK, acquiring mutex...");
    let gpu_lock = state.gpu_mutex.lock().unwrap();
    tracing::debug!("SIMULATION TICK, acquired mutex...");

    // Start a new command encoder
    tracing::debug!("Creating compute pass command encoder");
    let mut command_encoder = state.gpu.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Compute passes Command Encoder"),
    });

    // Diffuse and Decay
    {
        let mut compute_pass = create_compute_pass(
            &mut command_encoder,
            &state.pipelines.diffuse_and_decay,
            &state.resources,
            "Diffuse and Decay Compute Pass",
        );
        compute_pass.dispatch_workgroups(
            simulation_parameters.shader_parameters.canvas_width / 8,
            simulation_parameters.shader_parameters.canvas_height / 8,
            1,
        );
    }

    // Move agents and deposit
    {
        let mut compute_pass = create_compute_pass(
            &mut command_encoder,
            &state.pipelines.agent_sense_move_deposit,
            &state.resources,
            "Agent Compute Pass",
        );

        // Lay agents out in x and y so they can be mapped to shader workgroups

        let number_of_active_agents = simulation_parameters.shader_parameters.number_of_active_agents;

        // Must match what is in the shader code
        const WORKGROUP_SIZE_X: u32 = 8;
        const WORKGROUP_SIZE_Y: u32 = 8;
        const WORKGROUP_SIZE_Z: u32 = 1;

        let threads_per_workgroup = WORKGROUP_SIZE_X * WORKGROUP_SIZE_Y * WORKGROUP_SIZE_Z;

        // Hacked integer division: number_of_active_agents / threads_per_workgroup but hacked so it rounds up
        let workgroups_needed = (number_of_active_agents + (threads_per_workgroup - 1)) / threads_per_workgroup;

        const NUMBER_OF_WORKGROUPS_X: u32 = 32;
        // Hacked integer division: workgroups_needed / 32 but hacked so it rounds up
        let number_of_workgroups_y = (workgroups_needed + 31) / 32;
        let number_of_workgroups_z = 1;

        compute_pass.dispatch_workgroups(NUMBER_OF_WORKGROUPS_X, number_of_workgroups_y, number_of_workgroups_z);
    }

    tracing::debug!("Finishing command encoder...");
    let command_buffer = command_encoder.finish();
    tracing::debug!("Submitting...");
    state.gpu.queue.submit(Some(command_buffer));

    tracing::debug!("SIMULATION TICK, dropping mutex...");
    drop(gpu_lock);
    tracing::debug!("SIMULATION TICK, dropped mutex...");

    tracing::debug!("Submitted");
}

struct State {
    gpu: Gpu,
    resources: Resources,
    pipelines: Pipelines,
    gpu_mutex: Arc<std::sync::Mutex<()>>,
}

fn sync_shader_parameters(state: &State, simulation_parameters: &Parameters) {
    state.gpu.queue.write_buffer(
        &state.resources.shader_context.buffer,
        0,
        bytemuck::cast_slice(&[simulation_parameters.shader_parameters]),
    );
}

fn send_command(user_input_tx: &std::sync::mpsc::Sender<common::Command>, command: common::Command) {
    if let Err(err) = user_input_tx.send(command) {
        tracing::error!("Failed to send user input: {err}");
    }
}

fn run_event_loop(event_loop: winit::event_loop::EventLoop<()>, event_loop_context: EventLoopContext) {
    let mut window_surface = None::<WindowSurface<'_>>;
    event_loop
        .run(|event, active_event_loop| handle_event(event, active_event_loop, &mut window_surface, &event_loop_context))
        .unwrap_or_report();
}

fn handle_event(event: Event<()>, active_event_loop: &ActiveEventLoop, window_surface: &mut Option<WindowSurface<'_>>, ctx: &EventLoopContext) {
    let simulation_parameters = { *ctx.simulation_parameters_rx.borrow() };

    tracing::debug!("Event: {event:?}");
    if matches!(event, Event::NewEvents(StartCause::Init)) {
        let window = create_winit_window(
            active_event_loop,
            CreateWindowConfiguration {
                width: simulation_parameters.shader_parameters.canvas_width,
                height: simulation_parameters.shader_parameters.canvas_height,
            },
        );

        let s: WindowSurface<'_> = ctx.tokio_runtime.block_on(create_window_surface(&ctx.state.gpu, window));
        let _ = window_surface.insert(s);
        return;
    }

    let window_surface = window_surface.as_ref().unwrap();

    match event {
        Event::AboutToWait => {}
        Event::NewEvents(start_cause) => match start_cause {
            StartCause::ResumeTimeReached { .. } => {
                tracing::trace!("RESUME. REQUEST REDRAW");
                window_surface.window.request_redraw();
            }
            StartCause::WaitCancelled { .. } => {}
            StartCause::Poll => {}
            StartCause::Init => unreachable!(),
        },
        Event::UserEvent(()) => {}
        Event::WindowEvent { event, window_id: _ } => match event {
            WindowEvent::PinchGesture { .. } => {}
            WindowEvent::RotationGesture { .. } => {}
            WindowEvent::DoubleTapGesture { .. } => {}
            WindowEvent::ActivationTokenDone { .. } => {}
            WindowEvent::Resized(_) => {}
            WindowEvent::Moved(_) => {}
            WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                active_event_loop.exit();
            }
            WindowEvent::DroppedFile(_) => {}
            WindowEvent::HoveredFile(_) => {}
            WindowEvent::HoveredFileCancelled => {}
            WindowEvent::Focused(focused) => {
                tracing::info!("Focused: {focused}");
            }
            WindowEvent::KeyboardInput { event, .. } => match event {
                KeyEvent {
                    logical_key,
                    physical_key: _,
                    text: _,
                    location: _,
                    state: key_state,
                    repeat: _,
                    ..
                } => match logical_key {
                    Key::Character(c) if c.as_ref() == "q" && matches!(key_state, ElementState::Released) => {
                        active_event_loop.exit();
                    }
                    Key::Named(NamedKey::Space) if matches!(key_state, ElementState::Released) => {
                        send_command(&ctx.user_input_tx, common::Command::TogglePlayPause);
                    }
                    Key::Named(NamedKey::Backspace) if matches!(key_state, ElementState::Released) => {
                        send_command(&ctx.user_input_tx, common::Command::ResetSimulation);
                    }
                    Key::Character(c) if c.as_ref() == "r" && matches!(key_state, ElementState::Released) => {
                        send_command(&ctx.user_input_tx, common::Command::RandomizeParameters);
                    }
                    Key::Character(c) if c.as_ref() == "1" && matches!(key_state, ElementState::Released) => {
                        send_command(&ctx.user_input_tx, common::Command::ActivatePreset(1));
                    }
                    Key::Character(c) if c.as_ref() == "2" && matches!(key_state, ElementState::Released) => {
                        send_command(&ctx.user_input_tx, common::Command::ActivatePreset(2));
                    }
                    Key::Character(c) if c.as_ref() == "3" && matches!(key_state, ElementState::Released) => {
                        send_command(&ctx.user_input_tx, common::Command::ActivatePreset(3));
                    }
                    Key::Character(c) if c.as_ref() == "4" && matches!(key_state, ElementState::Released) => {
                        send_command(&ctx.user_input_tx, common::Command::ActivatePreset(4));
                    }
                    Key::Character(c) if c.as_ref() == "5" && matches!(key_state, ElementState::Released) => {
                        send_command(&ctx.user_input_tx, common::Command::ActivatePreset(5));
                    }
                    _ => {}
                },
            },
            WindowEvent::ModifiersChanged(_) => {}
            WindowEvent::Ime(_) => {}
            WindowEvent::CursorMoved { .. } => {}
            WindowEvent::CursorEntered { .. } => {}
            WindowEvent::CursorLeft { .. } => {}
            WindowEvent::MouseWheel { .. } => {}
            WindowEvent::MouseInput { .. } => {}
            WindowEvent::TouchpadPressure { .. } => {}
            WindowEvent::AxisMotion { .. } => {}
            WindowEvent::Touch(_) => {}
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                tracing::info!("Scale factor changed: {scale_factor:?}");
            }
            WindowEvent::ThemeChanged(_) => {}
            WindowEvent::Occluded(_) => {}
            WindowEvent::RedrawRequested => {
                let redraw_requested_at = Instant::now();

                // Allocate about 16.6ms (roughly 60FPS) per frame
                let time_per_frame = Duration::from_micros(16666);
                let next_frame = redraw_requested_at + time_per_frame;
                active_event_loop.set_control_flow(winit::event_loop::ControlFlow::WaitUntil(next_frame));

                tracing::trace!("REDRAW");

                // RENDER PASS

                let surface_texture = window_surface.surface.get_current_texture().expect("Failed to get current texture");
                let texture_view: wgpu::TextureView = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());

                tracing::debug!("RENDER PASS, acquiring mutex...");
                let gpu_lock = ctx.state.gpu_mutex.lock().unwrap();
                tracing::debug!("RENDER PASS, acquired mutex...");

                tracing::debug!("Creating render pass command encoder");
                let mut command_encoder = ctx.state.gpu.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render pass Command Encoder"),
                });
                let mut render_pass = create_render_pass(&mut command_encoder, &texture_view, &ctx.state.resources, &ctx.state.pipelines);

                let param: Parameters = { *ctx.simulation_parameters_rx.borrow() };
                let vertices = match param.number_of_triangles {
                    common::simulation::NumberOfTriangles::Zero => 0..0,
                    common::simulation::NumberOfTriangles::One => 0..3,
                    // Default: full-screen quad made of two triangles
                    common::simulation::NumberOfTriangles::Two => 0..6,
                };
                let instances = 0..1;
                render_pass.draw(vertices, instances);
                drop(render_pass); // End the render pass

                ctx.state.gpu.queue.submit(std::iter::once(command_encoder.finish()));

                tracing::debug!("RENDER PASS, dropping mutex...");
                drop(gpu_lock);
                tracing::debug!("RENDER PASS, dropped mutex...");

                tracing::debug!("Presenting...");
                surface_texture.present();
                tracing::debug!("Presented");
            }
        },

        Event::DeviceEvent { device_id: _, event } => match event {
            DeviceEvent::Added => {}
            DeviceEvent::Removed => {}
            DeviceEvent::MouseMotion { .. } => {}
            DeviceEvent::MouseWheel { .. } => {}
            DeviceEvent::Motion { .. } => {}
            DeviceEvent::Button { .. } => {}
            DeviceEvent::Key(_) => {}
        },

        // Unhandled events
        ev => {
            tracing::warn!("Unhandled event: {ev:?}");
        }
    }
}

struct EventLoopContext {
    state: Arc<State>,
    simulation_parameters_rx: tokio::sync::watch::Receiver<Parameters>,
    tokio_runtime: tokio::runtime::Runtime,
    user_input_tx: std::sync::mpsc::Sender<common::Command>,
}
