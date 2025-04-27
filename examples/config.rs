use quick_impl::quick_impl;

#[derive(Default)]
#[quick_impl(pub const new)]
#[quick_impl_all(pub get, pub set, take, pub with)]
struct Config {
    host: String,
    port: u16,

    #[quick_impl(replace)]
    max_connections: usize,
}

fn main() {
    // Using the `Default` trait to create a Config instance
    let mut config = Config::default();

    // Setting values
    config.set_host("localhost".to_string());
    config.set_port(8080);
    config.set_max_connections(100);

    assert_eq!(config.get_host(), "localhost");
    assert_eq!(*config.get_port(), 8080);
    assert_eq!(*config.get_max_connections(), 100);

    // Using `take` to retrieve and reset fields
    let host = config.take_host();
    assert_eq!(host, "localhost");
    assert_eq!(config.get_host(), ""); // After `take`, it should be default (empty String)

    let port = config.take_port();
    assert_eq!(port, 8080);
    assert_eq!(*config.get_port(), 0); // After `take`, it should be default (0)

    // Using `with` to modify fields fluently
    let new_config = config.with_host("127.0.0.1".to_string()).with_port(3000);
    assert_eq!(new_config.get_host(), "127.0.0.1");
    assert_eq!(*new_config.get_port(), 3000);
}
