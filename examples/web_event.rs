use quick_impl::quick_impl;

#[quick_impl]
pub enum WebEvent {
    #[quick_impl(pub const is)]
    PageLoad,

    #[quick_impl(pub const is)]
    PageUnload,

    #[quick_impl(pub as_ref, pub(crate) as_ref_mut, impl From)]
    KeyPress(char),

    #[quick_impl(pub is_and, pub as_ref, pub(crate) as_ref_mut, pub into)]
    Paste(String),

    #[quick_impl(pub from = "click_from_coordinates", pub const is, pub as_ref)]
    Click { x: i64, y: i64 },
}

fn main() {
    let page_load = WebEvent::PageLoad;
    assert!(page_load.is_page_load());

    let page_unload = WebEvent::PageUnload;
    assert!(page_unload.is_page_unload());

    let mut key_press = WebEvent::from('c');
    assert_eq!(*key_press.as_key_press().unwrap(), 'c');

    *key_press.as_key_press_mut().unwrap() = 'd';
    assert_eq!(*key_press.as_key_press().unwrap(), 'd');

    let paste = WebEvent::Paste("hello world".to_owned());
    assert_eq!(paste.as_paste().unwrap(), "hello world");
    assert!(paste.is_paste_and(|value| !value.is_empty()));
    assert_eq!(paste.into_paste().unwrap(), "hello world".to_owned());

    let click = WebEvent::click_from_coordinates(-10, 10);
    assert!(click.is_click());
    let WebEvent::Click { x, y } = click else { panic!() };
    assert_eq!(x, -10);
    assert_eq!(y, 10);
}
