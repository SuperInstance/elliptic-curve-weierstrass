//! Elliptic curve operations over Weierstrass form: y² = x³ + ax + b

use std::fmt;

/// A point on an elliptic curve in Weierstrass form.
#[derive(Debug, Clone, PartialEq)]
pub enum CurvePoint<F> {
    Point(F, F),
    Identity,
}

/// Parameters defining a Weierstrass curve y² = x³ + ax + b over field F.
#[derive(Debug, Clone)]
pub struct WeierstrassCurve<F> {
    pub a: F,
    pub b: F,
    _marker: std::marker::PhantomData<F>,
}

impl<F: CurvedField> WeierstrassCurve<F> {
    pub fn new(a: F, b: F) -> Self {
        // Discriminant 4a³ + 27b² must be non-zero for non-singular curve
        Self { a, b, _marker: std::marker::PhantomData }
    }

    /// Check if a point lies on the curve.
    pub fn is_on_curve(&self, x: F, y: F) -> bool {
        let lhs = y.clone() * y.clone();
        let rhs = x.clone() * x.clone() * x.clone()
            + self.a.clone() * x
            + self.b.clone();
        lhs == rhs
    }

    /// Point addition (group law).
    pub fn add(&self, p: &CurvePoint<F>, q: &CurvePoint<F>) -> CurvePoint<F> {
        match (p, q) {
            (CurvePoint::Identity, _) => q.clone(),
            (_, CurvePoint::Identity) => p.clone(),
            (CurvePoint::Point(x1, y1), CurvePoint::Point(x2, y2)) => {
                if x1 == x2 && y1.clone() + y1.clone() == F::zero() {
                    return CurvePoint::Identity;
                }
                let slope = if x1 == x2 && y1 == y2 {
                    (F::from(3) * x1.clone() * x1.clone() + self.a.clone())
                        / (F::from(2) * y1.clone())
                } else {
                    (y2.clone() - y1.clone()) / (x2.clone() - x1.clone())
                };
                let x3 = slope.clone() * slope.clone() - x1.clone() - x2.clone();
                let y3 = slope * (x1.clone() - x3.clone()) - y1.clone();
                CurvePoint::Point(x3, y3)
            }
        }
    }

    /// Scalar multiplication via double-and-add.
    pub fn scalar_mul(&self, point: &CurvePoint<F>, mut k: u64) -> CurvePoint<F> {
        let mut result = CurvePoint::Identity;
        let mut addend = point.clone();
        while k > 0 {
            if k & 1 == 1 {
                result = self.add(&result, &addend);
            }
            addend = self.add(&addend, &addend);
            k >>= 1;
        }
        result
    }
}

/// Field trait required for elliptic curve arithmetic.
pub trait CurvedField:
    Clone + fmt::Debug + PartialEq + Sized
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
{
    fn zero() -> Self;
    fn from(val: u64) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq)]
    struct Fp(u64);

    impl CurvedField for Fp {
        fn zero() -> Self { Fp(0) }
        fn from(val: u64) -> Self { Fp(val) }
    }
    // Minimal trait impl for compilation check
    impl std::ops::Add for Fp {
        type Output = Self;
        fn add(self, rhs: Self) -> Self { Fp(self.0 + rhs.0) }
    }
    impl std::ops::Sub for Fp {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self { Fp(self.0 - rhs.0) }
    }
    impl std::ops::Mul for Fp {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self { Fp(self.0 * rhs.0) }
    }
    impl std::ops::Div for Fp {
        type Output = Self;
        fn div(self, rhs: Self) -> Self { Fp(self.0 / rhs.0) }
    }
}
