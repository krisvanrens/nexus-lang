use core::fmt;

/// Immovable pointer type, able to take DSTs.
///
/// The idea for this was taken from the `P<T>` "frozen" AST pointer type in the Rust compiler.
#[derive(Debug)]
pub struct Ptr<T: ?Sized> {
    ptr: Box<T>,
}

impl<T> fmt::Display for Ptr<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ptr -> {}", self.ptr)
    }
}

impl<T: 'static> Ptr<T> {
    /// Create a new pointer from a value.
    ///
    /// # Example
    ///
    /// ```
    /// use nexus_rs::ptr::Ptr;
    ///
    /// let x = 42;
    /// let p = Ptr::new(x);
    /// ```
    pub fn new(value: T) -> Self {
        Self {
            ptr: Box::new(value),
        }
    }

    /// Get inner value held by the pointer.
    ///
    /// # Example
    ///
    /// ```
    /// use nexus_rs::ptr::Ptr;
    ///
    /// let p = Ptr::new(42);
    /// assert_eq!(p.into_inner(), 42);
    /// // Pointer 'p' is moved-from now..
    /// ```
    pub fn into_inner(self) -> T {
        *self.ptr
    }
}

#[test]
fn test_new() {
    let p0 = Ptr::new(42);
    assert_eq!(p0.into_inner(), 42);

    let x = 42;
    let p1 = Ptr::new(x);
    assert_eq!(p1.into_inner(), 42);
}
