use super::AggregateOrCreate;
use super::Asks;
use super::PriceAndQuantity;
use super::Update;
use serde::{Deserialize, Deserializer as DeserializerT};
use std::fmt::Display;
use std::ops::Add;
use std::str::FromStr;

/// Naive deserialization, consider implementing a visitor to stream json values instead of deseralizing into a vec and draining into the output type.
/// It uses the [AggregateOrCreate](super::AggregateOrCreate) strategy to fill the vec.
impl<'de, P, Q> Deserialize<'de> for Asks<P, Q>
where
    P: Deserialize<'de> + PartialOrd + FromStr,
    P::Err: Display,
    Q::Err: Display,
    Q: Deserialize<'de> + Add<Output = Q> + FromStr + Default + PartialEq,
    Q: Add<Q, Output = Q> + Copy,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: DeserializerT<'de>,
    {
        let mut prices: Vec<PriceAndQuantity<P, Q>> = Deserialize::deserialize(deserializer)?;
        let mut asks = Asks::new();
        for i in prices.drain(..) {
            Update::<AggregateOrCreate>::process(&mut asks, i)
        }
        Ok(asks)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let asks: Asks<f64, i32> = vec![PriceAndQuantity(1., 2)].into();
        let expected: Asks<f64, i32> = serde_json::from_str(r#"[["1.0","2"]]"#).unwrap();
        assert_eq!(asks, expected);
    }
}
