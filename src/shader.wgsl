// Vertex shader



// uniform buffer
struct ScreenUniform {
    bool_array: array<vec4<u32>, 512>, // 2048 / 4 = 512
};
@group(0) @binding(0)
var<uniform> screen_data: ScreenUniform;

fn map_to_clip_space(pos: vec2<u32>) -> vec2<f32> {
    // Map coordinates from screen space (0 to screen_size) to clip space (-1 to 1)
    let screen_width: f32 = 64.0;
    let screen_height: f32 = 32.0;
    let clip_space_x:f32 = (f32(pos.x) / screen_width) * 2.0 - 1.0;
    let clip_space_y:f32 = (f32(pos.y) / screen_height) * 2.0 - 1.0;

    return vec2<f32>(clip_space_x, clip_space_y);
}

fn get_cordinates_from_index(in_vertex_index: u32) -> vec2<u32> {
    let screen_width: u32 = 64u;
    let screen_height: u32 = 32u;


    let x:u32 = in_vertex_index % screen_width;
    let y:u32 = screen_height - 1 - ((in_vertex_index - x) / screen_width);
    return vec2<u32>(x, y);
}

fn get_relative_vertex_cordinates(in_vertex_index: u32) -> vec2<u32> {
    switch in_vertex_index {
        case 0u: {
            return vec2<u32>(0, 0);
        }
        case 1u: {
            return vec2<u32>(1, 0);
        }
        case 2u: {
            return vec2<u32>(0, 1);
        }
        case 3u: {
            return vec2<u32>(0, 1);
        }
        case 4u: {
            return vec2<u32>(1, 0);
        }
        case 5u: {
            return vec2<u32>(1, 1);
        }
        default: {
            return vec2<u32>(0, 0);
        }
    }
}

fn get_array_index(real_index: u32) -> vec2<u32> {
    let x:u32 = real_index % 4;
    let y:u32 = (real_index - x) / 4;
    return vec2<u32>(x, y);
}



struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32
) -> VertexOutput {
    var vert_num: u32 = in_vertex_index % 6; // 6 vertices per quad get in which vertex we are -> assume in_vertex_index starts with zero


    //calculate the index of the quad
    let real_index: u32 = (in_vertex_index - vert_num) / 6; 

    let pos: vec2<u32> = get_cordinates_from_index(real_index);
    let vert_pos: vec2<u32> = pos + get_relative_vertex_cordinates(vert_num);

    var out: VertexOutput;
    let arraypos = get_array_index(real_index);
    if screen_data.bool_array[arraypos.y][arraypos.x] == 1 { // later change this when changing the size of the screen
        out.color = vec3<f32>(1.0, 0.0, 0.0);
    }
    else {
        out.color = vec3<f32>(0.0, 1.0, 0.0);
    }
    out.clip_position = vec4<f32>(map_to_clip_space(vert_pos),1.0, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
