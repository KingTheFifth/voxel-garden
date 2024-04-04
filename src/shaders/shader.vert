#version 330

in  vec3 in_position;
in  vec3 in_inst_position;
in  vec4 in_inst_color;

out vec4 out_inst_color;

uniform mat4 proj_matrix;
uniform mat4 model_matrix;

void main(void) {
    gl_Position = proj_matrix * model_matrix * vec4(in_position + in_inst_position, 1.0);
    out_inst_color = in_inst_color;
}
