#version 330

in  vec3 in_position;
in  vec3 in_normal;
in  vec3 in_inst_position;
in  vec4 in_inst_color;
in uint is_water;

out vec4 out_inst_color;
flat out vec4 diffuse_sun_c;

uniform mat4 proj_matrix;
uniform mat4 model_matrix;
uniform mat4 camera_matrix;
uniform vec3 sun_direction;
uniform vec3 sun_color;
uniform float time;

void main(void) {
    vec3 new_inst_pos = in_inst_position;
    if (is_water != uint(0)) {
        float x_factor = 1 / (0.001*0.5);
        float z_factor = 1 / (0.0023 * 0.5);
        float x = mod(in_inst_position.x, x_factor);
        float z = mod(in_inst_position.z, z_factor);
        float x_square = x * x;
        float z_square = z * z;
        float amp = sin(time + x_square * 1/x_factor + z_square * 1/z_factor);
        float amp_sqare = amp * amp;
        new_inst_pos.y = ((amp_sqare * 0.7) + in_inst_position.y);
    }
    vec3 pos = new_inst_pos + in_position;
    gl_Position = proj_matrix * model_matrix * vec4(pos, 1.0);
    
    vec3 n = normalize(mat3(model_matrix) * in_normal);
    vec3 s = normalize(mat3(camera_matrix)*sun_direction);
    diffuse_sun_c = vec4(0.5 * sun_color * max(0.0, dot(n, s)), 1.0);


    out_inst_color = in_inst_color;
}
