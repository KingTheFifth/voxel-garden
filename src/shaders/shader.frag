#version 330

in  vec4 v_position;

void main(void) {
    gl_FragColor = vec4(vec3(v_position), 1.0);
}
