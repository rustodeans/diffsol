use crate::matrix::Matrix;
use num_traits::{One, Zero};

use super::{LinearOp, Op};

pub struct MatrixOp<M: Matrix> {
    m: M,
}

impl<M: Matrix> MatrixOp<M> {
    pub fn new(m: M) -> Self {
        Self { m }
    }
}

impl<M: Matrix> Op for MatrixOp<M> {
    type V = M::V;
    type T = M::T;
    type M = M;
    fn nstates(&self) -> usize {
        self.m.nrows()
    }
    fn nout(&self) -> usize {
        self.m.ncols()
    }
    fn nparams(&self) -> usize {
        0
    }
}

impl<M: Matrix> LinearOp for MatrixOp<M> {
    fn call_inplace(&self, x: &Self::V, _t: Self::T, y: &mut Self::V) {
        self.m.gemv(M::T::one(), x, M::T::zero(), y);
    }
    fn jacobian(&self, _t: Self::T) -> Self::M {
        self.m.clone()
    }
}