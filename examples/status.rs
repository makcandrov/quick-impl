use quick_impl::quick_impl;

#[derive(Debug)]
#[quick_impl]
pub enum Status {
    #[quick_impl(pub const is)]
    Pending,

    #[quick_impl(pub const is, pub try_into, impl TryInto)]
    Approved(i32),

    #[quick_impl(pub const is, pub try_into, impl TryInto)]
    Rejected(String),
}

#[quick_impl]
pub struct Wrapper {
    #[quick_impl(impl Deref, impl DerefMut)]
    data: Vec<u8>,
}

fn main() {
    // Enum example with `try_into` and `is`
    let status = Status::Approved(200);
    assert!(status.is_approved());

    // Use `try_into` to convert to the associated data
    let approved_code: Result<i32, Status> = status.try_into_approved();
    assert_eq!(approved_code.unwrap(), 200);

    let rejected_status = Status::Rejected("Invalid request".to_string());
    assert!(rejected_status.is_rejected());

    let rejection_reason: Result<String, Status> = rejected_status.try_into();
    assert_eq!(rejection_reason.unwrap(), "Invalid request");

    // Struct example with `Deref` and `DerefMut`
    let mut wrapper = Wrapper {
        data: vec![1, 2, 3],
    };

    // Accessing inner `Vec<u8>` through Deref
    assert_eq!(wrapper.len(), 3); // Calls `Vec<u8>::len`

    // Mutating inner `Vec<u8>` through DerefMut
    wrapper.push(4);
    assert_eq!(wrapper.len(), 4); // Now has 4 elements
}
