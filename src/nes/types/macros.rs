#[macro_export]
macro_rules! math_type {
    ($name:ident, $internal_type:ty) => {
        #[derive(
            Default,
            Clone,
            Copy,
            Debug,
            derive_more::Display,
            derive_more::Add,
            derive_more::Sub,
            derive_more::BitAnd,
            derive_more::BitOr,
            derive_more::BitXor,
            derive_more::Mul,
            derive_more::Div,
            derive_more::Rem,
            derive_more::Shr,
            derive_more::Shl,
            derive_more::Not,
            derive_more::AddAssign,
            derive_more::SubAssign,
            derive_more::BitAndAssign,
            derive_more::BitOrAssign,
            derive_more::BitXorAssign,
            derive_more::MulAssign,
            derive_more::DivAssign,
            derive_more::RemAssign,
            derive_more::ShrAssign,
            derive_more::ShlAssign,
            derive_more::From,
            derive_more::FromStr,
            derive_more::Into,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
        )]
        pub struct $name(pub $internal_type);

        impl std::fmt::UpperHex for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let val = self.0;
                std::fmt::UpperHex::fmt(&val, f)
            }
        }

        impl std::fmt::LowerHex for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let val = self.0;
                std::fmt::LowerHex::fmt(&val, f)
            }
        }

        impl $name {
            pub fn inc(&mut self) -> Self {
                self.0 = self.0.overflowing_add(1).0;
                *self
            }

            pub fn dec(&mut self) -> Self {
                self.0 = self.0.overflowing_sub(1).0;
                *self
            }

            pub fn overflowing_add(&mut self, op: $name) -> Self {
                self.0 = self.0.overflowing_add(op.0).0;
                *self
            }

            pub fn overflowing_sub(&mut self, op: $name) -> Self {
                self.0 = self.0.overflowing_sub(op.0).0;
                *self
            }
        }
    };
}
