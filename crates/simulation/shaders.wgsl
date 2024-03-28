struct ShaderParameters {
    agent_speed: f32,
    bool_enable_agent_bounce: u32,
    bool_enable_agent_deposit: u32,
    bool_enable_agent_rotate: u32,
    bool_enable_agent_rotate_left: u32,
    bool_enable_agent_rotate_randomly: u32,
    bool_enable_agent_rotate_right: u32,
    bool_enable_color: u32,
    bool_enable_decay: u32,
    bool_enable_diffuse: u32,
    bool_enable_render_trail_map: u32,
    canvas_width: u32,
    canvas_height: u32,
    number_of_active_agents: u32,
    vertex_stretch: f32,
    decay_strength: f32,
    sensor_angle_degrees: f32,
    max_turn_angle_degrees: f32,
    max_rand_turn_angle_degrees: f32,
    high_density_threshold: f32,
    high_density_speed_boost: f32,
    deposit_strength: f32,
    sensor_distance: f32,
}

struct Agent {
    position: vec2<f32>,
    velocity: vec2<f32>,
}

struct TrailMap {
    data: array<f32>,
}

@group(0) @binding(0)
var<uniform> ctx: ShaderParameters;
@group(1) @binding(0)
var<storage, read_write> agents_buffer: array<Agent>;
@group(2) @binding(0)
var<storage, read_write> trail_map: TrailMap;

@vertex
fn vertex_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
    var position: vec2<f32>;
    switch (vertex_index) {
        // Bottom left triangle
        case 0u: { position = lerp_vec2(vec2<f32>(-0.8, -0.4), vec2<f32>(-1.0, -1.0), ctx.vertex_stretch); }
        case 1u: { position = lerp_vec2(vec2<f32>(0.3, -0.05), vec2<f32>(1.0, -1.0), ctx.vertex_stretch); }
        case 2u: { position = lerp_vec2(vec2<f32>(-0.35, 0.2), vec2<f32>(-1.0, 1.0), ctx.vertex_stretch); }

        // Upper right triangle
        case 3u: { position = lerp_vec2(vec2<f32>(0.9, 0.30), vec2<f32>(1.0, 1.0), ctx.vertex_stretch); }
        case 4u: { position = lerp_vec2(vec2<f32>(0.5, -0.1), vec2<f32>(1.0, -1.0), ctx.vertex_stretch); }
        case 5u: { position = lerp_vec2(vec2<f32>(0.2, 0.74), vec2<f32>(-1.0, 1.0), ctx.vertex_stretch); }

        default: { position = vec2<f32>(0.0, 0.0); } // This should never happen 
    }

    let clip_position = vec4<f32>(position, 0.0, 1.0);
    return clip_position;
}

@fragment 
fn fragment_main(@builtin(position) coord: vec4<f32>) -> @location(0) vec4<f32> {
    let x = u32(coord.x);
    let y = u32(coord.y);

    if !bool(ctx.bool_enable_render_trail_map) {
        // Output a solid red color
        return vec4(1.0, 0.0, 0.0, 1.0);
    }

    // Sample trail map:
    let v: f32 = trail_map.data[y * ctx.canvas_width + x];

    if v > 1.0 {
        // Not supposed to happen
        return vec4(1.0, 0.0, 0.0, 1.0);
    }

    if bool(ctx.bool_enable_color) {
        return vec4(gradient(v), 1.0);
    } else {
        return vec4(v, v, v, 1.0);
    }
}

// Workgroup size stuff: https://webgpufundamentals.org/webgpu/lessons/webgpu-compute-shaders.html
// The workgroup size dictates how many threads in each dimension that execute the compute shader in parallel
// 8,8,1 is 8x8x1 = 64 threads per workgroup.
// So for 500_000 agents, we would need 500_000 / 64 workgroups.
@compute @workgroup_size(8,8,1)
fn agent_sense_move_deposit(
    @builtin(workgroup_id) workgroup_id : vec3<u32>, // Of all the dispatched workgroups (CPU side) what is the ID of this one
    @builtin(num_workgroups) num_workgroups: vec3<u32>, // What are the dimensions of the dispatched workgroups. (These are like the upper bounds on values in workgroup_id)
    @builtin(local_invocation_index) local_invocation_index: u32, // The thread ID within the workgroup. All the threads in the workgroup have the same workgroup ID
) {
    // Threads per workgroup is the product of the workgroup_size
    let num_threads_per_workgroup: u32 = 8u * 8u * 1u;

    // Reduce 3-dimensional workgroup id to 1-dimension workgroup index. Sequential and unique within the workgroup
    let workgroup_index =
        workgroup_id.x +
        workgroup_id.y * num_workgroups.x +
        workgroup_id.z * num_workgroups.x * num_workgroups.y;

    // Unique ID for _this_ thread on _this_ workgroup - globally unique and sequential from zero, so useable as a global index.
    let global_invocation_index = workgroup_index * num_threads_per_workgroup + local_invocation_index;

    let agent_idx = global_invocation_index;

    // Handle overflow (e.g. 500_000 agents are not divisible by the number of threads actually created on the gpu)
    if agent_idx >= ctx.number_of_active_agents {
        return;
    }

    // Fetch the agent data
    var agent = agents_buffer[agent_idx];

    // SENSE: Get deposit values at the sensors
    let sensor_ahead_val: f32 = sense(agent, 0.0);
    let sensor_ccw_val: f32 = sense(agent, ctx.sensor_angle_degrees);
    let sensor_cw_val: f32 = sense(agent, -ctx.sensor_angle_degrees);

    // let rand_turn_strength = random_float_in_range(-1.0, 1.0, seed_from_agent(agent));

    // Seed for randomness
    let seed = seed_from_agent(agent);

    // ROTATE: Update agent direction (rotate its velocity)
    if bool(ctx.bool_enable_agent_rotate) {
        if bool(ctx.bool_enable_agent_rotate_left) &&sensor_ccw_val > sensor_ahead_val && sensor_ahead_val > sensor_cw_val {
            // Turn counter-clockwise
            // agent.velocity = rotate_ccw(abs(rand_turn_strength) * ctx.max_turn_angle_degrees, agent.velocity);
            agent.velocity = rotate_ccw(ctx.max_turn_angle_degrees, agent.velocity);
        }
        if bool(ctx.bool_enable_agent_rotate_right) && sensor_cw_val > sensor_ahead_val && sensor_ahead_val > sensor_ccw_val {
            // Turn clockwise
            // agent.velocity = rotate_cw(abs(rand_turn_strength) * ctx.max_turn_angle_degrees, agent.velocity);
            agent.velocity = rotate_cw(ctx.max_turn_angle_degrees, agent.velocity);
        }
        if bool(ctx.bool_enable_agent_rotate_right) && sensor_cw_val > sensor_ahead_val && sensor_ccw_val > sensor_ahead_val {
            // Rotate randomly left or right by RA.

            let ra = rand_sign(seed) * ctx.max_turn_angle_degrees;
            agent.velocity = rotate_cw(ra, agent.velocity);

            // agent.velocity = rotate_cw(rand_turn_strength * ctx.max_rand_turn_angle_degrees, agent.velocity);
        }
    }

    var speed = ctx.agent_speed;

    // TWIST: If the deposit density is too great (too many agents in the same spot)
    // TODO: create toggle
    if deposit_strength_at(agent.position) >= ctx.high_density_threshold {
        // Add speed proportional to the speed boost param and the deposit strength at the current position
        speed = speed + ctx.high_density_speed_boost * deposit_strength_at(agent.position);

        // Turn agent randomly
        agent.velocity = rotate_cw(random_float_in_range(-ctx.max_rand_turn_angle_degrees, ctx.max_rand_turn_angle_degrees, seed), agent.velocity);
    }

    // BOUNCE: Bounce on window borders
    if bool(ctx.bool_enable_agent_bounce) {
        // Bounce on left edge
        if (agent.position.x < 0.0) {
            agent.velocity.x = abs(agent.velocity.x);
        }
        // Bounce on right edge
        if (agent.position.x > f32(ctx.canvas_width)) {
            agent.velocity.x = -abs(agent.velocity.x);
        }
        // Bounce on top edge
        if (agent.position.y < 0.0) {
            agent.velocity.y = abs(agent.velocity.y);
        }
        // Bounce on bottom edge
        if (agent.position.y > f32(ctx.canvas_height)) {
            agent.velocity.y = -abs(agent.velocity.y);
        }
    }

    // MOVE: Move agent forward
    agent.position += agent.velocity * speed;

    // Update agent data in the buffer
    agents_buffer[agent_idx] = agent;

    // Calculate pixel position to color
    let pos: vec2<f32> = agent.position;

    if bool(ctx.bool_enable_agent_deposit) && !is_out_of_bounds(pos) {
        let pixel_x = u32(round(pos.x));
        let pixel_y = u32(round(pos.y));
        let pixel_idx = pixel_y * ctx.canvas_width + pixel_x;

        // DEPOSIT
        trail_map.data[pixel_idx] = clamp(trail_map.data[pixel_idx] + ctx.deposit_strength, 0.0, 1.0);
    }
}

@compute @workgroup_size(8,8,1)
fn diffuse_and_decay(
    @builtin(workgroup_id) workgroup_id : vec3<u32>,
    @builtin(num_workgroups) num_workgroups: vec3<u32>,
    @builtin(local_invocation_index) local_invocation_index: u32,
) {
    let num_threads_per_workgroup: u32 = 8u * 8u * 1u;
    let workgroup_index =
        workgroup_id.x +
        workgroup_id.y * num_workgroups.x +
        workgroup_id.z * num_workgroups.x * num_workgroups.y;
    let global_invocation_index = workgroup_index * num_threads_per_workgroup + local_invocation_index;

    let idx = global_invocation_index;
    if idx >= ctx.canvas_width * ctx.canvas_height {
        return;
    }

    let x = idx % ctx.canvas_width;
    let y = idx / ctx.canvas_width;

    // DIFFUSE
    if bool(ctx.bool_enable_diffuse) {
        var sum: f32 = 0.0;
        for (var i = -1; i <= 1; i = i + 1) {
            for (var j = -1; j <= 1; j = j + 1) {
                // Option 1: To handle the borders, skip pixels outside the canvas
                let xi = i32(x) + i;
                let yi = i32(y) + j;
                if (xi < 0 || xi > i32(ctx.canvas_width) - 1 || yi < 0 || yi > i32(ctx.canvas_height) - 1) {
                    continue;
                }

                // Option 2: To handle the borders, replicate the edge pixel values, meaning pixels outside the image have the value of the nearest edge pixel.
                // let xi = clamp(i32(x) + i, 0, i32(ctx.canvas_width) - 1);
                // let yi = clamp(i32(y) + j, 0, i32(ctx.canvas_height) - 1);

                let xi_u = u32(xi);
                let yi_u = u32(yi);

                sum = sum + trail_map.data[yi_u * ctx.canvas_width + xi_u];
            }
        }
        trail_map.data[idx] = sum / 9.0;
    }

    // DECAY
    // Ratio that survives into the next iteration
    if bool(ctx.bool_enable_decay) {
        trail_map.data[idx] = trail_map.data[idx] * (1.0 - ctx.decay_strength);
    }
}

// Rotate clockwise, assuming a screen space coordinate system,
// ┌───➤ x
// │
// ▼
// y
fn rotate_cw(degrees: f32, v: vec2<f32>) -> vec2<f32> {
    let a = radians(degrees);
    return vec2(
        v.x * cos(a) - v.y * sin(a),
        v.x * sin(a) + v.y * cos(a)
    );
}

// Rotate counter-clockwise, assuming a screen space coordinate system,
// ┌───➤ x
// │
// ▼
// y
fn rotate_ccw(degrees: f32, v: vec2<f32>) -> vec2<f32> {
    return rotate_cw(-degrees, v);
}

fn sense(agent: Agent, sensor_angle_ccw_degrees: f32) -> f32 {
    let facing_direction = normalize(agent.velocity);
    let rotated_facing_direction: vec2<f32> = rotate_ccw(sensor_angle_ccw_degrees, facing_direction);
    // Sensor position:
    let sensor: vec2<f32> = agent.position + rotated_facing_direction * ctx.sensor_distance;
    return deposit_strength_at(sensor);
}

fn deposit_strength_at(pos: vec2<f32>) -> f32 {
    if is_out_of_bounds(pos) {
        return 0.0;
    }

    let pos_x: u32 = u32(round(pos.x));
    let pos_y: u32 = u32(round(pos.y));
    let pos_idx: u32 = pos_y * ctx.canvas_width + pos_x;
    return trail_map.data[pos_idx];
}

fn rand_sign(seed: f32) -> f32 {
    return sign(random_float_in_range(-1.0, 1.0, seed));
}

fn random_float_in_range(min: f32, max: f32, seed: f32) -> f32 {
    let seed_u32 = bitcast<u32>(seed);

    // Convert the u32 to a float in the range [0.0, 1.0)
    let random_float = f32(rand_u32(seed_u32)) / f32(0xFFFFFFFFu);

    // Scale and shift to the desired range
    return min + random_float * (max - min);
}

fn seed_from_agent(agent: Agent) -> f32 {
    let seed = agent.position.x + agent.position.y + abs(agent.velocity.x);
    return seed;
}

fn rand_u32(seed: u32) -> u32 {
    var h = seed * 747796405u + 2891336453u;
    h = ((h >> ((h >> 28u) + 4u)) ^ h) * 277803737u;
    return (h >> 22u) ^ h;
}

fn gradient(t: f32) -> vec3<f32> {
    let black = vec3(0.0, 0.0, 0.0);
    let red = vec3(1.0, 0.0, 0.0);
    let green = vec3(0.0, 1.0, 0.0);
    let blue = vec3(0.0, 0.0, 1.0);
    let purple = vec3(0.5, 0.0, 0.5);
    let white = vec3(1.0, 1.0, 1.0);

    if t > 1.0 {
        return purple;
    }

    let g1 = vec3(40.0, 54.0, 24.0) / vec3(255.0);
    let g2 = vec3(96.0, 108.0, 56.0) / vec3(255.0);
    let g3 = vec3(188.0, 108.0, 37.0) / vec3(255.0);
    let g4 = vec3(221.0, 161.0, 94.0) / vec3(255.0);
    let g5 = vec3(254.0, 250.0, 224.0) / vec3(255.0);

    let idx: f32 = floor(t * 5.0);
    let segment_length: f32 = 1.0 / 5.0;

    var c1 = black;
    var c2 = black;
    if t <= 0.2 {
        c1 = black;
        c2 = g1;
    }
    else if t <= 0.4 {
        c1 = g1;
        c2 = g2;
    }
    else if t <= 0.6 {
        c1 = g2;
        c2 = g3;
    }
    else if t <= 0.8 {
        c1 = g3;
        c2 = g4;
    }
    else if t <= 1.0 {
        c1 = g4;
        c2 = g5;
    }

    if idx >= 5.0 {
        return c2;
    }

    let t_segment: f32 = (t / segment_length) - (idx * segment_length / segment_length);
    return lerp_vec3(c1, c2, t_segment);

    // if (t < 0.5) {
    //     // Map t from [0.0, 0.5] to [0.0, 1.0]
    //     let t_segment = t * 2.0;
    //     return lerp(black, green, t_segment);
    // } else {
    //     // Map t from [0.5, 1.0] to [0.0, 1.0]
    //     let t_segment = (t - 0.5) * 2.0;
    //     return lerp(green, white, t_segment);
    // }

    // return lerp(black, white, t);
}

fn lerp_vec2(a: vec2<f32>, b: vec2<f32>, t: f32) -> vec2<f32> {
    return a + t * (b - a);
}

fn lerp_vec3(a: vec3<f32>, b: vec3<f32>, t: f32) -> vec3<f32> {
    return a + t * (b - a);
}

fn is_out_of_bounds(pos: vec2<f32>) -> bool {
    if (pos.x < 0.0 || pos.x > f32(ctx.canvas_width - 1u) || pos.y < 0.0 || pos.y > f32(ctx.canvas_height - 1u)) {
        return true;
    }
    return false;
}
