use crate::{
    exec::{Executable, InterpreterState},
    gc::{Finalize, Trace},
    syntax::ast::node::Node,
    Context, Result, Value,
};
use std::fmt;

#[cfg(feature = "deser")]
use serde::{Deserialize, Serialize};

/// The `loop` statement creates a loop that executes a specified statement forever
///
/// (fork)
///
#[cfg_attr(feature = "deser", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Trace, Finalize, PartialEq)]
pub struct Loop {
    expr: Box<Node>,
    label: Option<Box<str>>,
}

impl Loop {
    pub fn expr(&self) -> &Node {
        &self.expr
    }

    pub fn label(&self) -> Option<&str> {
        self.label.as_ref().map(Box::as_ref)
    }

    pub fn set_label(&mut self, label: Box<str>) {
        self.label = Some(label);
    }

    /// Creates a `Loop` AST node.
    pub fn new<B>(body: B) -> Self
    where
        B: Into<Node>,
    {
        Self {
            expr: Box::new(body.into()),
            label: None,
        }
    }

    pub(in crate::syntax::ast::node) fn display(
        &self,
        f: &mut fmt::Formatter<'_>,
        indentation: usize,
    ) -> fmt::Result {
        write!(f, "loop ")?;
        self.expr().display(f, indentation)
    }
}

impl Executable for Loop {
    fn run(&self, context: &mut Context) -> Result<Value> {
        let mut result;
        loop {
            result = self.expr().run(context)?;
            match context.executor().get_current_state() {
                InterpreterState::Break(label) => {
                    handle_state_with_labels!(self, label, context, break);
                    break;
                }
                InterpreterState::Continue(label) => {
                    handle_state_with_labels!(self, label, context, continue)
                }
                InterpreterState::Return => {
                    return Ok(result);
                }
                InterpreterState::Executing => {
                    // Continue execution.
                }
            }
        }
        Ok(result)
    }
}

impl fmt::Display for Loop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display(f, 0)
    }
}

impl From<Loop> for Node {
    fn from(r#loop: Loop) -> Self {
        Self::Loop(r#loop)
    }
}
