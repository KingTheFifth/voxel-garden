#version 330

in vec4 out_inst_color;
flat in vec4 diffuse_sun_c;

void main(void) {
    gl_FragColor = out_inst_color + diffuse_sun_c;
}
