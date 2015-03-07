#version 110
uniform sampler2D color;
varying vec2 v_texcoords;
void main() {
    gl_FragColor = texture2D(color, v_texcoords);
}
