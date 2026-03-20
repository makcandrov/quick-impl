use quick_impl::quick_impl;

#[derive(Default)]
#[quick_impl(pub const new, from_tuple, into_parts, impl From, impl Into)]
#[quick_impl_all(pub set, pub with, pub take)]
struct Config {
    #[quick_impl(pub get)]
    host: String,

    #[quick_impl(pub get)]
    port: u16,

    #[quick_impl(pub get_clone, pub replace, pub into, pub from)]
    max_connections: usize,
}

fn main() {
    // `new` — construct with all field values
    let config = Config::new("localhost".to_string(), 8080, 100);
    assert_eq!(config.get_host(), "localhost");
    assert_eq!(*config.get_port(), 8080);

    // `get_clone` — returns a clone instead of a reference
    let max: usize = config.get_max_connections();
    assert_eq!(max, 100);

    // `set` — returns &mut self for chaining
    let mut config = Config::default();
    config
        .set_host("127.0.0.1".to_string())
        .set_port(3000)
        .set_max_connections(50);
    assert_eq!(config.get_host(), "127.0.0.1");

    // `with` — builder-style, consumes and returns self
    let config = Config::default()
        .with_host("localhost".to_string())
        .with_port(8080)
        .with_max_connections(100);
    assert_eq!(config.get_host(), "localhost");

    // `take` — returns the field and resets it to its default
    let mut config = Config::new("localhost".to_string(), 8080, 100);
    let host = config.take_host();
    assert_eq!(host, "localhost");
    assert_eq!(config.get_host(), ""); // reset to default

    // `replace` — replaces the field, returning the previous value
    let mut config = Config::new("localhost".to_string(), 8080, 100);
    let old = config.replace_max_connections(200);
    assert_eq!(old, 100);
    assert_eq!(config.get_max_connections(), 200);

    // `from` (field method) — create from a single field, others default
    let config = Config::from_max_connections(42);
    assert_eq!(config.get_max_connections(), 42);
    assert_eq!(config.get_host(), ""); // default
    assert_eq!(*config.get_port(), 0); // default

    // `into` (field method) — consume self and extract a single field
    let max = config.into_max_connections();
    assert_eq!(max, 42);

    // `from_tuple` / `into_parts` — round-trip through tuples
    let config = Config::from_tuple(("localhost".to_string(), 8080, 100));
    let (host, port, max) = config.into_parts();
    assert_eq!(host, "localhost");
    assert_eq!(port, 8080);
    assert_eq!(max, 100);

    // `impl From` / `impl Into` — standard trait conversion with tuples
    let config = Config::from(("localhost".to_string(), 8080, 100));
    let (host, port, max): (String, u16, usize) = config.into();
    assert_eq!((host.as_str(), port, max), ("localhost", 8080, 100));
}
