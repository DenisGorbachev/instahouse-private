use time::OffsetDateTime;

pub trait Validate {
    /// Should contain all errors encountered during validation
    type Errors;

    // TODO: replace with special Vec
    fn validate(&self, now: OffsetDateTime) -> Self::Errors;
}
