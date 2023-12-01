// MIT License

// Copyright (c) 2022 shinylasers.com

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
};

struct DefaultInput {
    res: vec2<f32>,
    time: f32,
};

@group(0) @binding(0)
var<uniform> si: DefaultInput;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    //normalise
    let li = in.pos.xy/si.res;
    return vec4<f32>(abs(sin(si.time*5.9)), abs(sqrt(si.time*0.9)) , 0.0, 1.0);
}
