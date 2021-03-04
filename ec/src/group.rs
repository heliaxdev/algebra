use core::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};
use num_traits::Zero;

use ark_ff::{
    bytes::{FromBytes, ToBytes},
    fields::PrimeField,
    UniformRand,
};

use zkcrypto_group::Group as zkcryptoGroup;
use subtle::{Choice};
use rand_core::RngCore;

use core::iter::Sum;

#[derive (Copy,Debug,Clone, Eq, PartialEq)]
struct ArkworksGroup<G: Group>(G);

impl<G: Group> Add for ArkworksGroup<G>{
    type Output = ArkworksGroup<G>;
    fn add(self, other: Self) -> Self {
        ArkworksGroup(self.0 + other.0)
    }
}

impl<'a, G: Group> Add<&'a ArkworksGroup<G>> for ArkworksGroup<G>{
    type Output = ArkworksGroup<G>;
    fn add(self, other: &Self) -> Self {
        ArkworksGroup(self.0 + other.0)
    }
}

impl<G: Group> AddAssign for ArkworksGroup<G>{
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl<'a, G: Group> AddAssign<&'a ArkworksGroup<G>> for ArkworksGroup<G>{
    fn add_assign(&mut self, other: &Self) {
        self.0 += other.0;
    }
}

impl<G: Group> Sub for ArkworksGroup<G>{
    type Output = ArkworksGroup<G>;
    fn sub(self, other: Self) -> Self {
        ArkworksGroup(self.0 - other.0)
    }
}

impl<'a, G: Group> Sub<&'a ArkworksGroup<G>> for ArkworksGroup<G>{
    type Output = ArkworksGroup<G>;
    fn sub(self, other: &Self) -> Self {
        ArkworksGroup(self.0 - other.0)
    }
}

impl<G: Group> SubAssign for ArkworksGroup<G>{
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

impl<'a, G: Group> SubAssign<&'a ArkworksGroup<G>> for ArkworksGroup<G>{
    fn sub_assign(&mut self, other: &Self) {
        self.0 -= other.0;
    }
}

impl<G: Group> Neg for ArkworksGroup<G>{
    type Output = ArkworksGroup<G>;
    fn neg(self) -> Self {
        ArkworksGroup(-self.0)
    }
}

impl<'a, G: Group> Sum<&'a ArkworksGroup<G>> for ArkworksGroup<G>{
    fn sum<I: Iterator>(iter: I) -> Self {
        ArkworksGroup(iter.map(|x| x.0).sum())
    }
}

impl<G: Group> Sum for ArkworksGroup<G>{
    fn sum<I: Iterator<Item=ArkworksGroup<G>>>(iter: I) -> Self {
        ArkworksGroup(iter.map(|x| x.0).sum())
    }
}

impl<'a, G: Group> Mul<&'a ArkworksGroup<G>> for ArkworksGroup<G> {
    type Output = Self;

    fn mul(self, other: &ArkworksGroup<G>) -> Self {
        // TODO
        ArkworksGroup(self.0*other.0)
    }
}

impl<'a, G: Group> Mul<ArkworksGroup<G>> for ArkworksGroup<G> {
    type Output = Self;

    fn mul(self, other: ArkworksGroup<G>) -> Self {
        // TODO
        ArkworksGroup(self.0*other.0)
    }
}

// impl<'a, G: Group> MulAssign<&'a ArkworksGroup<G>> for ArkworksGroup<G> {
//     fn mul_assign(&mut self, other: &ArkworksGroup<G>) {
//         self.0 *= other.0;
//     }
// }

// impl<G: Group> MulAssign<ArkworksGroup<G>> for ArkworksGroup<G> {
//     fn mul_assign(&mut self, other: ArkworksGroup<G>) {
//         self.0 *= other.0;
//     }
// }

impl<'a, G: Group> MulAssign<&'a G::ScalarField> for ArkworksGroup<G> {
    fn mul_assign(&mut self, other: &ArkworksGroup<G>) {
        self.0 *= other.0;
    }
}

impl<G: Group> MulAssign<G::ScalarField> for ArkworksGroup<G> {
    fn mul_assign(&mut self, other: ArkworksGroup<G>) {
        self.0 *= other.0;
    }
}




// TODO: From, Into
impl<G: Group> zkcryptoGroup for ArkworksGroup<G> {
    // type Scalar = G; // G::ScalarField;
    type Scalar = G::ScalarField;

    fn random(rng: impl RngCore) -> Self {
    }

    // Returns the additive identity, also known as the "neutral element".
    fn identity() -> Self {

    }

    // Returns a fixed generator of the prime-order subgroup.
    fn generator() -> Self {

    }

    // Determines if this point is the identity.
    fn is_identity(&self) -> Choice {

    }

    // Doubles this element.
    fn double(&self) -> Self {
        ArkworksGroup(self.0)

    }
}

pub trait Group:
    ToBytes
    + 'static
    + FromBytes
    + Copy
    + Clone
    + Debug
    + Display
    + Default
    + Send
    + Sync
    + Eq
    + Hash
    + Neg<Output = Self>
    + UniformRand
    + Zero
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + AddAssign<Self>
    + SubAssign<Self>
    + MulAssign<<Self as Group>::ScalarField>
    + for<'a> Add<&'a Self, Output = Self>
    + for<'a> Sub<&'a Self, Output = Self>
    + for<'a> AddAssign<&'a Self>
    + for<'a> SubAssign<&'a Self>
    + core::iter::Sum<Self>
    + for<'a> core::iter::Sum<&'a Self>
{
    type ScalarField: PrimeField + Into<<Self::ScalarField as PrimeField>::BigInt>;

    /// Returns `self + self`.
    #[must_use]
    fn double(&self) -> Self;

    /// Sets `self := self + self`.
    fn double_in_place(&mut self) -> &mut Self;

    #[must_use]
    fn mul<'a>(&self, other: &'a Self::ScalarField) -> Self {
        let mut copy = *self;
        copy *= *other;
        copy
    }
}
