#version 330

in  vec3 in_position;
in  vec3 in_normal;
in  vec3 in_inst_position;
in  vec4 in_inst_color;

out vec4 out_inst_color;

uniform mat4 proj_matrix;
uniform mat4 model_matrix;
uniform mat4 camera_matrix;
uniform mat4 normal_matrix;
uniform vec3 sun_direction;
uniform vec3 sun_color;

void main(void) {
    gl_Position = proj_matrix * model_matrix * vec4(in_position + in_inst_position, 1.0);
    vec3 n = normalize(mat3(normal_matrix) * in_normal);
    vec3 s = normalize(mat3(camera_matrix) * sun_direction);
    vec3 diffuse_c = 0.5 * sun_color * max(0.0, dot(n, s));
    
    out_inst_color = in_inst_color + vec4(diffuse_c, 1.0);
}
