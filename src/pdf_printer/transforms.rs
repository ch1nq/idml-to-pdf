#[derive(Debug, Default, Clone, Copy)]
struct Matrix3x3<T>([[T; 3]; 3]);

#[derive(Debug, Default, Clone, Copy)]
struct Vector3x1<T>([T; 3]);

impl<T> std::ops::Index<(usize, usize)> for Matrix3x3<T> {
    type Output = T;
    fn index(&self, s: (usize, usize)) -> &T {
        &self.0[s.0][s.1]
    }
}

impl<T> std::ops::IndexMut<(usize, usize)> for Matrix3x3<T> {
    fn index_mut(&mut self, s: (usize, usize)) -> &mut T {
        &mut self.0[s.0][s.1]
    }
}

impl<T> std::ops::Index<usize> for Vector3x1<T> {
    type Output = T;
    fn index(&self, s: usize) -> &T {
        &self.0[s]
    }
}

impl<T> std::ops::IndexMut<usize> for Vector3x1<T> {
    fn index_mut(&mut self, s: usize) -> &mut T {
        &mut self.0[s]
    }
}

impl Matrix3x3<f64> {
    fn transpose(self) -> Self {
        let mut matrix: Self = Default::default();
        for i in 0..3 {
            for j in 0..3 {
                matrix[(i, j)] = self[(j, i)];
            }
        }
        matrix
    }

    fn dot(self, rhs: Self) -> Self {
        let mut matrix: Self = Default::default();
        for i in 0..3 {
            for j in 0..3 {
                matrix[(i, j)] = self[(i, 0)] * rhs[(0, j)]
                    + self[(i, 1)] * rhs[(1, j)]
                    + self[(i, 2)] * rhs[(2, j)];
            }
        }
        matrix
    }

    // Computes the inverse
    fn inverse(self) -> Self {
        let m = &self;
        let det: f64 = m[(0, 0)] * (m[(1, 1)] * m[(2, 2)] - m[(2, 1)] * m[(1, 2)])
            - m[(0, 1)] * (m[(1, 0)] * m[(2, 2)] - m[(1, 2)] * m[(2, 0)])
            + m[(0, 2)] * (m[(1, 0)] * m[(2, 1)] - m[(1, 1)] * m[(2, 0)]);

        let invdet: f64 = 1_f64 / det;

        // inverse of matrix m
        let mut minv: Self = Default::default();
        minv[(0, 0)] = (m[(1, 1)] * m[(2, 2)] - m[(2, 1)] * m[(1, 2)]) * invdet;
        minv[(0, 1)] = (m[(0, 2)] * m[(2, 1)] - m[(0, 1)] * m[(2, 2)]) * invdet;
        minv[(0, 2)] = (m[(0, 1)] * m[(1, 2)] - m[(0, 2)] * m[(1, 1)]) * invdet;
        minv[(1, 0)] = (m[(1, 2)] * m[(2, 0)] - m[(1, 0)] * m[(2, 2)]) * invdet;
        minv[(1, 1)] = (m[(0, 0)] * m[(2, 2)] - m[(0, 2)] * m[(2, 0)]) * invdet;
        minv[(1, 2)] = (m[(1, 0)] * m[(0, 2)] - m[(0, 0)] * m[(1, 2)]) * invdet;
        minv[(2, 0)] = (m[(1, 0)] * m[(2, 1)] - m[(2, 0)] * m[(1, 1)]) * invdet;
        minv[(2, 1)] = (m[(2, 0)] * m[(0, 1)] - m[(0, 0)] * m[(2, 1)]) * invdet;
        minv[(2, 2)] = (m[(0, 0)] * m[(1, 1)] - m[(1, 0)] * m[(0, 1)]) * invdet;

        minv
    }
}

impl Vector3x1<f64> {
    fn dot(self, rhs: Matrix3x3<f64>) -> Vector3x1<f64> {
        let mut vector: Vector3x1<f64> = Default::default();
        for (i, col) in rhs.transpose().0.iter().enumerate() {
            vector[i] = self[0] * col[0] + self[1] * col[1] + self[2] * col[2];
        }
        vector
    }
}

#[derive(Debug)]
pub struct Transform {
    matrix: Matrix3x3<f64>,
    inverse: Option<Matrix3x3<f64>>,
}

/// Returns the identity matrix
pub fn identity() -> Transform {
    let m = Matrix3x3([
        [1_f64, 0_f64, 0_f64],
        [0_f64, 1_f64, 0_f64],
        [0_f64, 0_f64, 1_f64],
    ]);

    Transform {
        matrix: m,
        inverse: None,
    }
}

/// Creates a transformation matrix as specified in the IDML specification, page 102
/// https://wwwimages.adobe.com/content/dam/acom/en/devnet/indesign/sdk/cs6/idml/idml-specification.pdf
pub fn from_values(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Transform {
    let m = Matrix3x3([
        [a.to_owned(), b.to_owned(), 0_f64],
        [c.to_owned(), d.to_owned(), 0_f64],
        [e.to_owned(), f.to_owned(), 1_f64],
    ]);
    Transform {
        matrix: m,
        inverse: None,
    }
}

/// Creates a transformation matrix from a Option of a Vec of the form [a,b,c,d,e,f]
pub fn from_vec(transform: &Option<Vec<f64>>) -> Transform {
    match transform {
        Some(transform) => match transform.as_slice() {
            &[a, b, c, d, e, f] => from_values(a, b, c, d, e, f),
            _ => identity(),
        },
        _ => identity(),
    }
}

pub fn transform_point(x: &f64, y: &f64, transform: &Transform) -> Vec<f64> {
    // Has the form [x, y, 1] in order to match dimensions of transformation matrix
    let homogenous_coords = &Vector3x1([x.to_owned(), y.to_owned(), 1_f64]);
    let point = homogenous_coords.dot(transform.matrix);
    vec![point[0], point[1]]
}

pub fn _transform_point_inverse(x: &f64, y: &f64, transform: &Transform) -> Vec<f64> {
    transform_point(x, y, &inverse(transform))
}

pub fn inverse(transform: &Transform) -> Transform {
    // If we know the inverse, just return that
    let inverse = match &transform.inverse {
        Some(inv) => *inv,
        None => transform.matrix.inverse(),
    };

    Transform {
        matrix: inverse,
        inverse: Some(transform.matrix),
    }
}

/// Returns dot product of matrices
pub fn combine(transform1: &Transform, transform2: &Transform) -> Transform {
    Transform {
        matrix: transform1.matrix.dot(transform2.matrix),
        inverse: None,
    }
}

impl Transform {
    pub fn apply_to_point(&self, x: &f64, y: &f64) -> Vec<f64> {
        transform_point(x, y, &self)
    }

    pub fn combine_with(&self, transform: &Transform) -> Transform {
        combine(&self, transform)
    }

    pub fn inverse(&self) -> Transform {
        inverse(&self)
    }

    fn with_updated_value_at_index(self, value: f64, index: (usize, usize)) -> Transform {
        let mut new_matrix = self.matrix;
        new_matrix[index] = value;
        Transform {
            matrix: new_matrix,
            inverse: None,
        }
    }

    pub fn with_a(self, a: f64) -> Transform {
        self.with_updated_value_at_index(a, (0, 0))
    }

    pub fn with_b(self, b: f64) -> Transform {
        self.with_updated_value_at_index(b, (0, 1))
    }

    pub fn with_c(self, c: f64) -> Transform {
        self.with_updated_value_at_index(c, (1, 0))
    }

    pub fn with_d(self, d: f64) -> Transform {
        self.with_updated_value_at_index(d, (1, 1))
    }

    pub fn with_e(self, e: f64) -> Transform {
        self.with_updated_value_at_index(e, (2, 0))
    }

    pub fn with_f(self, f: f64) -> Transform {
        self.with_updated_value_at_index(f, (2, 1))
    }

    pub fn with_transpose(self, x: f64, y: f64) -> Transform {
        self.with_e(x).with_f(y)
    }

    pub fn with_scale(self, x_scale: f64, y_scale: f64) -> Transform {
        self.with_a(x_scale).with_d(y_scale)
    }
}
