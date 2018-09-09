#version 330 core

in Vertex {
    vec3 color;
} vertex_data;

out vec3 out_color;

void main() {
    out_color = vertex_data.color;
}