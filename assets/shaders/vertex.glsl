#version 140

in vec3 position;
in vec2 texture_coords;
in vec3 normal;
in vec4 colour;

out vec2 uv;
out vec3 v_normal;
out vec3 v_position;
out vec4 v_colour;

uniform mat4 model;
uniform mat4 camera;
uniform mat4 view;

void main() {
    v_colour = colour;
    uv = texture_coords;
    mat4 model_view = inverse(camera) * model;
    v_normal = inverse(transpose(mat3(model_view))) * normal;
    gl_Position = view * model_view * vec4(position, 1.0);
    v_position = gl_Position.xyz / gl_Position.w;
}