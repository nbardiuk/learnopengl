#version 330 core

uniform sampler2D texture1;
uniform sampler2D texture2;

in vec3 ourColor;
in vec2 TexCoord;

out vec4 FragColor;

void main() {
    FragColor = mix(texture(texture1, TexCoord)
                  , texture(texture2, vec2(TexCoord.x, -TexCoord.y))
                  , 0.2);
}
