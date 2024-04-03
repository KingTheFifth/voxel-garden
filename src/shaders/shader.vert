#version 330

in  vec3 in_position;
out vec4 v_position;

uniform mat4 proj_matrix;
uniform mat4 model_matrix;

void main(void) {
    gl_Position = proj_matrix * model_matrix * vec4(in_position, 1.0);
    v_position = vec4(2.0 * in_position, 1.0);
}
