use std::ops::{AddAssign, Div, Mul};

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::pool::CommodityIndex;

/**
 * Amount
 */

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Amount {
    pub quantity: Decimal,
    pub commodity_index: Option<CommodityIndex>,
}

impl Amount {
    pub fn new(quantity: Decimal, commodity_index: Option<CommodityIndex>) -> Self {
        Self {
            quantity,
            commodity_index,
        }
    }

    pub fn abs(&self) -> Amount {
        if self.quantity.is_sign_positive() {
            let mut clone = self.clone();
            clone.quantity.set_sign_negative(true);
            clone
        } else {
            self.clone()
        }
    }

    /// Creates a new Amount instance.
    /// Parses the quantity only and uses the given commodity index.
    pub fn parse(amount: &str, commodity_index: Option<CommodityIndex>) -> Option<Self> {
        if amount.is_empty() {
            return None;
        }

        let quantity_result = Decimal::from_str_exact(amount);
        if quantity_result.is_err() {
            return None;
        }

        let amount = Self {
            quantity: quantity_result.unwrap(),
            commodity_index,
        };

        Some(amount)
    }

    pub fn copy_from(other: &Amount) -> Self {
        // let com = match &other.commodity {
        //     Some(other_commodity) => {
        //         //let symbol = &other.commodity.as_ref().unwrap().symbol;
        //         let s = &other_commodity.symbol;
        //         let c = Commodity::new(s);
        //         Some(c)
        //     }
        //     None => None,
        // };

        Self {
            quantity: other.quantity,
            commodity_index: other.commodity_index,
        }
    }

    pub fn null() -> Self {
        Self {
            quantity: dec!(0),
            commodity_index: None,
        }
    }

    pub fn add(&mut self, other: &Amount) {
        if self.commodity_index != other.commodity_index {
            log::error!("different commodities");
            panic!("don't know yet how to handle this")
        }
        if other.quantity.is_zero() {
            // nothing to do
            return;
        }

        self.quantity += other.quantity;
    }

    /// Returns an inverse amount.
    /// Normally it is a quantity with the opposite sign.
    pub fn inverse(&self) -> Amount {
        let new_quantity = self.quantity.mul(dec!(-1));
        // let new_commodity = match &self.commodity {
        //     Some(c) => Some(Commodity::new(&c.symbol)),
        //     None => None,
        // };
        Amount::new(new_quantity, self.commodity_index)
    }

    /// Indicates whether the amount is initialized.
    /// This is a 0 quantity and no Commodity.
    pub fn is_null(&self) -> bool {
        if self.quantity.is_zero() {
            return self.commodity_index.is_none();
        } else {
            false
        }
    }

    pub fn is_zero(&self) -> bool {
        self.quantity.is_zero()
    }
}

impl std::ops::Add<Amount> for Amount {
    type Output = Amount;

    fn add(self, rhs: Amount) -> Self::Output {
        if self.commodity_index != rhs.commodity_index {
            panic!("don't know yet how to handle this")
        }

        let sum = self.quantity + rhs.quantity;

        Amount::new(sum, self.commodity_index)
    }
}

impl AddAssign<Amount> for Amount {
    fn add_assign(&mut self, other: Amount) {
        if self.commodity_index != other.commodity_index {
            panic!("don't know yet how to handle this")
        }

        self.quantity += other.quantity;
    }
}

impl Div for Amount {
    type Output = Amount;

    fn div(self, rhs: Self) -> Self::Output {
        if self.quantity.is_zero() || rhs.quantity.is_zero() {
            todo!("handle no quantity");
        }

        let mut result = Amount::new(Decimal::ZERO, None);

        if self.commodity_index.is_none() {
            result.commodity_index = rhs.commodity_index;
        } else {
            result.commodity_index = self.commodity_index
        }

        result.quantity = self.quantity / rhs.quantity;

        result
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::Amount;

    #[test]
    fn test_division() {
        let a = Amount::new(dec!(10), Some(3.into()));
        let b = Amount::new(dec!(5), Some(3.into()));
        let expected = Amount::new(dec!(2), Some(3.into()));

        let c = a / b;

        assert_eq!(expected, c);
    }
}