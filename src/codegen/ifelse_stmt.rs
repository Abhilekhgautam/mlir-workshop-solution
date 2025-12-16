use std::collections::HashMap;
use melior::dialect::scf;
use melior::ir::{Block, Location, Region, Value};

use crate::ast::IfStmt;
use crate::codegen::expressions::compile_expr;
use super::{compile_statement, ModuleCtx};

pub fn compile_if<'ctx, 'parent>(
    ctx: &ModuleCtx<'ctx>,
    locals: &mut HashMap<String, Value<'ctx, 'parent>>,
    block: &'parent Block<'ctx>,
    stmt: &IfStmt,
) {
    let cond_value = compile_expr(ctx, locals, block, &stmt.cond);

    let then_region = Region::new();

    let then_block = Block::new(&[]);

    let mut then_locals = locals.clone();

    for stmt in &stmt.then.stmts {
        compile_statement(ctx, &mut then_locals, &then_block, stmt);
    }
    let op_ref = scf::r#yield(&[], Location::unknown(ctx.ctx));

    then_block.append_operation(op_ref);

    then_region.append_block(then_block);

    let else_region = Region::new();
    //let block = then_region.append_block(stmt.then);

    let if_op = scf::r#if(cond_value, &[], then_region, else_region, Location::unknown(ctx.ctx));

    block.append_operation(if_op);
}
