use quick_impl::quick_impl_all;

#[derive(Debug)]
#[quick_impl_all(pub const is)]
pub enum Status {
    #[quick_impl(impl Default)]
    Pending,

    #[quick_impl(pub try_into, impl TryInto)]
    Approved(i32),

    #[quick_impl(pub try_into, impl TryFrom)]
    Rejected(String),
}

fn main() {
    // `impl Default` — defaults to the specified variant
    let status = Status::default();
    assert!(status.is_pending());

    // `try_into` method — fallible extraction returning Result
    let status = Status::Approved(200);
    let code: Result<i32, Status> = status.try_into_approved();
    assert_eq!(code.unwrap(), 200);

    // `try_into` on the wrong variant returns Err(self)
    let status = Status::Pending;
    let result: Result<i32, Status> = status.try_into_approved();
    assert!(result.is_err());

    // `impl TryInto` trait — same behavior via the standard trait
    let status = Status::Approved(200);
    let code: Result<i32, Status> = status.try_into();
    assert_eq!(code.unwrap(), 200);

    // `impl TryFrom` trait — extract from the other direction
    let status = Status::Rejected("Invalid request".to_string());
    let reason = String::try_from(status);
    assert_eq!(reason.unwrap(), "Invalid request");
}
