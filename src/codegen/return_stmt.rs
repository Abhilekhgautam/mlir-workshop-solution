use std::collections::HashMap;

use melior::{
    dialect::func,
    ir::{Block, BlockRef, Location, Value},
};
use melior::dialect::arith;
use melior::ir::attribute::IntegerAttribute;
use melior::ir::r#type::IntegerType;
use crate::ast::ReturnStmt;

use super::{expressions::compile_expr, ModuleCtx};

pub fn compile_return<'ctx, 'parent>(
    ctx: &ModuleCtx<'ctx>,
    locals: &HashMap<String, Value>,
    block: &'parent Block<'ctx>,
    stmt: &ReturnStmt,
) {
    let ret_val = compile_expr(ctx, locals, block, &stmt.expr);
    let op = func::r#return(
        &[ret_val],
        Location::unknown(ctx.ctx),
    );
    block.append_operation(op);
}
