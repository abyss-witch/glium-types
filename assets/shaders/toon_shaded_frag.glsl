#version 140

out vec4 colour;
in vec3 v_normal;
in vec3 v_position;

const vec3 shadow_colour = vec3(0.3, 0.0, 0.0);
const vec3 light_colour = vec3(0.6, 0.2, 0.3);
const vec3 highlight_colour = vec3(1.0, 0.8, 0.9);

const vec3 u_light = vec3(1.0, 0.5, -0.5);
void main() {
    float diffuse = float(dot(normalize(v_normal), u_light) > 0.0);

    vec3 camera_dir = normalize(-v_position);
    vec3 half_direction = normalize(u_light + camera_dir);
    float specular = float(dot(half_direction, normalize(v_normal)) > 0.98);

    vec3 a = mix(shadow_colour, light_colour, diffuse);

    colour = vec4(mix(a, highlight_colour, specular), 1.0);
}