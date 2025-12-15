use std::collections::HashMap;

use melior::{
    helpers::LlvmBlockExt,
    ir::{r#type::IntegerType, Block, Location, Value},
};
use melior::dialect::{arith, llvm};
use melior::dialect::llvm::{AllocaOptions, LoadStoreOptions};
use melior::dialect::llvm::r#type::pointer;
use melior::ir::attribute::{IntegerAttribute, TypeAttribute};
use crate::{
    ast::{AssignStmt, LetStmt},
    codegen::expressions::compile_expr,
};

use super::ModuleCtx;

/// A let statement
///
/// let x = 2;
pub fn compile_let<'ctx: 'parent, 'parent>(
    ctx: &ModuleCtx<'ctx>,
    locals: &mut HashMap<String, Value<'ctx, 'parent>>,
    block: &'parent Block<'ctx>,
    stmt: &LetStmt,
) {
    let i64_ty = IntegerType::new(ctx.ctx, 64).into();

    let op = arith::constant(
        ctx.ctx,
        IntegerAttribute::new(IntegerType::new(ctx.ctx, 64).into(), 1).into(),
        Location::unknown(ctx.ctx),
    );
    let op_ref = block.append_operation(op);

    let value:Value = op_ref.result(0).expect("IDK").into();

    let expr = stmt.expr.clone();

    let expr_value = compile_expr(ctx, locals, block, &expr);

    let op = llvm::store(ctx.ctx, expr_value, value, Location::unknown(ctx.ctx), LoadStoreOptions::default());

    let i64_ptr_ty = llvm::r#type::pointer(ctx.ctx, 64);

    let op = llvm::alloca(ctx.ctx, value, i64_ptr_ty, Location::unknown(ctx.ctx), AllocaOptions::new().elem_type(Some(TypeAttribute::new(i64_ty))));

    let op_ref = block.append_operation(op);

    let val = op_ref.result(0).expect("IDK").into();

    locals.insert(stmt.variable.clone(), val);

    let op = llvm::store(ctx.ctx, expr_value, val, Location::unknown(ctx.ctx), LoadStoreOptions::default());

    block.append_operation(op);
}

/// An assign statement
///
/// x = 2;
pub fn compile_assign<'ctx: 'parent, 'parent>(
    ctx: &ModuleCtx<'ctx>,
    locals: &mut HashMap<String, Value<'ctx, 'parent>>,
    block: &'parent Block<'ctx>,
    stmt: &AssignStmt,
) {
    match locals.get(&stmt.variable) {
        Some(val) => {
            let expr = stmt.expr.clone();

            let expr_value = compile_expr(ctx, locals, block, &expr);

            let op = llvm::store(ctx.ctx, expr_value, *val, Location::unknown(ctx.ctx), LoadStoreOptions::default());

            block.append_operation(op);

        },
        None => {}
    }


}
