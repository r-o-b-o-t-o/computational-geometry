#version 330 core

in vec3 colorDiffuse;
in vec4 colorSpecular;

out vec4 fragColor;

void main() {
    fragColor = vec4(colorDiffuse.rgb, 1.0);
}
