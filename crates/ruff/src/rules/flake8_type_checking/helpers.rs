use num_traits::Zero;
use rustpython_parser::ast::{Constant, Expr, ExprKind};

use ruff_python_ast::context::Context;
use ruff_python_ast::helpers::{map_callable, to_call_path};
use ruff_python_ast::types::{Binding, BindingKind, ExecutionContext, ScopeKind};

/// Return `true` if [`Expr`] is a guard for a type-checking block.
pub fn is_type_checking_block(context: &Context, test: &Expr) -> bool {
    // Ex) `if False:`
    if matches!(
        test.node,
        ExprKind::Constant {
            value: Constant::Bool(false),
            ..
        }
    ) {
        return true;
    }

    // Ex) `if 0:`
    if let ExprKind::Constant {
        value: Constant::Int(value),
        ..
    } = &test.node
    {
        if value.is_zero() {
            return true;
        }
    }

    // Ex) `if typing.TYPE_CHECKING:`
    if context.resolve_call_path(test).map_or(false, |call_path| {
        call_path.as_slice() == ["typing", "TYPE_CHECKING"]
    }) {
        return true;
    }

    false
}

pub const fn is_valid_runtime_import(binding: &Binding) -> bool {
    if matches!(
        binding.kind,
        BindingKind::Importation(..)
            | BindingKind::FromImportation(..)
            | BindingKind::SubmoduleImportation(..)
    ) {
        binding.runtime_usage.is_some() && matches!(binding.context, ExecutionContext::Runtime)
    } else {
        false
    }
}

pub fn runtime_evaluated(
    context: &Context,
    base_classes: &[String],
    decorators: &[String],
) -> bool {
    if !base_classes.is_empty() {
        if runtime_evaluated_base_class(context, base_classes) {
            return true;
        }
    }
    if !decorators.is_empty() {
        if runtime_evaluated_decorators(context, decorators) {
            return true;
        }
    }
    false
}

fn runtime_evaluated_base_class(context: &Context, base_classes: &[String]) -> bool {
    if let ScopeKind::Class(class_def) = &context.current_scope().kind {
        for base in class_def.bases.iter() {
            if let Some(call_path) = context.resolve_call_path(base) {
                if base_classes
                    .iter()
                    .any(|base_class| to_call_path(base_class) == call_path)
                {
                    return true;
                }
            }
        }
    }
    false
}

fn runtime_evaluated_decorators(context: &Context, decorators: &[String]) -> bool {
    if let ScopeKind::Class(class_def) = &context.current_scope().kind {
        for decorator in class_def.decorator_list.iter() {
            if let Some(call_path) = context.resolve_call_path(map_callable(decorator)) {
                if decorators
                    .iter()
                    .any(|decorator| to_call_path(decorator) == call_path)
                {
                    return true;
                }
            }
        }
    }
    false
}
