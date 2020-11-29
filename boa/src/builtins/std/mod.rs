//! This module implements global `Std` functions.
//!
//! (fork)
//!

use super::function::make_builtin_fn;
use super::console::formatter;
use crate::{
    builtins::BuiltIn,
    object::ObjectInitializer,
    property::Attribute,
    value::Value,
    BoaProfiler, Context, Result,
};

/// `Std` implementation.
///
/// (fork)
///
#[derive(Debug, Clone, Copy)]
pub(crate) struct Std;

impl BuiltIn for Std {
    const NAME: &'static str = "Std";


    fn attribute() -> Attribute {
        Attribute::READONLY | Attribute::NON_ENUMERABLE | Attribute::PERMANENT
    }

    fn init(context: &mut Context) -> (&'static str, Value, Attribute) {
        let _timer = BoaProfiler::global().start_event(Self::NAME, "init");

        let global = context.global_object().clone();
        // make_builtin_fn(Self::println, "println", &global, 0, context);
        make_builtin_fn(Self::print, "print", &global, 0, context);

        let std = ObjectInitializer::new(context)
            // .function(Self::println, "println", 0)
            .function(Self::print, "print", 0)
            .build();
        (Self::NAME, std.into(), Self::attribute())
    }
}

impl Std {
    /// `println(...data)`
    ///
    /// (fork)
    /// Prints to the console.
    ///
    // pub(crate) fn println(_: &Value, args: &[Value], context: &mut Context) -> Result<Value> {
    //     println!("{}", formatter(&args, context)?);
    //     Ok(Value::undefined())
    // }

    /// `print(...data)`
    ///
    /// (fork)
    /// Prints to the console.
    ///
    pub(crate) fn print(_: &Value, args: &[Value], context: &mut Context) -> Result<Value> {
        print!("{}\n", formatter(&args, context)?);
        Ok(Value::undefined())
    }
}
