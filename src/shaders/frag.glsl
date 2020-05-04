varying highp vec2 texCoord;

uniform sampler2D uSampler;

void main() {
    gl_FragColor = texture2D(uSampler, texCoord);
}