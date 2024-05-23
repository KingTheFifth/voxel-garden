#version 330

in  vec3 in_position;
in  vec3 in_normal;
in  vec3 in_inst_position;
in  vec4 in_inst_color;

flat out vec4 out_inst_color;

uniform mat4 proj_matrix;
uniform mat4 model_matrix;
uniform mat4 camera_matrix;
uniform vec3 sun_direction;
uniform vec4 sun_color;
uniform vec4 ambient_light_color;

void main(void) {
    gl_Position = proj_matrix * model_matrix * vec4(in_position + in_inst_position, 1.0);

    vec3 n = normalize(mat3(model_matrix) * in_normal);
    vec3 s = normalize(mat3(camera_matrix)*sun_direction);
    vec4 color_ambient = ambient_light_color * in_inst_color;
    vec4 color_sun = sun_color * max(0.0, dot(n, s)) * in_inst_color;

    out_inst_color = color_ambient + color_sun;
}
