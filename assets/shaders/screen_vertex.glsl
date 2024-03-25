#version 140

in vec3 position;
in vec2 texture_coords;

out vec2 uv;
out vec3 v_position;
uniform mat4 camera;

void main() {
    uv = texture_coords;
    gl_Position = vec4(position, 1.0) * camera;
    v_position = gl_Position.xyz / gl_Position.w;
}