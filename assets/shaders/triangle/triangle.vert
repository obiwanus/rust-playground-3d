#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec2 TexCoord;

uniform float x_offset;

out VS_OUTPUT {
    vec2 TexCoord;
} OUT;

void main() {
    gl_Position = vec4(Position.x + x_offset, Position.y, Position.z, 1.0);
    OUT.TexCoord = TexCoord;
}
