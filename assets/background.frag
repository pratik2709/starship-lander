#version 450

layout(location = 0) in vec2 fragCoord;
layout(location = 0) out vec4 outColor;

layout(set = 0, binding = 0) uniform Resolution {
    vec2 iResolution;
};

#define NUM_LAYERS = 6

mat2 rotate(float a){
    float s = sin(a);
    float c = cos(a);
    return mat2(c, -s, s, c);
}

float Star(vec2 uv, float flare){
    float d = length(uv);
    float m = 0.05/d;

    // add max so that rays do not become negative
    float rays = max(0.0, 1.0 - abs(uv.x * uv.y * 1000.0));

    m += rays * flare;

    //rotate
    uv *= rotate(3.1415/4.0);

    rays = max(0.0, 1.0 - abs(uv.x * uv.y * 1000.0));

    m += rays*0.3 * flare;

    // completly faded out at x, starting at y, source value for interpolation
    m *= smoothstep(1.0, 0.2 ,d);
    return m;
}

//random number genrerator function
float rand(vec2 p){
    p = fract(p*vec2(123.34, 456.21));
    p += dot(p, p+45.32);
    return fract(p.x*p.y);
}

vec3 StarLayer(vec2 uv){

    // length of the point from the origin

    vec3 col = vec3(0);

    //fractional uv componenet
    vec2 gv = fract(uv) - 0.5;
    // integer component
    vec2 id = floor(uv);

    for(int y = -1; y <= 1; y++){
        for(int x = -1; x <= 1; x++){
              vec2 offs = vec2(x, y ) ;

              // random number betweeen 0 and 1
              float random_number = rand(id + offs);
              float size = fract(random_number* 345.32);
              float flare_randomize = smoothstep(0.85, 1.0, size);
              float star = Star(gv -offs - (vec2(random_number, fract(random_number*34.0)) - 0.5), flare_randomize);

              //make stars of different sizes
              vec3 color =  sin(vec3(0.2,0.3,0.9) * fract(random_number * 2345.2)*123.2) * 0.5 + 0.5;

              //remove green channel
              color *= vec3(1.,0.5,1. + size);

              col += star*size * color;
        }
    }
    return col;

}

void main() {
    vec2 uv = (fragCoord * iResolution.xy)/iResolution.y;
    uv *= 1500.0;

    vec3 col = vec3(0);
    col += StarLayer(uv);

    // Output to screen
    outColor = vec4(col, 1.0);
}
