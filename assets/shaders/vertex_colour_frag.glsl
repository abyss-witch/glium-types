#version 140

out vec4 colour;
in vec3 vertex_colour;

void main() {
    colour = vec4(vertex_colour, 1.0);
}