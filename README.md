# Elliptic Curve (Weierstrass Form)

**A generic elliptic curve library implementing the group law for curves in Weierstrass form** — y² = x³ + ax + b — with point addition, point doubling, and scalar multiplication via double-and-add. This is the mathematical foundation of ECDSA, ECDH, and every modern cryptographic curve (secp256k1, P-256, P-384).

## Why It Matters

Elliptic Curve Cryptography (ECC) secures the internet. Every TLS connection, every Bitcoin transaction, every SSH key uses elliptic curves. The core operation — scalar multiplication `k·G` (multiply a point by a large integer) — is the one-way function that makes ECC secure: computing k from G and k·G is the Elliptic Curve Discrete Logarithm Problem (ECDLP), believed to be intractable.

**Weierstrass form** is the most common elliptic curve representation: `y² = x³ + ax + b`. Famous curves in this form:
- **secp256k1** — Bitcoin, Ethereum: `y² = x³ + 7`
- **P-256 (NIST)** — TLS certificates, Apple iMessage: `y² = x³ − 3x + b`
- **P-384** — Higher-security TLS connections

**The group law:** Points on an elliptic curve form an abelian group under a geometric addition operation. To add points P and Q, draw a line through them; it intersects the curve at a third point, which is reflected across the x-axis to give P + Q. This geometric rule translates into algebraic formulas involving slopes and field arithmetic.

**Scalar multiplication:** Computing `k·P` (adding P to itself k times) is done via **double-and-add** — the elliptic curve equivalent of square-and-multiply. This requires O(log k) point additions/doublings, making it efficient even for 256-bit scalars.

## How It Works

The library is parameterized over a generic field `F` that implements `CurvedField` (requiring Add, Sub, Mul, Div, zero, and from). This allows the same group law to work over any field:

**Point representation:** `CurvePoint<F>` is either `Point(x, y)` or `Identity` (the point at infinity — the group's identity element).

**Point addition (group law):**
- Identity + P = P (identity is the neutral element)
- If x₁ = x₂ and y₁ = −y₂: result is Identity (additive inverse)
- Otherwise: compute slope λ:
  - If P = Q (doubling): λ = (3x² + a) / 2y
  - If P ≠ Q (addition): λ = (y₂ − y₁) / (x₂ − x₁)
- Then: x₃ = λ² − x₁ − x₂, y₃ = λ(x₁ − x₃) − y₁

**Point doubling:** When P = Q, the slope is the derivative dy/dx = (3x² + a) / (2y), which is the tangent line at P.

**Scalar multiplication (double-and-add):** Decompose k into binary bits. For each bit: double the accumulator, and if the bit is 1, add the base point. This is O(log k) — computing k·G for a 256-bit k requires at most 510 group operations (255 doublings + 255 additions).

**Curve validation:** The discriminant 4a³ + 27b² must be non-zero (ensures the curve is non-singular — no self-intersections or cusps).

## Quick Start

```rust
use elliptic_curve_weierstrass::{WeierstrassCurve, CurvePoint, CurvedField};

// Define your field (in production: use a prime field implementation)
// The curve y² = x³ + 7 (secp256k1's curve, simplified)
let curve = WeierstrassCurve::new(a, b);

// Point operations
let p = CurvePoint::Point(x1, y1);
let q = CurvePoint::Point(x2, y2);
let sum = curve.add(&p, &q);

// Scalar multiplication (e.g., for cryptographic operations)
let k = 42;
let result = curve.scalar_mul(&p, k);
```

## API

### `WeierstrassCurve<F>` where `F: CurvedField`
- `new(a: F, b: F) -> Self` — Create curve y² = x³ + ax + b
- `is_on_curve(x: F, y: F) -> bool` — Verify point is on the curve. O(1)
- `add(p: &CurvePoint<F>, q: &CurvePoint<F>) -> CurvePoint<F>` — Group addition. O(1)
- `scalar_mul(point: &CurvePoint<F>, k: u64) -> CurvePoint<F>` — k·P via double-and-add. O(log k)

### `CurvePoint<F>` (enum)
- `Point(F, F)` — An (x, y) coordinate pair
- `Identity` — Point at infinity (group identity)

### `CurvedField` trait
- Requires: `Clone + Debug + PartialEq + Add + Sub + Mul + Div`
- `zero() -> Self` — Additive identity
- `from(u64) -> Self` — Integer to field element

## Architecture Notes

This library provides the mathematical foundation for SuperInstance's cryptographic toolkit. It implements the generic group law that underlies ECC signatures (ECDSA), key exchange (ECDH), and zero-knowledge proofs. Production usage requires a proper prime field implementation with modular arithmetic.

See the full architecture: [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md)

## License

MIT
