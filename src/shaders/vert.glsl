struct Rect {
    float x;
    float y;
    float w;
    float h;
};

attribute vec2 vertPosition;

attribute vec2 texCoordIn;

uniform mat4 projection;

uniform vec2 textureDimensions;

uniform Rect destRect;

varying highp vec2 texCoord;

void main() {
    vec2 position = vec2(vertPosition.x * destRect.w, vertPosition.y * destRect.h) + vec2(destRect.x, destRect.y);
    gl_Position = projection * vec4(position, 0.0, 1.0);
    texCoord = texCoordIn;
}