//! Structure for uppercase AZ property : Capital, Small, Decoration

/// Enumeration for uppercase AZ property
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Property {
    NotAnUppercase,
    Capital,
    Small,
    SmallAndCapital,
    CapitalWithDecoration,
    SmallWithDecoration,
    SmallAndCapitalWithDecoration,
}

impl Property {
    #[must_use]
    pub const fn is_capital(self) -> bool {
        matches!(
            self,
            Self::Capital
                | Self::SmallAndCapital
                | Self::CapitalWithDecoration
                | Self::SmallAndCapitalWithDecoration
        )
    }

    #[must_use]
    pub const fn is_small(self) -> bool {
        matches!(
            self,
            Self::Small
                | Self::SmallAndCapital
                | Self::SmallWithDecoration
                | Self::SmallAndCapitalWithDecoration
        )
    }

    #[must_use]
    pub const fn is_decoration(self) -> bool {
        matches!(
            self,
            Self::CapitalWithDecoration
                | Self::SmallWithDecoration
                | Self::SmallAndCapitalWithDecoration
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_capital() {
        assert!(Property::Capital.is_capital());
        assert!(!Property::Small.is_capital());
        assert!(!Property::NotAnUppercase.is_capital());
    }

    #[test]
    fn test_is_small() {
        assert!(Property::Small.is_small());
        assert!(!Property::Capital.is_small());
        assert!(!Property::NotAnUppercase.is_small());
    }

    #[test]
    fn test_is_decoration() {
        assert!(Property::CapitalWithDecoration.is_decoration());
        assert!(Property::SmallWithDecoration.is_decoration());
        assert!(!Property::Capital.is_decoration());
        assert!(!Property::Small.is_decoration());
        assert!(!Property::NotAnUppercase.is_decoration());
    }
}
