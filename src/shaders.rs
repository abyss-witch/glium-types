//! Shader examples to help get you started


/// simple vertex shader. uniform requires `model: Mat4`,
/// `perspective: Mat4` (use `Mat4::view_matrix_3d()` for 3d or `Mat4::view_matrix_2d()` for
/// 2d/orthographic) and `camera: Mat4` (inverse camera matrix. use `Mat4::from_inverse_transform()`
/// or call `.inverse()` before feeding the matrix into to the shader)
///
/// If you are new to shaders or how things render I highly recomend you check out the glium tutorial
/// book (https://github.com/glium/glium/tree/master/book) it is where this shader is from after
/// all
///
/// WARNING! if ALL vertex bindings aren't bound in the draw call your program will PANIC on most
/// computers, HOWEVER, some don't panic, which may lead you to FALSELY believe you program will run
/// else where
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
uniform mat4 perspective;
uniform mat4 camera;
void main() {
    v_colour = colour;
    uv = texture_coords;
    mat3 norm_mat = transpose(inverse(mat3(camera * model)));
    v_normal = normalize(norm_mat * normal);
    gl_Position = perspective * camera * model * vec4(position, 1.0);
    v_position = gl_Position.xyz / gl_Position.w;
}";
