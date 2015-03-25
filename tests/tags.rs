#![feature(plugin, custom_attribute)]
#![plugin(jit_macros)]
#[no_link] #[macro_use]
extern crate jit_macros;
extern crate jit;
use jit::*;

#[test]
fn test_context_tags() {
    let ctx = Context::new();
    ctx.set_meta(Box::new(3u8));
    assert_eq!(ctx.get_meta(), Some(&3u8));
}

#[derive(Debug, Eq, PartialEq)]
struct PanicDrop(isize);
impl Drop for PanicDrop {
    fn drop(&mut self) {
        panic!("Dropped {:?}", self)
    }
}
#[test]
#[should_panic]
fn test_panic_tags() {
    let pos_t = get::<Pos>();
    let kind = pos_t.get_kind();
    let pos_t = TaggedType::new(&pos_t, kind, Box::new(PanicDrop(42)));
    assert_eq!(pos_t.get_tagged_data(), Some(&PanicDrop(42)));
}
#[jit]
#[repr(packed)]
struct Pos {
    x: f64,
    y: f64
}
#[test]
fn test_tags() {
    let pos_t = get::<Pos>();
    let kind = pos_t.get_kind();
    let new_pos_t = TaggedType::new(&pos_t, kind, Box::new(42));
    assert!(new_pos_t.get_tagged_data() == Some(&42));
    assert!(new_pos_t.get_tagged_type() == &*pos_t);
}
