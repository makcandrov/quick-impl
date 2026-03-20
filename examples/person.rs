use quick_impl::{quick_impl, quick_impl_all};

#[quick_impl_all(pub get)]
struct Person {
    #[quick_impl(impl AsRef, impl Borrow)]
    name: String,

    #[quick_impl(pub set, pub get_mut, impl AsMut)]
    age: u32,
}

#[quick_impl]
struct PersonRef<'a> {
    #[quick_impl(impl Deref, impl DerefMut)]
    person: &'a mut Person,
}

fn main() {
    // `get` — generated for all fields via `quick_impl_all`
    let mut person = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    assert_eq!(person.get_name(), "Alice");
    assert_eq!(*person.get_age(), 30);

    // `get_mut` — returns a mutable reference
    *person.get_age_mut() += 1;
    assert_eq!(*person.get_age(), 31);

    // `set` — setter that returns &mut self
    person.set_age(40);
    assert_eq!(*person.get_age(), 40);

    // `impl AsRef` — standard trait for cheap reference conversion
    let name: &String = person.as_ref();
    assert_eq!(name, "Alice");

    // `impl Borrow` — standard trait for borrowing
    let name: &String = std::borrow::Borrow::borrow(&person);
    assert_eq!(name, "Alice");

    // `impl AsMut` — standard trait for mutable reference conversion
    let age: &mut u32 = person.as_mut();
    *age = 50;
    assert_eq!(*person.get_age(), 50);

    // `impl Deref` / `impl DerefMut` — transparent access through a wrapper
    let mut person_ref = PersonRef {
        person: &mut person,
    };
    assert_eq!(person_ref.get_name(), "Alice");
    person_ref.set_age(60);
    assert_eq!(*person_ref.get_age(), 60);
}
