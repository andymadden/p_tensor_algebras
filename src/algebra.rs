use std::ops::{Add, AddAssign, Index, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Algebra<'v, T: Copy + Mul> {
    pub dim: usize,
    pub p_tensor: &'v Vec<Vec<Vec<T>>>,
}

#[derive(Debug, Clone)]
pub struct AlgebraicVector<'v, T: Copy + Mul> {
    pub algebra: Algebra<'v, T>,
    pub data: Vec<T>,
}

impl<'v, T: Copy + Mul> Algebra<'v, T>
    where T: Copy + Mul
{
    pub fn create_vector(self, data: Vec<T>) -> AlgebraicVector<'v, T> {
        AlgebraicVector {algebra: self, data }
    }

}

impl<'v, T> Mul for AlgebraicVector<'v, T>
    where T: Copy + Mul<Output = T> + AddAssign + Sub<Output = T>
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = vec![];

        for i in 0..self.algebra.dim {
            let mut sum: T = self.data[0] - self.data[0];
            for j in 0..self.algebra.dim {
                for k in 0..self.algebra.dim {
                    sum += self.data[j]*rhs.data[k]*self.algebra.p_tensor[i][j][k];
                }
            }
            result.push(sum);
        }
        self.algebra.create_vector(result)
    }
}

impl<'v, T> Index<usize> for AlgebraicVector<'v, T>
    where T: Copy + Mul + Add
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<'v, T> Add for AlgebraicVector<'v, T>
    where T: Copy + Clone + Mul + Add<Output=T>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut data = vec![];

        for i in 0..self.algebra.dim {
            data.push(self[i] + rhs[i]);
        }

        self.algebra.create_vector(data)
    }
}

impl<'v, T> AddAssign for AlgebraicVector<'v, T>
    where T: Copy + Clone + Mul + AddAssign
{
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..self.algebra.dim {
            self.data[i] += rhs[i];
        }
    }
}