#version 330 core

in vec3 position;
in vec3 normal;
in vec3 color_diffuse;
in vec4 color_specular;

out vec3 vertNormal;
out vec3 vertPos;

out vec3 colorDiffuse;
out vec4 colorSpecular;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    vertNormal = mat3(transpose(inverse(model))) * normal;
    vertPos = vec3(model * vec4(position.xyz, 1.0));

    colorDiffuse = color_diffuse;
    colorSpecular = color_specular;

    gl_Position = projection * view * vec4(vertPos.xyz, 1.0);
}
