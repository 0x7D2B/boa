//! For statement parsing.
//!
//! More information:
//!  - [MDN documentation][mdn]
//!  - [ECMAScript specification][spec]
//!
//! [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/for
//! [spec]: https://tc39.es/ecma262/#sec-for-statement

// use crate::syntax::lexer::TokenKind;
use crate::{
    syntax::{
        ast::{
            node::{ForLoop, Node},
            Keyword,
        },
        parser::{
            expression::Expression,
            statement::declaration::{Declaration, TokenParserForConst},
            statement::Statement,
            AllowAwait, AllowReturn, AllowYield, Cursor, ParseError, TokenParser,
        },
    },
    BoaProfiler,
};

use std::io::Read;

/// For statement parsing
///
/// More information:
///  - [MDN documentation][mdn]
///  - [ECMAScript specification][spec]
///
/// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/for
/// [spec]: https://tc39.es/ecma262/#sec-for-statement
#[derive(Debug, Clone, Copy)]
pub(in crate::syntax::parser::statement) struct ForStatement {
    allow_yield: AllowYield,
    allow_await: AllowAwait,
    allow_return: AllowReturn,
}

impl ForStatement {
    /// Creates a new `ForStatement` parser.
    pub(in crate::syntax::parser::statement) fn new<Y, A, R>(
        allow_yield: Y,
        allow_await: A,
        allow_return: R,
    ) -> Self
    where
        Y: Into<AllowYield>,
        A: Into<AllowAwait>,
        R: Into<AllowReturn>,
    {
        Self {
            allow_yield: allow_yield.into(),
            allow_await: allow_await.into(),
            allow_return: allow_return.into(),
        }
    }
}

impl<R> TokenParser<R> for ForStatement
where
    R: Read,
{
    type Output = Node;

    fn parse(self, cursor: &mut Cursor<R>) -> Result<Self::Output, ParseError> {
        let _timer = BoaProfiler::global().start_event("ForStatement", "Parsing");
        cursor.expect(Keyword::For, "for statement")?;
        let init =
            Declaration::new(self.allow_yield, self.allow_await, false).parse_const(cursor)?;
        cursor.expect(Keyword::In, "for statement")?;
        let iterable = Expression::new(true, self.allow_yield, self.allow_await).parse(cursor)?;
        let body =
            Statement::new(self.allow_yield, self.allow_await, self.allow_return).parse(cursor)?;
        return Ok(ForLoop::new(init, iterable, body).into());
    }
}
