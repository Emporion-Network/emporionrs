# Reviews contract for Emporion

This contract handles reveiews
- Reviews are for an **Address** or an **Id**
- rating is a u8 between 0 and 5
- the contract is instantiated with a validator address
- the validator contract should accept a query message `CheckReview` that accepts a
```rust
pub struct QueryCheckReview {
    pub reviewer: Addr,
    pub reviewed: UncheckedReviewed,
}

pub enum UncheckedReviewed {
    Addr(String),
    Id(u64),
}

```

and returns a

```rust

pub struct CheckReviewResp {
    pub resp: bool,
}

```
