use crate::{
    ode_solver::{OdeSolverProblem, OdeSolverSolution},
    DenseMatrix, OdeEquations, Vector,
};
use nalgebra::ComplexField;
use num_traits::Zero;
use std::ops::MulAssign;

// exponential decay problem
// dy/dt = -ay (p = [a])
fn exponential_decay<M: DenseMatrix>(x: &M::V, p: &M::V, _t: M::T, y: &mut M::V) {
    y.copy_from(x);
    y.mul_assign(-p[0]);
}

// Jv = -av
fn exponential_decay_jacobian<M: DenseMatrix>(
    _x: &M::V,
    p: &M::V,
    _t: M::T,
    v: &M::V,
    y: &mut M::V,
) {
    y.copy_from(v);
    y.mul_assign(-p[0]);
}

fn exponential_decay_init<M: DenseMatrix>(_p: &M::V, _t: M::T) -> M::V {
    M::V::from_vec(vec![1.0.into(), 1.0.into()])
}

pub fn exponential_decay_problem<M: DenseMatrix + 'static>() -> (
    OdeSolverProblem<impl OdeEquations<M = M, V = M::V, T = M::T>>,
    OdeSolverSolution<M::V>,
) {
    let p = M::V::from_vec(vec![0.1.into()]);
    let problem = OdeSolverProblem::new_ode(
        exponential_decay::<M>,
        exponential_decay_jacobian::<M>,
        exponential_decay_init::<M>,
        p.clone(),
    );
    let mut soln = OdeSolverSolution::default();
    for i in 0..10 {
        let t = M::T::from(i as f64 / 10.0);
        let y0: M::V = problem.eqn.init(M::T::zero());
        let y = y0 * M::T::exp(-p[0] * t);
        soln.push(y, t);
    }
    (problem, soln)
}
