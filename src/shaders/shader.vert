#version 330

in  vec3 in_position;
in  vec3 in_normal;
in  vec3 in_inst_position;
in  vec4 in_inst_color;
in uint is_water;

flat out vec4 out_inst_color;

uniform mat4 proj_matrix;
uniform mat4 model_matrix;
uniform mat4 camera_matrix;
uniform vec3 sun_direction;
uniform vec4 sun_color;
uniform vec4 ambient_light_color;
uniform float time;
uniform float ambient_water_activity;
uniform float wave_water_peak;
uniform float wave_water_pow;
uniform float wave_water_x_factor;
uniform float wave_water_z_factor;
uniform float wave_water_frequency;
uniform sampler2D water_random;

void main(void) {
    vec3 new_inst_pos = in_inst_position;
    if (is_water != uint(0)) {
        float x = in_inst_position.x;
        float z = in_inst_position.z;
        float random = texture(water_random, in_inst_position.xz / 1024.0).r;
        float amp = (sin((time + x*x * wave_water_x_factor + z*z * wave_water_z_factor) / wave_water_frequency) + 1.0) / 2.0;
        new_inst_pos.y = ((pow(abs(amp), wave_water_pow) * wave_water_peak) + ambient_water_activity * (sin(time * (random * 3.0) + random*12.0) + 1.0) / 2.0 + in_inst_position.y);
    }
    vec3 pos = new_inst_pos + in_position;
    gl_Position = proj_matrix * model_matrix * vec4(pos, 1.0);

    vec3 n = normalize(mat3(model_matrix) * in_normal);
    vec3 s = normalize(mat3(camera_matrix)*sun_direction);
    vec4 color_ambient = ambient_light_color * in_inst_color;
    vec4 color_sun = sun_color * max(0.0, dot(n, s)) * in_inst_color;

    out_inst_color = color_ambient + color_sun;
}
