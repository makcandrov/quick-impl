use quick_impl::{quick_impl, quick_impl_all};

#[quick_impl_all(pub get)]
struct Person {
    name: String,

    #[quick_impl(pub set, get_mut)]
    age: u32,
}

// Define a struct PersonRef that holds a reference to a Person
#[quick_impl]
struct PersonRef<'a> {
    #[quick_impl(impl Deref, pub(crate) get)]
    person: &'a Person,
}

fn main() {
    // Create a new person instance
    let mut person = Person { name: "Alice".to_string(), age: 30 };

    assert_eq!(person.get_name(), "Alice");
    assert_eq!(*person.get_age(), 30);

    *person.get_age_mut() += 1;

    assert_eq!(*person.get_age(), 31);

    person.set_age(40);

    assert_eq!(*person.get_age(), 40);

    let person_ref = PersonRef { person: &person };

    assert_eq!(person_ref.get_name(), person.get_name());
    assert_eq!(person_ref.get_age(), person.get_age());
}
