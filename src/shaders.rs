//! Generic shaders that are used often


///simple vertex shader. uniform requires `model: Mat4`,
///`view: Mat4` (`Mat4::view_matrix_3d()` or `Mat4::view_matrix_2d()`)
///and `camera: Mat4` (inverse camera matrix. use `Mat4::from_inverse_transform()` or call
///`inverse()` before feeding it to the shader)
pub const VERTEX: &str = 
"#version 140
in vec3 position;
in vec2 texture_coords;
in vec3 normal;
in vec4 colour;

out vec3 v_position;
out vec2 uv;
out vec3 v_normal;
out vec4 v_colour;

uniform mat4 model;
uniform mat4 view;
uniform mat4 camera;
void main() {
    v_colour = colour;
    uv = texture_coords;
    mat3 norm_mat = transpose(inverse(mat3(view * model)));
    v_normal = normalize(norm_mat * normal);
    gl_Position = view * camera * model * vec4(position, 1.0);
    v_position = gl_Position.xyz / gl_Position.w;
}";
