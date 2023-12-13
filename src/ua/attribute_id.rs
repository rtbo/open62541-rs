use open62541_sys::UA_AttributeId_UA_ATTRIBUTEID_VALUE;

/// Wrapper for attribute IDs from [`open62541_sys`].
#[derive(Clone, Copy, Debug)]
pub struct AttributeId(u32);

impl AttributeId {
    #[must_use]
    pub const fn value() -> Self {
        Self(UA_AttributeId_UA_ATTRIBUTEID_VALUE)
    }

    /// Creates wrapper by taking ownership of `src`.
    #[allow(dead_code)]
    #[must_use]
    pub(crate) const fn new(src: u32) -> Self {
        Self(src)
    }

    /// Gives up ownership and returns inner value.
    #[must_use]
    pub(crate) const fn into_inner(self) -> u32 {
        self.0
    }
}
