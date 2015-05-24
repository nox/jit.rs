extern crate jit;
#[no_link] #[macro_use]
extern crate jit_macros;
use jit::*;

#[test]
fn test_sqrt() {
    let mut ctx = Context::<()>::new();
    assert_eq!(ctx.functions().count(), 0);
    jit_func!(&mut ctx, func, fn(num: usize) -> usize {
        let sqrt = func.insn_sqrt(num);
        let sqrt_arg_ui = func.insn_convert(sqrt, &get::<usize>(), false);
        func.insn_return(sqrt_arg_ui);
    }, {
        let sqrt = func;
        assert_eq!(sqrt(64), 8);
        assert_eq!(sqrt(16), 4);
        assert_eq!(sqrt(9), 3);
        assert_eq!(sqrt(4), 2);
        assert_eq!(sqrt(1), 1);
    });
    assert_eq!(ctx.functions().count(), 1);
}
