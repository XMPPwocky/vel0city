#version 110
uniform mat4 transform;
attribute vec3 position;
attribute vec2 texcoords;
varying vec2 v_texcoords;
void main() {
    gl_Position = transform * vec4(position, 1.0); 
    v_texcoords = texcoords;
}
