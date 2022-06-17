//! Parsers to recognize try-catch and try-of-catch blocks

use crate::erl_syntax::erl_ast::node_impl::AstNodeImpl;
use crate::erl_syntax::erl_ast::AstNode;
use crate::erl_syntax::node::erl_catch_clause::CatchClause;
use crate::erl_syntax::node::erl_exception_pattern::ExceptionPattern;
use crate::erl_syntax::parsers::defs::{ErlParserError, ParserResult};
use crate::erl_syntax::parsers::misc::{tok, tok_keyword};
use crate::erl_syntax::parsers::parse_case::parse_case_clause;
use crate::erl_syntax::parsers::parse_expr::{
  parse_comma_sep_exprs1, parse_guardexpr, parse_matchexpr, EXPR_STYLE_FULL,
};
use crate::erl_syntax::parsers::parser_input::ParserInput;
use crate::erl_syntax::token_stream::keyword::Keyword;
use crate::erl_syntax::token_stream::token_type::TokenType;
use crate::source_loc::SourceLoc;
use nom::combinator::{cut, map, opt};
use nom::multi::{many1, separated_list1};
use nom::sequence::{preceded, terminated, tuple};
use nom::{bytes::complete::tag, character::complete::char, error::context};

/// Parse `Class:Error:Stack` triple into `ExceptionPattern`
pub fn parse_exception_pattern(
  input: ParserInput,
) -> nom::IResult<ParserInput, ExceptionPattern, ErlParserError> {
  map(
    tuple((
      parse_matchexpr,
      preceded(tok(TokenType::Colon), parse_matchexpr),
      opt(preceded(tok(TokenType::Colon), parse_matchexpr)),
    )),
    |(class_pattern, err_pattern, stack_pattern)| {
      ExceptionPattern::new(class_pattern, err_pattern, stack_pattern)
    },
  )(input)
}

/// Parses a repeated catch-clause part after `catch` keyword: `Expr when Expr -> Expr`
pub fn parse_catch_clause(
  input: ParserInput,
) -> nom::IResult<ParserInput, CatchClause, ErlParserError> {
  map(
    tuple((
      // Class:Error:Stacktrace
      parse_exception_pattern,
      // when <Expression>
      opt(preceded(tok_keyword(Keyword::When), parse_guardexpr)),
      // -> Expression
      preceded(tok(TokenType::RightArr), parse_comma_sep_exprs1::<{ EXPR_STYLE_FULL }>),
    )),
    |(exc_pattern, maybe_when, body)| {
      CatchClause::new(
        exc_pattern,
        maybe_when,
        AstNodeImpl::new_comma_expr(SourceLoc::new(&input), body),
      )
    },
  )(input.clone())
}

fn parse_try_catch_inner(input: ParserInput) -> ParserResult<AstNode> {
  map(
    tuple((
      context(
        "try-catch block trial expression",
        cut(parse_comma_sep_exprs1::<{ EXPR_STYLE_FULL }>),
      ),
      // Optional OF followed by match clauses
      opt(preceded(
        tok_keyword(Keyword::Of),
        context("try block: 'of' clauses", cut(many1(parse_case_clause))),
      )),
      // Followed by 1 or more `catch Class:Exception:Stack -> ...` clauses
      preceded(
        tok_keyword(Keyword::Catch),
        context(
          "try block: 'catch' clauses",
          cut(separated_list1(tok(TokenType::Semicolon), parse_catch_clause)),
        ),
      ),
    )),
    |(body, of_branches, catch_clauses)| {
      let loc = SourceLoc::new(&input);
      AstNodeImpl::new_try_catch(
        loc.clone(),
        AstNodeImpl::new_comma_expr(loc, body),
        of_branches,
        catch_clauses,
      )
    },
  )(input.clone())
}

/// Parses a `try-catch` or a `try-of-catch` block
pub(crate) fn parse_try_catch(input: ParserInput) -> ParserResult<AstNode> {
  preceded(
    tok_keyword(Keyword::Try),
    context(
      "try-catch or try-of block",
      cut(terminated(parse_try_catch_inner, tok_keyword(Keyword::End))),
    ),
  )(input)
}
