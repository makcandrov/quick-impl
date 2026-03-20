use quick_impl::quick_impl_all;

#[quick_impl_all(pub const is)]
pub enum WebEvent {
    PageLoad,
    PageUnload,

    #[quick_impl(pub as_ref, pub(crate) as_ref_mut, impl From)]
    KeyPress(char),

    #[quick_impl(pub is_and, pub as_ref, pub into)]
    Paste(String),

    #[quick_impl(pub from = "click_from_coordinates", pub as_ref)]
    Click {
        x: i64,
        y: i64,
    },
}

fn main() {
    // `quick_impl_all` generates `is_*` for every variant
    let page_load = WebEvent::PageLoad;
    assert!(page_load.is_page_load());
    assert!(!page_load.is_page_unload());

    // `impl From` — construct from associated data
    let key_press = WebEvent::from('c');
    assert!(key_press.is_key_press());
    assert_eq!(*key_press.as_key_press().unwrap(), 'c');

    // `is_and` — check variant and test a predicate on the data
    let paste = WebEvent::Paste("hello world".to_owned());
    assert!(paste.is_paste_and(|value| !value.is_empty()));
    assert!(!paste.is_paste_and(|value| value.is_empty()));

    // `into` — extract associated data
    assert_eq!(paste.into_paste().unwrap(), "hello world");

    // `from` with a custom method name
    let click = WebEvent::click_from_coordinates(-10, 10);
    assert!(click.is_click());
    assert_eq!(click.as_click().unwrap(), (&-10, &10));
}
