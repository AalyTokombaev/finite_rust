use super::field::Field;
use super::field_element::FieldElement;
use super::gf2::GF2;

/* 
    let's think ..
    consider the field GF(2^3)
    with polynomial x^3 + x + 1

    now a function F would be:
        F(x) = sum_{0 <= i < j < n} a_{i,j}^{2^i + 2^j}
        or written out:

        F(x) = \sum_{0 <= i, j < 3} a_{i,j}^{2^i + 2^j}

        F(x) = 
            a_{0, 1}x^{2^0 + 2^1} +
            a_{0, 2}x^{2^0 + 2^2} +
            a_{1, 2}x^{2^1 + 2^2} +

        let's pick som random a's from GF(2^3):
            a_{0, 1} = 1
            a_{0, 2} = 0
            a_{1, 2} = 1
*/


pub struct QuadraticFunction {
    pub coefficients = Vec<FieldElement>,
}


