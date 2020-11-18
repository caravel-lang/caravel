use super::symbol_table::{Symbol, SymbolTable};
use crate::lexer::token::TokenValue;
use crate::parser::ast;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Linkage;
use inkwell::module::Module;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetTriple,
};
use inkwell::types::BasicTypeEnum;
use inkwell::values::{AnyValueEnum, BasicValueEnum};
use inkwell::IntPredicate;
use inkwell::OptimizationLevel;
use std::convert::TryFrom;
use std::path::Path;

pub struct CodegenContext<'a, 'ct> {
    llvm_ctx: &'ct Context,
    module: &'a Module<'ct>,
    builder: &'a Builder<'ct>,
    symbol_table: &'a mut SymbolTable<'a>,
}

pub trait Codegen {
    fn codegen<'a, 'ct>(&self, ctx: &mut CodegenContext<'a, 'ct>) -> Option<BasicValueEnum<'a>>;
}

impl Codegen for ast::ExpressionNode {
    fn codegen<'a, 'ct>(&self, ctx: &mut CodegenContext<'a, 'ct>) -> Option<BasicValueEnum<'a>> {
        match self.get_value() {
            ast::ExpressionValue::BinaryOp(lhs, op, rhs) => {
                let lhs_val_wrapper = lhs.codegen(ctx).unwrap();
                let rhs_val_wrapper = rhs.codegen(ctx).unwrap();

                let ty = lhs_val_wrapper.get_type();
                if ty != rhs_val_wrapper.get_type() {
                    panic!("Mismatched types");
                }

                Some(match ty {
                    BasicTypeEnum::IntType(_) => {
                        let lhs_val = lhs_val_wrapper.into_int_value();
                        let rhs_val = rhs_val_wrapper.into_int_value();
                        match op.value {
                            TokenValue::Addition => {
                                let result = ctx.builder.build_int_add(lhs_val, rhs_val, "int_add");
                                BasicValueEnum::IntValue(result)
                            }
                            TokenValue::Subtraction => {
                                let result = ctx.builder.build_int_sub(lhs_val, rhs_val, "int_sub");
                                BasicValueEnum::IntValue(result)
                            }
                            TokenValue::Multiplication => {
                                let result = ctx.builder.build_int_mul(lhs_val, rhs_val, "int_mul");
                                BasicValueEnum::IntValue(result)
                            }
                            TokenValue::Division => {
                                let result = ctx
                                    .builder
                                    .build_int_unsigned_div(lhs_val, rhs_val, "int_div");
                                BasicValueEnum::IntValue(result)
                            }
                            TokenValue::Modulo => {
                                let result = ctx
                                    .builder
                                    .build_int_signed_rem(lhs_val, rhs_val, "int_mod");
                                BasicValueEnum::IntValue(result)
                            }
                            TokenValue::Equals => {
                                BasicValueEnum::IntValue(ctx.builder.build_int_compare(
                                    IntPredicate::EQ,
                                    lhs_val,
                                    rhs_val,
                                    "int_eq",
                                ))
                            }

                            _ => unreachable!(),
                        }
                    }

                    BasicTypeEnum::FloatType(_) => {
                        let lhs_val = lhs_val_wrapper.into_float_value();
                        let rhs_val = rhs_val_wrapper.into_float_value();
                        match op.value {
                            TokenValue::Addition => {
                                let result =
                                    ctx.builder.build_float_add(lhs_val, rhs_val, "float_add");
                                BasicValueEnum::FloatValue(result)
                            }
                            TokenValue::Subtraction => {
                                let result =
                                    ctx.builder.build_float_sub(lhs_val, rhs_val, "float_sub");
                                BasicValueEnum::FloatValue(result)
                            }
                            TokenValue::Multiplication => {
                                let result =
                                    ctx.builder.build_float_mul(lhs_val, rhs_val, "float_mul");
                                BasicValueEnum::FloatValue(result)
                            }
                            TokenValue::Division => {
                                let result =
                                    ctx.builder.build_float_div(lhs_val, rhs_val, "float_div");
                                BasicValueEnum::FloatValue(result)
                            }
                            TokenValue::Modulo => {
                                let result =
                                    ctx.builder.build_float_rem(lhs_val, rhs_val, "float_mod");
                                BasicValueEnum::FloatValue(result)
                            }

                            _ => unreachable!(),
                        }
                    }

                    _ => unreachable!(),
                })
            }

            ast::ExpressionValue::IntLiteral(int_val) => Some(BasicValueEnum::IntValue(
                ctx.llvm_ctx.i64_type().const_int(int_val as u64, false),
            )),

            ast::ExpressionValue::FloatLiteral(float_val) => Some(BasicValueEnum::FloatValue(
                ctx.llvm_ctx.f64_type().const_float(float_val),
            )),

            ast::ExpressionValue::BoolLiteral(val) => Some(BasicValueEnum::IntValue(
                ctx.llvm_ctx.bool_type().const_int(val as u64, false),
            )),

            ast::ExpressionValue::StringLiteral(_) => unimplemented!(),

            ast::ExpressionValue::CharLiteral(_) => unimplemented!(),

            ast::ExpressionValue::Debug(expr_node) => {
                let expr_val = expr_node.codegen(ctx).unwrap();

                match expr_val {
                    BasicValueEnum::IntValue(_) => {
                        let size = expr_val.get_type().into_int_type().get_bit_width();

                        // Is boolean?
                        if size == 1 {
                            ctx.builder.build_call(
                                ctx.symbol_table
                                    .get("debug_bool")
                                    .unwrap()
                                    .value
                                    .into_function_value(),
                                &[expr_val],
                                "call_debug_bool",
                            );
                        } else {
                            ctx.builder.build_call(
                                ctx.symbol_table
                                    .get("debug_int")
                                    .unwrap()
                                    .value
                                    .into_function_value(),
                                &[expr_val],
                                "call_debug_int",
                            );
                        }
                    }
                    BasicValueEnum::FloatValue(_) => {
                        ctx.builder.build_call(
                            ctx.symbol_table
                                .get("debug_float")
                                .unwrap()
                                .value
                                .into_function_value(),
                            &[expr_val],
                            "call_debug_float",
                        );
                    }

                    _ => panic!("Unexpected."),
                }

                None
            }

            ast::ExpressionValue::Assignment(identifier, expr) => {
                let expr_val = expr.codegen(ctx).unwrap();

                ctx.symbol_table.add(Symbol {
                    identifier,
                    value: AnyValueEnum::from(expr_val),
                });

                Some(expr_val)
            }

            ast::ExpressionValue::Identifier(identifier) => Some(
                BasicValueEnum::try_from(ctx.symbol_table.get(&identifier[..]).unwrap().value)
                    .unwrap(),
            ),
        }
    }
}

impl Codegen for ast::BlockNode {
    fn codegen<'a, 'ct>(&self, ctx: &mut CodegenContext<'a, 'ct>) -> Option<BasicValueEnum<'a>> {
        for statement in self.get_statements() {
            statement.codegen(ctx);
        }

        None
    }
}

/// Creates the main function and generates IR for expressions
pub fn codegen_program<'a>(program: ast::BlockNode) {
    let mut root_symbol_table = SymbolTable::new();

    let context = Context::create();
    let builder = context.create_builder();
    let module = context.create_module("module");

    // void main()
    let main_type = context.void_type().fn_type(&[], false);
    let main = module.add_function("main", main_type, Some(Linkage::External));

    let main_block = context.append_basic_block(main, "main_entry");
    builder.position_at_end(main_block);

    // Initialize stdlib

    // void debug_float(double out);
    let debug_float_fn_type = context
        .void_type()
        .fn_type(&[BasicTypeEnum::FloatType(context.f64_type())], false);

    let debug_float_fn =
        module.add_function("debug_float", debug_float_fn_type, Some(Linkage::External));

    root_symbol_table.add(Symbol {
        identifier: "debug_float".to_owned(),
        value: AnyValueEnum::FunctionValue(debug_float_fn),
    });

    // void debug_int(int out);
    let debug_int_fn_type = context
        .void_type()
        .fn_type(&[BasicTypeEnum::IntType(context.i64_type())], false);

    let debug_int_fn = module.add_function("debug_int", debug_int_fn_type, Some(Linkage::External));

    root_symbol_table.add(Symbol {
        identifier: "debug_int".to_owned(),
        value: AnyValueEnum::FunctionValue(debug_int_fn),
    });

    // void debug_bool(bool out);
    let debug_bool_fn_type = context
        .void_type()
        .fn_type(&[BasicTypeEnum::IntType(context.bool_type())], false);

    let debug_bool_fn =
        module.add_function("debug_bool", debug_bool_fn_type, Some(Linkage::External));

    root_symbol_table.add(Symbol {
        identifier: "debug_bool".to_owned(),
        value: AnyValueEnum::FunctionValue(debug_bool_fn),
    });

    // Codegen program
    let mut codegen_context = CodegenContext {
        builder: &builder,
        llvm_ctx: &context,
        module: &module,
        symbol_table: &mut root_symbol_table,
    };
    program.codegen(&mut codegen_context);

    // return void
    builder.build_return(None);

    module.verify().unwrap();

    // Calls LLVM initialize functions
    Target::initialize_x86(&InitializationConfig::default());

    let target = Target::from_name("x86-64").unwrap();
    let target_machine = target
        .create_target_machine(
            &TargetTriple::create("x86_64-apple-darwin19.6.0"),
            "generic",
            "",
            OptimizationLevel::Default,
            RelocMode::Default,
            CodeModel::Default,
        )
        .unwrap();

    module.print_to_file("debug.ll").unwrap();
    target_machine
        .write_to_file(&module, FileType::Object, Path::new("out.o"))
        .unwrap();
}
