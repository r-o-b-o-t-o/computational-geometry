#version 330 core

in vec3 vertNormal;
in vec3 vertPos;

out vec4 fragColor;

uniform vec3 viewPosition;

uniform vec3 materialAmbient;
uniform vec3 materialDiffuse;
uniform vec3 materialSpecular;
uniform float materialShininess;

uniform vec3 lightPosition;
uniform vec3 lightAmbient;
uniform vec3 lightDiffuse;
uniform vec3 lightSpecular;

void main() {
    vec3 ambient = materialAmbient * lightAmbient;

    vec3 norm = normalize(vertNormal);
    vec3 lightDir = normalize(lightPosition - vertPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * materialDiffuse * lightDiffuse;

    vec3 viewDir = normalize(viewPosition - vertPos);
    vec3 reflectionDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectionDir), 0.0f), materialShininess);
    vec3 specular = spec * materialSpecular * lightSpecular;

    vec3 result = ambient + diffuse + specular;
    fragColor = vec4(result.rgb, 1.0);
}
