#import bevy_pbr::mesh_view_bindings globals


// Simple hash function
fn hash(n: f32) -> f32 {
    let x = sin(n) * 43758.5453;
    return fract(x);
}

// Basic noise function using hash
fn noise(uv: vec2<f32>) -> f32 {
    let p = floor(uv);
    var f = fract(uv);
    f = f * f * (3.0 - 2.0 * f);

    var n = p.x + p.y * 57.0;

    return mix(mix(hash(n + 0.0), hash(n + 1.0), f.x),
               mix(hash(n + 57.0), hash(n + 58.0), f.x), f.y);
}

// Function to create a line pattern
fn linePattern(uv: vec2<f32>) -> f32 {
    let lines = sin(uv.y * 30.0 + uv.x * 10.0) * noise(uv * 2.0);
    return smoothstep(0.4, 0.6, abs(lines));
}


@fragment
fn fragment(
    @builtin(position) fragCoord: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) normals: vec3<f32>,
    @location(2) uv: vec2<f32>,
) -> @location(0) vec4<f32> {

    let resolution = vec2<f32>(1000.0, 500.0); // Use your viewport's resolution here
    let aspect = resolution.x / resolution.y;

    var uv: vec2<f32> = fragCoord.xy / resolution;
    uv = uv * 2.0 - vec2<f32>(1.0, 1.0);
    uv.x *= aspect; // Adjust the x coordinate by the aspect ratio

    // Convert to normalized space (0.0 to 1.0)

    // Create the bands based on the sine of the Y coordinate, adding some noise
    let bands: f32 = sin(uv.y * 10.0 + noise(uv * 10.0)) * 0.5 + 0.5;

    // Create the curve to give the illusion of a sphere
    let curve: f32 = smoothstep(0.2, 0.8, uv.x) * smoothstep(0.2, 0.8, 1.0 - uv.x);

    // Multiply the bands by the curve to confine them to a circular shape
    let circularBands: f32 = bands * curve;

    // Convert the circular bands to a grayscale color
    let color: vec4<f32> = vec4<f32>(circularBands, circularBands, circularBands, 1.0);

    return vec4<f32>(sin(globals.time * 10.0), 0.0, 0.0, 1.0);
}


//@fragment
//fn fragment(
//    @builtin(position) fragCoord: vec4<f32>,
//    @location(0) world_position: vec4<f32>,
//    @location(1) normals: vec3<f32>,
//    @location(2) uv: vec2<f32>
//) -> @location(0) vec4<f32> {
//
//    let resolution = vec2<f32>(1000.0, 500.0); // Use your viewport's resolution here
//    let aspect = resolution.x / resolution.y;
//
//    var uv: vec2<f32> = fragCoord.xy / resolution;
//    uv = uv * 2.0 - vec2<f32>(1.0, 1.0);
//    uv.x *= aspect; // Adjust the x coordinate by the aspect ratio
//
//    let normalizedUV: vec2<f32> = uv * 2.0 - 1.0; // Convert to -1 to 1 range
//
//    // Calculate the distance to the center of the image
//    let distanceToCenter: f32 = length(normalizedUV);
//
//    // Generate a pattern of concentric circles using a sine function
//    // Adjust the scale to control the density of the circles
//    let scale: f32 = 10.0;
//    let pattern: f32 = 0.5 + 0.5 * sin(distanceToCenter * scale);
//
//    // Use the pattern to mix between two colors
//    let color1: vec4<f32> = vec4<f32>(0.1, 0.2, 0.3, 1.0); // Dark color
//    let color2: vec4<f32> = vec4<f32>(0.8, 0.9, 1.0, 1.0); // Light color
//    let mixedColor: vec4<f32> = mix(color1, color2, pattern);
//
//    return mixedColor;
//}

//@fragment
//fn fragment(
//    @builtin(position) fragCoord: vec4<f32>,
//    @location(0) world_position: vec4<f32>,
//    @location(1) normals: vec3<f32>,
//    @location(2) uv: vec2<f32>
//) -> @location(0) vec4<f32> {
//
//    let resolution = vec2<f32>(1000.0, 500.0); // Use your viewport's resolution here
//    let aspect = resolution.x / resolution.y;
//
//    var uv: vec2<f32> = fragCoord.xy / resolution;
//    uv = uv * 2.0 - vec2<f32>(1.0, 1.0);
//    uv.x *= aspect; // Adjust the x coordinate by the aspect ratio
//
//    let coneApex = vec2<f32>(1.0, -0.5); // The tip of the cone pointing downward
//    let leftLineDirection = normalize(vec2<f32>(-0.3, 0.5)); // Direction for the left line of the cone
//    let rightLineDirection = normalize(vec2<f32>(0.3, 0.5)); // Direction for the right line of the cone
//
//    let outlineWidth = 0.01; // Width of the cone's outline
//
//    // Calculate the minimum distance to the left and right lines
//    let leftLine = dot(uv - coneApex, leftLineDirection);
//    let distLeftLine = length((uv - coneApex) - leftLine * leftLineDirection);
//    let rightLine = dot(uv - coneApex, rightLineDirection);
//    let distRightLine = length((uv - coneApex) - rightLine * rightLineDirection);
//
//    // Calculate whether the point is within the cone area
//    let isInCone = leftLine > 0.0 && rightLine > 0.0 && (distLeftLine + distRightLine) < 1.0;
//
//    // Calculate the flame effect based on the distance from the apex
//    let flameIntensity = smoothstep(0.0, 1.0, 1.0 - length(uv - coneApex));
//    let flameColor = vec3<f32>(1.0, flameIntensity, 0.0); // Gradient from yellow to red
//
//    // Determine if the pixel is within the outline width of either line
//        let onLeftLine = leftLine > 0.0 && distLeftLine < outlineWidth;
//        let onRightLine = rightLine > 0.0 && distRightLine < outlineWidth;
//
//    // Draw the lines if within the outline width
//    if (onLeftLine || onRightLine) {
//        return vec4<f32>(1.0, 1.0, 1.0, 1.0); // White color for the outline
//    } else if (isInCone) {
//        return vec4<f32>(flameColor, 1.5); // Flame color inside the cone
//    } else {
//        return vec4<f32>(0.0, 0.0, 0.0, 0.0); // Transparent otherwise
//    }
//}
