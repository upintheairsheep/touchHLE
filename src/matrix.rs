/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! Transformation matrices implementation.
//!
//! This is used for, among other things:
//! - Generating transformations for OpenGL ES
//! - Translating between co-ordinate spaces for user inputs and display outputs
//!   (e.g. when the screen is rotated)
//! - Implementing transformation matrix APIs (e.g. `CGAffineTransform`)
//! - Interoperability between the above

/// OpenGL-style column-major matrix.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix<const N: usize>([[f32; N]; N]);

impl<const N: usize> Matrix<N> {
    pub fn identity() -> Self {
        let mut matrix = [[0f32; N]; N];
        #[allow(clippy::needless_range_loop)]
        for i in 0..N {
            matrix[i][i] = 1f32;
        }
        Matrix(matrix)
    }

    pub fn columns(&self) -> &[[f32; N]; N] {
        &self.0
    }

    // This constructor is used instead of direct field access so there can be
    // some flexibility with adjusting the representation.
    pub fn from_columns(columns: [[f32; N]; N]) -> Self {
        Matrix(columns)
    }

    pub fn from<const M: usize>(other: &Matrix<M>) -> Self {
        // FIXME: This is probably wrong for homogenous co-ordinates.
        let mut new = Self::identity();
        for i in 0..M {
            for j in 0..M {
                new.0[i][j] = other.0[i][j];
            }
        }
        new
    }

    pub fn multiply(&self, other: &Self) -> Self {
        let mut res = [[0f32; N]; N];
        #[allow(clippy::needless_range_loop)]
        for i in 0..N {
            for j in 0..N {
                for k in 0..N {
                    res[i][j] += self.0[i][k] * other.0[k][j];
                }
            }
        }
        Matrix(res)
    }

    #[allow(dead_code)]
    pub fn transpose(&self) -> Self {
        let mut res = [[0f32; N]; N];
        #[allow(clippy::needless_range_loop)]
        for i in 0..N {
            for j in 0..N {
                res[j][i] = self.0[i][j];
            }
        }
        Matrix(res)
    }

    /// Transform a column vector using the matrix: computes M × v where M is
    /// the matrix and v is `vector` as a column vector.
    pub fn transform(&self, vector: [f32; N]) -> [f32; N] {
        let mut new = [0f32; N];
        for (i, basis_vector) in self.columns().iter().enumerate() {
            for j in 0..N {
                new[j] += basis_vector[j] * vector[i];
            }
        }
        new
    }
}

impl Matrix<2> {
    pub fn determinant(&self) -> f32 {
        // https://en.wikipedia.org/wiki/Determinant
        let &Matrix([[a, c], [b, d]]) = self;
        a * d - b * c
    }

    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        // Square matrix is only invertible if its determinant is nonzero
        if det == 0.0 {
            return None;
        }
        // https://en.wikipedia.org/wiki/Invertible_matrix#Inversion_of_2_%C3%97_2_matrices
        let &Matrix([[a, c], [b, d]]) = self;
        Some(Matrix([
            [1.0 / det * d, 1.0 / det * -c],
            [1.0 / det * -b, 1.0 / det * a],
        ]))
    }

    pub fn y_flip() -> Matrix<2> {
        Matrix([[1.0, 0.0], [0.0, -1.0]])
    }

    pub fn z_rotation(angle: f32) -> Matrix<2> {
        Matrix([[angle.cos(), angle.sin()], [-angle.sin(), angle.cos()]])
    }

    pub fn scale_2d(x: f32, y: f32) -> Matrix<2> {
        Matrix([[x, 0.0], [0.0, y]])
    }
}
impl Matrix<3> {
    pub fn determinant(&self) -> f32 {
        // https://en.wikipedia.org/wiki/Determinant#Leibniz_formula
        let &Matrix([[a, d, g], [b, e, h], [c, f, i]]) = self;
        a * e * i + b * f * g + c * d * h - c * e * g - b * d * i - a * f * h
    }

    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        // Square matrix is only invertible if its determinant is nonzero
        if det == 0.0 {
            return None;
        }
        // https://en.wikipedia.org/wiki/Invertible_matrix#Inversion_of_3_%C3%97_3_matrices
        let &Matrix([[a, d, g], [b, e, h], [c, f, i]]) = self;
        let a_ = e * i - f * h;
        let b_ = -(d * i - f * g);
        let c_ = d * h - e * g;
        let d_ = -(b * i - c * h);
        let e_ = a * i - c * g;
        let f_ = -(a * h - b * g);
        let g_ = b * f - c * e;
        let h_ = -(a * f - c * d);
        let i_ = a * e - b * d;
        Some(Matrix([
            [1.0 / det * a_, 1.0 / det * b_, 1.0 / det * c_],
            [1.0 / det * d_, 1.0 / det * e_, 1.0 / det * f_],
            [1.0 / det * g_, 1.0 / det * h_, 1.0 / det * i_],
        ]))
    }

    pub fn x_rotation(angle: f32) -> Matrix<3> {
        Matrix([
            [1.0, 0.0, 0.0],
            [0.0, angle.cos(), angle.sin()],
            [0.0, -angle.sin(), angle.cos()],
        ])
    }
    pub fn y_rotation(angle: f32) -> Matrix<3> {
        Matrix([
            [angle.cos(), 0.0, -angle.sin()],
            [0.0, 1.0, 0.0],
            [angle.sin(), 0.0, angle.cos()],
        ])
    }

    pub fn translate_2d(x: f32, y: f32) -> Matrix<3> {
        Matrix([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [x, y, 1.0]])
    }
}
impl Matrix<4> {
    pub fn translate_3d(x: f32, y: f32, z: f32) -> Matrix<4> {
        Matrix([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [x, y, z, 1.0],
        ])
    }
}
