use std::collections::HashMap;

use melior::{
    dialect::{arith, llvm},
    helpers::{ArithBlockExt, LlvmBlockExt},
    ir::{
        attribute::IntegerAttribute, r#type::IntegerType, Block, BlockRef, Location, Type, Value,
    },
};
use melior::dialect::func;
use melior::dialect::llvm::LoadStoreOptions;
use melior::ir::attribute::FlatSymbolRefAttribute;
use crate::ast::{Expr, Opcode};

use super::ModuleCtx;

// A right hand side expression: `2 + x * 3`
pub fn compile_expr<'ctx: 'parent, 'parent>(
    // Helper struct with the MLIR Context and Module
    ctx: &ModuleCtx<'ctx>,
    // Hashmap storing the local variables
    locals: &HashMap<String, Value<'ctx, 'parent>>,
    // The current block to work on.
    block: &'parent Block<'ctx>,
    // The expression to compile.
    expr: &Expr,
) -> Value<'ctx, 'parent> {
    match expr {
        Expr::Number(value) => {
            let op = arith::constant(
                ctx.ctx,
                IntegerAttribute::new(IntegerType::new(ctx.ctx, 64).into(), *value).into(),
                Location::unknown(ctx.ctx),
            );
            let op_ref = block.append_operation(op);

            op_ref.result(0).expect("IDK").into()
        }
        Expr::Variable(name) => match locals.get(name) {
            Some(value) => {
                let op = llvm::load(ctx.ctx, *value, IntegerType::new(ctx.ctx, 64).into(), Location::unknown(ctx.ctx), LoadStoreOptions::default());
                let op_ref = block.append_operation(op);

                op_ref.result(0).expect("IDK").into()
            },
            None => panic!("Undefined variable: {}", name),
        },
        Expr::Op(lhs_expr, opcode, rhs_expr) => match opcode {
            Opcode::Mul => {
                let op = arith::muli(
                    compile_expr(ctx, locals, block, lhs_expr),
                    compile_expr(ctx, locals, block, rhs_expr),
                    Location::unknown(ctx.ctx),
                );
                let op_ref = block.append_operation(op);

                op_ref.result(0).expect("IDK").into()
            }
            Opcode::Div => {
                let op = arith::divsi(
                    compile_expr(ctx, locals, block, lhs_expr),
                    compile_expr(ctx, locals, block, rhs_expr),
                    Location::unknown(ctx.ctx),
                );
                let op_ref = block.append_operation(op);

                op_ref.result(0).expect("IDK").into()
            },
            Opcode::Add => {
                let op = arith::addi(
                    compile_expr(ctx, locals, block, lhs_expr),
                    compile_expr(ctx, locals, block, rhs_expr),
                    Location::unknown(ctx.ctx),
                );
                let op_ref = block.append_operation(op);

                op_ref.result(0).expect("IDK").into()
            },
            Opcode::Sub => {
                let op = arith::subi(
                    compile_expr(ctx, locals, block, lhs_expr),
                    compile_expr(ctx, locals, block, rhs_expr),
                    Location::unknown(ctx.ctx),
                );
                let op_ref = block.append_operation(op);

                op_ref.result(0).expect("IDK").into()
            },
            Opcode::Eq => {
                let op = arith::cmpi(
                    ctx.ctx,
                    arith::CmpiPredicate::Eq,
                    compile_expr(ctx, locals, block, lhs_expr),
                    compile_expr(ctx, locals, block, rhs_expr),
                    Location::unknown(ctx.ctx),
                );
                let op_ref = block.append_operation(op);

                op_ref.result(0).expect("IDK").into()
            },
            Opcode::Neq => {
                let op = arith::cmpi(
                    ctx.ctx,
                    arith::CmpiPredicate::Ne,
                    compile_expr(ctx, locals, block, lhs_expr),
                    compile_expr(ctx, locals, block, rhs_expr),
                    Location::unknown(ctx.ctx),
                );
                let op_ref = block.append_operation(op);

                op_ref.result(0).expect("IDK").into()
            },
        },
        Expr::Call { target, args } => {
            let arg_vec:Vec<Value<'_, '_>> = args.iter().map(|arg| compile_expr(ctx, locals, block, arg)).collect();
            let op = func::call(ctx.ctx, FlatSymbolRefAttribute::new(ctx.ctx, target), &arg_vec, &[IntegerType::new(ctx.ctx, 64).into()], Location::unknown(ctx.ctx));

            let op_ref = block.append_operation(op);

            op_ref.result(0).expect("IDK").into()
        },
    }
}
