#version 330 core
layout (location = 0) in vec2 in_position;
layout (location = 1) in vec3 in_color;

out Vertex {
    vec3 color;
} vertex_data;

void main() {
    gl_Position = vec4(in_position, 0.0, 1.0);
    
    vertex_data.color = in_color;
}