use std::ops::Mul;
// TODO: Import necessary libraries. Check cargo.toml and the documentation of the libraries.
use ark_bls12_381::{Fq, FQ_ONE};
use ndarray::{Array2, Array1};
use rand::random;

struct Freivald {
    x: Array1<Fq>, // Array/Vec of Fq,
}

impl Freivald {
    fn new(array_size: usize) -> Self {
        // Generate random number
        let r = random::<Fq>();
        // Populate vector with values r^i for i=0..matrix_size
        let mut x: Vec<Fq> = Vec::with_capacity(array_size);
        let mut vec_val: Fq = FQ_ONE;
        for _i in 0..array_size {
            x.push(vec_val);
            vec_val = vec_val.mul(&r);
        }
        let xarr = Array1::from(x);
        // Return freivald value with this vector as its x value
        Freivald { x: xarr }
    }

    fn verify(&self, matrix_a: &Array2<Fq>, matrix_b: &Array2<Fq>, supposed_ab: &Array2<Fq>) -> bool {
        assert!(check_matrix_dimensions(&matrix_a, &matrix_b, &supposed_ab));
        // check if a * b * x == c * x. Check algorithm to make sure order of operations are
        // correct
        let bx = matrix_b.dot(&self.x);
        let abx = matrix_a.dot(&bx);
        let cx = supposed_ab.dot(&self.x);
        let abx_vec = abx as Array1<Fq>;
        let cx_vec  = cx as Array1<Fq>;
        return abx_vec.eq(&cx_vec);
    }

    // utility function to not have to instantiate Freivalds if you just want to make one
    // verification.
    fn verify_once(matrix_a: &Array2<Fq>, matrix_b: &Array2<Fq>, supposed_ab: &Array2<Fq>) -> bool {
        let freivald = Freivald::new(supposed_ab.nrows());
        return freivald.verify(matrix_a, matrix_b, supposed_ab);
    }
}

// TODO: [Bonus] Modify code to increase your certainty that A * B == C by iterating over the protocol.
// Note that you need to generate new vectors for new iterations or you'll be recomputing same
// value over and over. No problem in changing data structures used by the algorithm (currently its a struct
// but that can change if you want to)
pub fn verify_n(matrix_a:&Array2<Fq>, matrix_b:&Array2<Fq>, supposed_ab:&Array2<Fq>, n:u32) -> bool {
    let mut success= true;
    for _i in 0..n {
        success = Freivald::verify_once(&matrix_a, &matrix_b, &supposed_ab);
        if !success {
            break;
        }
    }
    success
}


// You can either do a test on main or just remove main function and rename this file to lib.rs to remove the
// warning of not having a main implementation
fn main() {
    todo!()
}

// TODO: Add proper types to input matrices. Remember matrices should hold Fq values
pub fn check_matrix_dimensions(matrix_a: &Array2<Fq>, matrix_b: &Array2<Fq>, supposed_ab: &Array2<Fq>) -> bool {
    // If it doesn't you know its not the correct result independently of matrix contents
    let arows = matrix_a.nrows();
    let acols = matrix_a.ncols();

    let brows = matrix_b.nrows();
    let bcols = matrix_b.ncols();

    let abrows = supposed_ab.nrows();
    let abcols = supposed_ab.ncols();

    return acols == brows && bcols == abcols && arows == abrows;
}

#[cfg(test)]
mod tests {
    // #[macro_use]
    use lazy_static::lazy_static;
    use rstest::rstest;
    use ndarray::arr2;

    use super::*;

    lazy_static! {
        static ref MATRIX_A:Array2<Fq> = arr2(&[[Fq::from(2), Fq::from(400), Fq::from(50)],
                                                [Fq::from(123), Fq::from(6543), Fq::from(1)],
                                                [Fq::from(2123), Fq::from(23), Fq::from(231)]]);
        static ref MATRIX_A_DOT_A: Array2<Fq> = matrix_square(&MATRIX_A);
        static ref MATRIX_B: Array2<Fq> = arr2(&[[Fq::from(4), Fq::from(1), Fq::from(1)],
                                                [Fq::from(1), Fq::from(4), Fq::from(1)],
                                                [Fq::from(1), Fq::from(45), Fq::from(3)]]);
        static ref MATRIX_B_DOT_B: Array2<Fq> = matrix_square(&MATRIX_B);
        static ref MATRIX_C: Array2<Fq> = create_matrix(200);
        static ref MATRIX_C_DOT_C: Array2<Fq> = matrix_square(&MATRIX_C);
    }

    pub fn matrix_square(matrix: &Array2::<Fq>) -> Array2::<Fq> {
        matrix.dot(matrix)
    }

    pub fn create_matrix(size: usize) -> Array2<Fq> {
        Array2::ones((size,size))
    }

    #[rstest]
    #[case(& MATRIX_A, & MATRIX_A, & MATRIX_A_DOT_A)]
    #[case(& MATRIX_B, & MATRIX_B, & MATRIX_B_DOT_B)]
    #[case(& MATRIX_C, & MATRIX_C, & MATRIX_C_DOT_C)]
    fn freivald_verify_success_test(
        #[case] matrix_a: &Array2<Fq>,
        #[case] matrix_b: &Array2<Fq>,
        #[case] supposed_ab: &Array2<Fq>,
    ) {
        let freivald = Freivald::new(supposed_ab.nrows());
        assert!(freivald.verify(matrix_a, matrix_b, supposed_ab));
        assert!(verify_n(matrix_a, matrix_b, supposed_ab, 5));
    }

    #[rstest]
    #[case(& MATRIX_A, & MATRIX_B, & MATRIX_A_DOT_A)]
    #[case(& MATRIX_B, & MATRIX_A, & MATRIX_B_DOT_B)]
    #[case(& MATRIX_C, & MATRIX_B, & MATRIX_C_DOT_C)]
    fn freivald_verify_fail_test(
        #[case] a: &Array2<Fq>,
        #[case] b: &Array2<Fq>,
        #[case] c: &Array2<Fq>,
    ) {
        let freivald = Freivald::new(c.nrows());
        assert!(!freivald.verify(a, b, c));
    }
}
