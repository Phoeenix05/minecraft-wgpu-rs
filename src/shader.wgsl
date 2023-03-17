struct CameraUniform {
  view_proj: mat4x4<f32>,
}
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
  @location(0) position: vec3<f32>,
  @location(1) color: vec3<f32>,
}

struct VertexOuput {
  @builtin(position) clip_position: vec4<f32>,
  @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(
  model: VertexInput,
) -> VertexOuput {
  var out: VertexOuput;
  // let x = f32(1 - i32(in_vertex_index)) * 0.5;
  // let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
  out.color = model.color;
  out.clip_position = vec4<f32>(model.position, 1.0);
  // out.vert_pos = out.clip_position.xyz;
  return out;
}

@fragment
fn fs_main(in: VertexOuput) -> @location(0) vec4<f32> {
  return vec4<f32>(in.color, 1.0);
}