use syn::{
    Arm, Block, Expr, FieldValue, Ident, Item, Local, Stmt, Type, parse_quote, parse_quote_spanned,
    spanned::Spanned,
};

const MACRO_ERROR: &str = r"
Macro invocations are not allowed inside of the main generator code. Please do one of the following:
- extract it into a function
- extract it into a closure
- move it out of the generator
";
const AWAIT_ERROR: &str = "
Await outside of async context.
";
const ATTR_ERROR: &str = "
Attributes are FORBIDDEN inside of the generator code, as they can expand into unintended code.
";

macro_rules! assert_no_attr {
    ($e:expr, $self:ident) => {
        if !$e.attrs.is_empty() {
            *$self = parse_quote_spanned!($e.span() => ::core::compile_error!(#ATTR_ERROR));
            return;
        }
    };
}

pub trait Trans: Sized {
    fn trans(&mut self, _out: &Type, _is_async: bool) {}
}

impl<T: Trans> Trans for Option<T> {
    fn trans(&mut self, out: &Type, is_async: bool) {
        if let Some(s) = self {
            s.trans(out, is_async);
        }
    }
}

impl<T: Trans> Trans for Box<T> {
    fn trans(&mut self, out: &Type, is_async: bool) {
        T::trans(&mut **self, out, is_async);
    }
}

impl Trans for Block {
    fn trans(&mut self, out: &Type, is_async: bool) {
        self.stmts
            .iter_mut()
            .for_each(|stmt| stmt.trans(out, is_async));
    }
}

impl Trans for Stmt {
    fn trans(&mut self, out: &Type, is_async: bool) {
        match self {
            Stmt::Local(local) => {
                assert_no_attr!(local, self);
                local.trans(out, is_async);
            }
            // ignore items
            Stmt::Item(_item) => {}
            Stmt::Expr(expr, _semi) => expr.trans(out, is_async),
            Stmt::Macro(stmt_macro) => {
                *self = parse_quote_spanned! { stmt_macro.span() => ::core::compile_error!(#MACRO_ERROR); };
            }
        }
    }
}

impl Trans for Item {
    fn trans(&mut self, _out: &Type, _is_async: bool) {
        match self {
            Item::Const(item_const) => assert_no_attr!(item_const, self),
            Item::Enum(item_enum) => assert_no_attr!(item_enum, self),
            Item::ExternCrate(item_extern_crate) => assert_no_attr!(item_extern_crate, self),
            Item::Fn(item_fn) => assert_no_attr!(item_fn, self),
            Item::ForeignMod(item_foreign_mod) => assert_no_attr!(item_foreign_mod, self),
            Item::Impl(item_impl) => assert_no_attr!(item_impl, self),
            Item::Macro(item_macro) => assert_no_attr!(item_macro, self),
            Item::Mod(item_mod) => assert_no_attr!(item_mod, self),
            Item::Static(item_static) => assert_no_attr!(item_static, self),
            Item::Struct(item_struct) => assert_no_attr!(item_struct, self),
            Item::Trait(item_trait) => assert_no_attr!(item_trait, self),
            Item::TraitAlias(item_trait_alias) => assert_no_attr!(item_trait_alias, self),
            Item::Type(item_type) => assert_no_attr!(item_type, self),
            Item::Union(item_union) => assert_no_attr!(item_union, self),
            Item::Use(item_use) => assert_no_attr!(item_use, self),
            Item::Verbatim(_token_stream) => {}
            _ => unreachable!(),
        }
    }
}

impl Trans for Local {
    fn trans(&mut self, out: &Type, is_async: bool) {
        if let Some(init) = &mut self.init {
            init.expr.trans(out, is_async);
            if let Some((_, diverge)) = &mut init.diverge {
                diverge.trans(out, is_async);
            }
        }
    }
}

impl Trans for Expr {
    #[allow(clippy::too_many_lines)]
    fn trans(&mut self, out: &Type, is_async: bool) {
        match self {
            Expr::Array(expr_array) => {
                assert_no_attr!(expr_array, self);
                expr_array
                    .elems
                    .iter_mut()
                    .for_each(|el| el.trans(out, is_async));
            }
            Expr::Assign(expr_assign) => {
                assert_no_attr!(expr_assign, self);
                expr_assign.right.trans(out, is_async);
            }
            // async blocks are OK, but still no attrs allowed
            Expr::Async(expr_async) => {
                assert_no_attr!(expr_async, self);
            }
            Expr::Binary(expr_binary) => {
                assert_no_attr!(expr_binary, self);
                expr_binary.left.trans(out, is_async);
                expr_binary.right.trans(out, is_async);
            }
            Expr::Block(expr_block) => {
                assert_no_attr!(expr_block, self);
                expr_block.block.trans(out, is_async);
            }
            Expr::Break(expr_break) => {
                assert_no_attr!(expr_break, self);
                if let Some(expr) = &mut expr_break.expr {
                    expr.trans(out, is_async);
                }
            }
            Expr::Call(expr_call) => {
                assert_no_attr!(expr_call, self);
                expr_call.func.trans(out, is_async);
                expr_call
                    .args
                    .iter_mut()
                    .for_each(|arg| arg.trans(out, is_async));
            }
            Expr::Cast(expr_cast) => {
                assert_no_attr!(expr_cast, self);
                expr_cast.expr.trans(out, is_async);
            }
            // closures can be ignored, but no attrs allowed
            Expr::Closure(expr_closure) => {
                assert_no_attr!(expr_closure, self);
            }
            // const blocks can't have `await`s and/or `yield`s YET. that would be posible, once const-traits arrove
            Expr::Const(expr_const) => {
                assert_no_attr!(expr_const, self);
                expr_const.block.trans(out, is_async);
            }
            Expr::Continue(expr_continue) => {
                assert_no_attr!(expr_continue, self);
            }
            Expr::Field(expr_field) => {
                assert_no_attr!(expr_field, self);
                expr_field.base.trans(out, is_async);
            }
            Expr::ForLoop(expr_for_loop) => {
                assert_no_attr!(expr_for_loop, self);
                expr_for_loop.expr.trans(out, is_async);
                expr_for_loop.body.trans(out, is_async);
            }
            Expr::Group(expr_group) => {
                assert_no_attr!(expr_group, self);
                expr_group.expr.trans(out, is_async);
            }
            Expr::If(expr_if) => {
                assert_no_attr!(expr_if, self);
                expr_if.cond.trans(out, is_async);
                expr_if.then_branch.trans(out, is_async);
                if let Some((_, else_branch)) = &mut expr_if.else_branch {
                    else_branch.trans(out, is_async);
                }
            }
            Expr::Index(expr_index) => {
                assert_no_attr!(expr_index, self);
                expr_index.expr.trans(out, is_async);
                expr_index.index.trans(out, is_async);
            }
            Expr::Infer(expr_infer) => {
                assert_no_attr!(expr_infer, self);
            }
            Expr::Let(expr_let) => {
                assert_no_attr!(expr_let, self);
                expr_let.expr.trans(out, is_async);
            }
            Expr::Lit(expr_lit) => {
                assert_no_attr!(expr_lit, self);
            }
            Expr::Loop(expr_loop) => {
                assert_no_attr!(expr_loop, self);
                expr_loop.body.trans(out, is_async);
            }
            Expr::Macro(expr_macro) => {
                *self = parse_quote_spanned! { expr_macro.span() => ::core::compile_error!(#MACRO_ERROR) }
            }
            Expr::Match(expr_match) => {
                assert_no_attr!(expr_match, self);
                expr_match.expr.trans(out, is_async);
                expr_match
                    .arms
                    .iter_mut()
                    .for_each(|arm| arm.trans(out, is_async));
            }
            Expr::MethodCall(expr_method_call) => {
                assert_no_attr!(expr_method_call, self);
                expr_method_call.receiver.trans(out, is_async);
                expr_method_call
                    .args
                    .iter_mut()
                    .for_each(|arg| arg.trans(out, is_async));
            }
            Expr::Paren(expr_paren) => {
                assert_no_attr!(expr_paren, self);
                expr_paren.expr.trans(out, is_async);
            }
            Expr::Path(expr_path) => assert_no_attr!(expr_path, self),
            Expr::Range(expr_range) => {
                assert_no_attr!(expr_range, self);
                expr_range.start.trans(out, is_async);
                expr_range.end.trans(out, is_async);
            }
            Expr::RawAddr(expr_raw_addr) => {
                assert_no_attr!(expr_raw_addr, self);
                expr_raw_addr.expr.trans(out, is_async);
            }
            Expr::Reference(expr_reference) => {
                assert_no_attr!(expr_reference, self);
                expr_reference.expr.trans(out, is_async);
            }
            Expr::Repeat(expr_repeat) => {
                assert_no_attr!(expr_repeat, self);
                expr_repeat.expr.trans(out, is_async);
            }
            Expr::Return(expr_return) => {
                assert_no_attr!(expr_return, self);
                expr_return.expr.trans(out, is_async);
            }
            Expr::Struct(expr_struct) => {
                assert_no_attr!(expr_struct, self);
                expr_struct
                    .fields
                    .iter_mut()
                    .for_each(|f| f.trans(out, is_async));
                expr_struct.rest.trans(out, is_async);
            }
            Expr::Try(expr_try) => {
                assert_no_attr!(expr_try, self);
                expr_try.expr.trans(out, is_async);
            }
            Expr::TryBlock(expr_try_block) => {
                assert_no_attr!(expr_try_block, self);
                expr_try_block.block.trans(out, is_async);
            }
            Expr::Tuple(expr_tuple) => {
                assert_no_attr!(expr_tuple, self);
                expr_tuple
                    .elems
                    .iter_mut()
                    .for_each(|el| el.trans(out, is_async));
            }
            Expr::Unary(expr_unary) => {
                assert_no_attr!(expr_unary, self);
                expr_unary.expr.trans(out, is_async);
            }
            Expr::Unsafe(expr_unsafe) => {
                assert_no_attr!(expr_unsafe, self);
                expr_unsafe.block.trans(out, is_async);
            }
            Expr::Verbatim(token_stream) => {
                *self = parse_quote_spanned! {token_stream.span() => ::core::compile_error!("Could not parse this, please remove")}
            }
            Expr::While(expr_while) => {
                assert_no_attr!(expr_while, self);
                expr_while.cond.trans(out, is_async);
                expr_while.body.trans(out, is_async);
            }
            Expr::Await(expr_await) => {
                assert_no_attr!(expr_await, self);
                if is_async {
                    let fut = &expr_await.base;
                    expr_await.base = parse_quote_spanned! { expr_await.span() => unsafe { ::do_you_yield::not_sync::Await::<_, #out>::___make(#fut) } }
                } else {
                    *self = parse_quote_spanned! { expr_await.span() => ::core::compile_error!(#AWAIT_ERROR) }
                }
            }
            Expr::Yield(expr_yield) => {
                let span = expr_yield.span();
                let expr = &mut expr_yield.expr;
                expr.trans(out, is_async);
                let module: Ident = if is_async {
                    parse_quote!(not_sync)
                } else {
                    parse_quote!(sync)
                };
                *self = parse_quote_spanned! {span => unsafe { ::do_you_yield::#module::Yield::<#out>::___make(#expr) }.await };
            }
            _ => todo!(),
        }
    }
}

impl Trans for Arm {
    fn trans(&mut self, _out: &Type, _is_async: bool) {
        if !self.attrs.is_empty() {
            *self = parse_quote_spanned!(self.span() =>  _ => ::core::compile_error!(#ATTR_ERROR));
        }
    }
}

impl Trans for FieldValue {
    fn trans(&mut self, _out: &Type, _is_async: bool) {
        if !self.attrs.is_empty() {
            self.expr = parse_quote_spanned!(self.span() =>  ::core::compile_error!(#ATTR_ERROR));
        }
    }
}
