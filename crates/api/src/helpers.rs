use bigdecimal::BigDecimal;
use serde::Serializer;

pub fn round_to_pence<S>(value: &Option<BigDecimal>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(v) => s.serialize_str(&format!("{:.2}", v)),
        None => s.serialize_none(),
    }
}
