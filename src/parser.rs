//! parser

use crate::prelude::Token;

/// syntax tree
pub enum Statement {
    /// assign
    Assign {
        /// name
        name: Token,
        /// expression
        expression: Expression,
    },
    /// assign quote
    AssignQuote {
        /// name
        name: Token,
        /// expression
        expression: Expression,
    },
    Expression(Expression),
}

pub enum Expression {
    /// identifier
    Ident {
        /// name
        name: Token,
    },
    /// list
    List {
        /// value
        value: Vec<Token>,
    },
    /// monad call
    MonadCall {
        /// operator
        op: Box<Expression>,
        /// operand
        lhs: Box<Expression>,
    },
    /// dyad call
    DyadCall {
        /// rhs operand
        rhs: Box<Expression>,
        /// operator
        op: Box<Expression>,
        /// lhs operand
        lhs: Box<Expression>,
    },
    /// quote
    Quote {
        /// colon
        colon: Token,
        /// expression
        expression: Box<Expression>,
    },
    /// call
    Call {
        expression: Box<Expression>,
        colon: Token,
    },
    /// spread
    Spread {
        /// verb
        verb: Box<Expression>,
        /// slash
        slash: Token,
    },
    /// anon function
    Lambda {
        /// left brace
        left_brace: Token,
        /// expr
        expression: Box<Expression>,
    },
}

pub fn parse() -> Option<Statement> {
    None
}

#[cfg(test)]
mod test {
    use crate::prelude::*;

    #[test]
    fn parse1() {
        let s = Scanner::new("x+y");
        let v: Vec<_> = s.collect();
        v.into_iter()
            .zip(
                vec![
                    Token::no_span(TokenKind::Ident, "x"),
                    Token::no_span(TokenKind::Builtin, "+"),
                    Token::no_span(TokenKind::Ident, "y"),
                ]
                .into_iter(),
            )
            .map(|(a, b)| assert!(a.compare_no_span(b)))
            .for_each(drop);
    }

    #[test]
    fn parse2() {
        let _src = "a = 1 + 2";
        let _a = Statement::Assign {
            name: Token::no_span(TokenKind::Ident, "a"),
            expression: Expression::DyadCall {
                rhs: Box::new(Expression::List {
                    value: vec![Token::no_span(TokenKind::Int(1), "1")],
                }),
                op: Box::new(Expression::Ident {
                    name: Token::no_span(TokenKind::Ident, "+"),
                }),
                lhs: Box::new(Expression::List {
                    value: vec![Token::no_span(TokenKind::Int(2), "2")],
                }),
            },
        };
    }
}

// x =. 1 2 3 4 5
// y =. 6 7 8 9 10
// x + y
// # $ x
// {] + ]} x
// {1+]} (f 1 2 3 4 5)
// amp :=: [:[ ]: [:]
