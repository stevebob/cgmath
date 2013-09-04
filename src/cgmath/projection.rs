// Copyright 2013 The CGMath Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::num::{zero, one};

use matrix::Mat4;
use util::two;

/// Create a perspective projection matrix
///
/// Note: the fovy parameter should be specified in degrees.
///
/// This is the equivalent of the gluPerspective function, the algorithm of which
/// can be found [here](http://www.opengl.org/wiki/GluPerspective_code).
pub fn perspective<S: Clone + Float>(fovy: S, aspectRatio: S, near: S, far: S) -> Mat4<S> {
    let ymax = near * (fovy / two::<S>()).to_radians().tan();
    let xmax = ymax * aspectRatio;

    frustum(-xmax, xmax, -ymax, ymax, near, far)
}

/// Define a view frustrum
///
/// This is the equivalent of the now deprecated [glFrustrum]
/// (http://www.opengl.org/sdk/docs/man2/xhtml/glFrustum.xml) function.
pub fn frustum<S: Clone + Float>(left: S, right: S, bottom: S, top: S, near: S, far: S) -> Mat4<S> {
    let c0r0 = (two::<S>() * near) / (right - left);
    let c0r1 = zero();
    let c0r2 = zero();
    let c0r3 = zero();

    let c1r0 = zero();
    let c1r1 = (two::<S>() * near) / (top - bottom);
    let c1r2 = zero();
    let c1r3 = zero();

    let c2r0 = (right + left) / (right - left);
    let c2r1 = (top + bottom) / (top - bottom);
    let c2r2 = -(far + near) / (far - near);
    let c2r3 = -one::<S>();

    let c3r0 = zero();
    let c3r1 = zero();
    let c3r2 = -(two::<S>() * far * near) / (far - near);
    let c3r3 = zero();

    Mat4::new(c0r0, c0r1, c0r2, c0r3,
              c1r0, c1r1, c1r2, c1r3,
              c2r0, c2r1, c2r2, c2r3,
              c3r0, c3r1, c3r2, c3r3)
}

/// Create an orthographic projection matrix
///
/// This is the equivalent of the now deprecated [glOrtho]
/// (http://www.opengl.org/sdk/docs/man2/xhtml/glOrtho.xml) function.
pub fn ortho<S: Clone + Float>(left: S, right: S, bottom: S, top: S, near: S, far: S) -> Mat4<S> {
    let c0r0 = two::<S>() / (right - left);
    let c0r1 = zero();
    let c0r2 = zero();
    let c0r3 = zero();

    let c1r0 = zero();
    let c1r1 = two::<S>() / (top - bottom);
    let c1r2 = zero();
    let c1r3 = zero();

    let c2r0 = zero();
    let c2r1 = zero();
    let c2r2 = -two::<S>() / (far - near);
    let c2r3 = zero();

    let c3r0 = -(right + left) / (right - left);
    let c3r1 = -(top + bottom) / (top - bottom);
    let c3r2 = -(far + near) / (far - near);
    let c3r3 = one();

    Mat4::new(c0r0, c0r1, c0r2, c0r3,
              c1r0, c1r1, c1r2, c1r3,
              c2r0, c2r1, c2r2, c2r3,
              c3r0, c3r1, c3r2, c3r3)
}