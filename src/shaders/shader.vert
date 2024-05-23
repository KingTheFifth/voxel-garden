#version 330

in  vec3 in_position;
in  vec3 in_normal;
in  vec3 in_inst_position;
in  vec4 in_inst_color;
in int is_water;

out vec4 out_inst_color;
flat out vec4 diffuse_sun_c;

uniform mat4 proj_matrix;
uniform mat4 model_matrix;
uniform mat4 camera_matrix;
uniform vec3 sun_direction;
uniform vec3 sun_color;
uniform float time;

void main(void) {
    vec3 pos = in_inst_position + in_position;
    if (is_water == 1) {
        pos.y += 5.0;
    }
    gl_Position = proj_matrix * model_matrix * vec4(pos, 1.0);
    
    vec3 n = normalize(mat3(model_matrix) * in_normal);
    vec3 s = normalize(mat3(camera_matrix)*sun_direction);
    diffuse_sun_c = vec4(0.5 * sun_color * max(0.0, dot(n, s)), 1.0);


    out_inst_color = in_inst_color;
}
