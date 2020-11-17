#version 330 core
struct Material {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    float shininess;
};

struct PointLight {
    vec3 position;
  
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;

    float constant;
    float linear;
    float quadratic;
};

struct DirectionalLight {
    vec3 direction;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

in vec3 FragPos;
in vec3 Normal;

uniform vec3 view_pos;
uniform Material material;
uniform PointLight point_light;
uniform DirectionalLight directional_light;

out vec4 FragColor;

vec3 apply_point_light(in PointLight light, in Material material, in vec3 normal) {
    vec3 posDelta = light.position - FragPos;

    // Ambient
    vec3 ambient = light.ambient * material.ambient;
    
    // Diffuse
    vec3 lightDir = normalize(posDelta);
    float diff = max(dot(normal, lightDir), 0.0);
    vec3 diffuse = light.diffuse * (diff * material.diffuse);

    // Specular
    vec3 viewDir = normalize(view_pos - FragPos);
    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    vec3 specular = light.specular * (spec * material.specular);

    // Attenuation
    float distance = length(posDelta);
    float attenuation = 1.0 / (
        light.constant + 
        light.linear * distance + 
        light.quadratic * (distance * distance)
    );

    ambient *= attenuation;
    diffuse *= attenuation;
    specular *= attenuation;

    return ambient + diffuse + specular;
}

vec3 apply_directional_light(in DirectionalLight light, in Material material, in vec3 normal) {
    // Ambient
    vec3 ambient = light.ambient * material.ambient;
    
    // Diffuse
    vec3 lightDir = normalize(-light.direction);
    float diff = max(dot(normal, lightDir), 0.0);
    vec3 diffuse = light.diffuse * (diff * material.diffuse);

    // Specular
    vec3 viewDir = normalize(view_pos - FragPos);
    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    vec3 specular = light.specular * (spec * material.specular);

    return ambient + diffuse + specular;
}

void main()
{
    vec3 result = vec3(0.0);
    vec3 norm = normalize(Normal);
    result += apply_point_light(point_light, material, norm);
    result += apply_directional_light(directional_light, material, norm);

    FragColor = vec4(result, 1.0);
}
