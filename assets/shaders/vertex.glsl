#version 140

in vec3 position;
in vec2 texture_coords;
in vec3 normal;
in vec3 colour;

out vec2 uv;
out vec3 v_normal;
out vec3 v_position;
out vec3 vertex_colour;

uniform mat4 matrix;
uniform mat4 view;

void main() {
    vertex_colour = colour;
    uv = texture_coords;
    v_normal = transpose(inverse(mat3(matrix))) * normal;
    gl_Position = view * matrix * vec4(position, 1.0);
    v_position = gl_Position.xyz / gl_Position.w;
}