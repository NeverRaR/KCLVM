use indexmap::IndexSet;
use kclvm_ast::ast::Program;
use kclvm_error::Diagnostic;
use kclvm_sema::{resolver::scope::ProgramScope, core::global_state::GlobalState};

/// Holds the result of the compile
#[derive(Default, Clone)]
pub struct AnalysisDatabase {
    pub prog: Program,
    pub scope: ProgramScope,
    pub diags: IndexSet<Diagnostic>,
    pub gs: GlobalState,
}
