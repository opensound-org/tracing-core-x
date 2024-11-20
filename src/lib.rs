#![cfg_attr(nightly, feature(doc_auto_cfg))]

use serde::{Deserialize, Serialize};
use std::num::NonZeroU64;
use tracing_core::span::Id;

/// [Id](https://docs.rs/tracing-core/latest/tracing_core/span/struct.Id.html) with: Serialize + Deserialize + Copy
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct IdX(NonZeroU64);

impl From<Id> for IdX {
    fn from(value: Id) -> Self {
        Self(value.into_non_zero_u64())
    }
}

impl From<&Id> for IdX {
    fn from(value: &Id) -> Self {
        Self(value.into_non_zero_u64())
    }
}

impl From<IdX> for Id {
    fn from(value: IdX) -> Self {
        Self::from_non_zero_u64(value.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_id() -> Id {
        Id::from_non_zero_u64(rand::random())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn idx() {
        let id = gen_id();
        let idx: IdX = (&id).into();
        let de: IdX = serde_json::from_str(
            &tokio::spawn(async move { serde_json::to_string(&idx).unwrap() })
                .await
                .unwrap(),
        )
        .unwrap();

        assert_eq!(id, de.into());
    }
}
