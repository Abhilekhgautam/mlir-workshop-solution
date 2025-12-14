use std::collections::HashMap;

use melior::{
    helpers::LlvmBlockExt,
    ir::{r#type::IntegerType, Block, Location, Value},
};
use melior::dialect::{arith, llvm};
use melior::dialect::llvm::{AllocaOptions, LoadStoreOptions};
use melior::ir::attribute::IntegerAttribute;
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

    let op = arith::constant(
        ctx.ctx,
        IntegerAttribute::new(IntegerType::new(ctx.ctx, 32).into(), 1).into(),
        Location::unknown(ctx.ctx),
    );
    let op_ref = block.append_operation(op);

    let value = op_ref.result(0).expect("IDK").into();
    
    let op = llvm::alloca(ctx.ctx, value, IntegerType::new(ctx.ctx, 32).into(), Location::unknown(ctx.ctx), AllocaOptions::default());

    let op_ref = block.append_operation(op);

    let val = op_ref.result(0).expect("IDK").into();

    locals.insert(stmt.variable.clone(), val);
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

        },
        None => {}
    }


}
