// Copyright 2021 The KCL Authors. All rights reserved.

use kclvm_ast::ast;
use kclvm_runtime::ApiFunc;
use std::str;

use crate::codegen::traits::ValueMethods;

use super::context::LLVMCodeGenContext;

/*
 * Temporal functions
 */

/// Update runtime context pkgpath
pub fn update_ctx_pkgpath<'ctx>(gen: &'ctx LLVMCodeGenContext, pkgpath: &str) {
    gen.build_void_call(
        &ApiFunc::kclvm_context_set_kcl_pkgpath.name(),
        &[
            gen.current_runtime_ctx_ptr(),
            gen.native_global_string_value(pkgpath),
        ],
    );
}

/// Update runtime context filename
pub fn update_ctx_filename<'ctx, T>(gen: &'ctx LLVMCodeGenContext, node: &'ctx ast::Node<T>) {
    if !node.filename.is_empty() {
        gen.build_void_call(
            &ApiFunc::kclvm_context_set_kcl_filename.name(),
            &[
                gen.current_runtime_ctx_ptr(),
                gen.native_global_string_value(&node.filename),
            ],
        );
    }
}

/// Update runtime context line and column
pub fn update_ctx_line_col<'ctx, T>(gen: &'ctx LLVMCodeGenContext, node: &'ctx ast::Node<T>) {
    let mut current_line = gen.current_line.borrow_mut();
    if node.line != *current_line {
        *current_line = node.line;
        gen.build_void_call(
            &ApiFunc::kclvm_context_set_kcl_line_col.name(),
            &[
                gen.current_runtime_ctx_ptr(),
                gen.native_int_value(node.line as i32),
                gen.native_int_value(0),
            ],
        );
    }
}

/// Update runtime context line and column
pub fn update_ctx_current_line(gen: &LLVMCodeGenContext) {
    let current_line = gen.current_line.borrow_mut();
    gen.build_void_call(
        &ApiFunc::kclvm_context_set_kcl_line_col.name(),
        &[
            gen.current_runtime_ctx_ptr(),
            gen.native_int_value(*current_line as i32),
            gen.native_int_value(0),
        ],
    );
}
