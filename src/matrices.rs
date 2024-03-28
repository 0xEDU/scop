use std::fmt;
use std::ops;


use cgmath::Angle;
use cgmath::Deg;
use cgmath::Rad;

use crate::tuple::cross;
use crate::tuple::dot;
use crate::tuple::normalize;
use crate::tuple::Tuple;
use crate::tuple::Vector;

/* Matrix declaration and implementation ==================================== */
#[derive(Clone, Copy)]
pub struct Matrix {
    pub data: [[f32; 4]; 4],
    pub size: usize,
}

impl Matrix {
    /* Special constructors ------------------------------------------------ */
    pub fn new() -> Self {
        Matrix {
            data: [[0.0; 4]; 4],
            size: 4,
        }
    }

    pub fn as_ptr(&self) -> *const f32 {
        &self.data[0][0]
    }

    pub fn identity() -> Self {
        let mut id: Matrix = Matrix::new();
        id.data[0][0] = 1.0;
        id.data[1][1] = 1.0;
        id.data[2][2] = 1.0;
        id.data[3][3] = 1.0;
        id
    }

    pub fn from_translation(v: Vector) -> Self {
        let mut m = Matrix::identity();
        m.data[3][0] = v.x;
        m.data[3][1] = v.y;
        m.data[3][2] = v.z;
        m
    }

    // pub fn from_angle_z(angle: f32) -> Self {
    //     let mut m = Matrix::identity();
    //     m.data[0][0] = angle.cos();
    //     m.data[0][1] = angle.sin();
    //     m.data[1][0] = -angle.sin();
    //     m.data[1][1] = angle.cos();
    //     m
    // }
    //
    // pub fn from_angle_x(angle: f32) -> Self {
    //     let mut m = Matrix::identity();
    //     let theta = Deg(angle);
    //     let (s, c) = theta.sin_cos();
    //     m.data[1][1] = c;
    //     m.data[1][2] = -s;
    //     m.data[2][1] = s;
    //     m.data[2][2] = c;
    //     m
    // }

    pub fn from_axis_angle(axis: Vector, angle: f32) -> Self {
        let mut m = Matrix::new();
        let (s, c) = Rad(angle).sin_cos();
        let _1subc = 1. - c;
        m.data[0][0] = _1subc * axis.x * axis.x + c;
        m.data[0][1] = _1subc * axis.x * axis.y + s * axis.z;
        m.data[0][2] = _1subc * axis.x * axis.z - s * axis.y;
        m.data[1][0] = _1subc * axis.x * axis.y - s * axis.z;
        m.data[1][1] = _1subc * axis.y * axis.y + c;
        m.data[1][2] = _1subc * axis.y * axis.z + s * axis.x;
        m.data[2][0] = _1subc * axis.x * axis.z + s * axis.y;
        m.data[2][1] = _1subc * axis.y * axis.z - s * axis.x;
        m.data[2][2] = _1subc * axis.z * axis.z + c;
        m.data[3][3] = 1.;
        m
    }
    /* --------------------------------------------------------------------- */

    /* Matrix opeartions --------------------------------------------------- */
    // pub fn transpose(&self) -> Matrix {
    //     let mut t = Matrix::new();
    //     for i in 0..4 {
    //         for j in 0..4 {
    //             t.data[i][j] = self.data[j][i];
    //         }
    //     }
    //     t
    // }
    //
    // pub fn determinant(&self) -> f32 {
    //     let mut det: f32 = 0.0;
    //     if self.size == 2 {
    //         return (self.data[0][0] * self.data[1][1]) - (self.data[0][1] * self.data[1][0])
    //     }
    //     for i in 0..self.size {
    //         det += self.data[0][i] * self.cofactor(0, i);
    //     }
    //     det
    // }

    // pub fn submatrix(&self, row: usize, col: usize) -> Matrix {
    //     let mut sub_matrix = Matrix::new();
    //
    //     let mut sub_row = 0;
    //     for i in 0..4 {
    //         if i == row {
    //             continue;
    //         }
    //
    //         let mut sub_col = 0;
    //         for j in 0..4 {
    //             if j == col {
    //                 continue;
    //             }
    //             sub_matrix.data[sub_row][sub_col] = self.data[i][j];
    //             sub_col += 1;
    //         }
    //         sub_row += 1;
    //     }
    //     sub_matrix.size  = self.size - 1;
    //     sub_matrix
    // }
    //
    // pub fn minor(&self, row: usize, col: usize) -> f32 {
    //     let sub_matrix = self.submatrix(row, col);
    //     sub_matrix.determinant()
    // }
    //
    // pub fn cofactor(&self, row: usize, col: usize) -> f32 {
    //     let minor = self.minor(row, col);
    //     if (col + row) % 2 == 0 { minor } else { minor * -1.0 }
    // }
    //
    // pub fn is_invertible(&self) -> bool {
    //     if self.determinant() == 0.0 { false } else { true }
    // }
    //
    // pub fn inverse(&self) -> Result<Matrix, ()> {
    //     if !self.is_invertible() {
    //         return Err(());
    //     }
    //     let mut inverted: Matrix = Matrix::new();
    //     inverted.size = self.size;
    //     for row in 0..self.size {
    //         for col in 0..self.size {
    //             let c = self.cofactor(row, col);
    //             inverted.data[col][row] = c / self.determinant();
    //         }
    //     }
    //     Ok(inverted)
    // }

    pub fn look_at(camera_position: Tuple, camera_target_position: Tuple, up_vector: Tuple) -> Self {
        let mut m = Matrix::identity();

        let camera_direction = normalize(camera_target_position - camera_position);
        let camera_right = normalize(cross(camera_direction, up_vector));
        let camera_up = cross(camera_right, camera_direction);

        m.data[0][0] = camera_right.x;
        m.data[1][0] = camera_right.y;
        m.data[2][0] = camera_right.z;

        m.data[0][1] = camera_up.x;
        m.data[1][1] = camera_up.y;
        m.data[2][1] = camera_up.z;

        m.data[0][2] = -camera_direction.x;
        m.data[1][2] = -camera_direction.y;
        m.data[2][2] = -camera_direction.z;

        m.data[3][0] = -dot(camera_position, camera_right);
        m.data[3][1] = -dot(camera_position, camera_up);
        m.data[3][2] = dot(camera_position, camera_direction);

        m
    }
    /* --------------------------------------------------------------------- */
}
/* ========================================================================== */

/* Operator overloads for Matrix ============================================ */
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            for &value in row {
                write!(f, "{:10.5} ", value)?
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl std::cmp::PartialEq<Matrix> for Matrix {
    fn eq(&self, rhs: &Self) -> bool {
        self.data == rhs.data
    }
}

impl std::cmp::Eq for Matrix {}

impl ops::Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        let mut product: Matrix = Matrix::new();
        for row in 0..4 {
            for col in 0..4 {
                product.data[row][col] = self.data[row][0] * rhs.data[0][col] +
                                         self.data[row][1] * rhs.data[1][col] +
                                         self.data[row][2] * rhs.data[2][col] +
                                         self.data[row][3] * rhs.data[3][col] as f32
            }
        }
        product
    }
}

impl ops::Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let mut product: Tuple = Tuple::new();
        product.x = self.data[0][0] * rhs.x + self.data[0][1] * rhs.y + self.data[0][2] * rhs.z + self.data[0][3] * rhs.w;
        product.y = self.data[1][0] * rhs.x + self.data[1][1] * rhs.y + self.data[1][2] * rhs.z + self.data[1][3] * rhs.w;
        product.z = self.data[2][0] * rhs.x + self.data[2][1] * rhs.y + self.data[2][2] * rhs.z + self.data[2][3] * rhs.w;
        product.w = self.data[3][0] * rhs.x + self.data[3][1] * rhs.y + self.data[3][2] * rhs.z + self.data[3][3] * rhs.w;
        product
    }
}
/* ========================================================================== */

/* Useful operations ======================================================== */
pub fn perspective(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Matrix {
    let f = Deg(fov / 2.).cot();
    let mut m = Matrix::new();
    m.data[0][0] = f / aspect_ratio;
    m.data[1][1] = f;
    m.data[2][2] = (far + near) / (near - far);
    m.data[3][2] = (2. * far * near) / (near - far);
    m.data[2][3] = -1.;
    m
}
/* ========================================================================== */
