//! Parse function definitions with Nom

use std::sync::Arc;

use crate::erl_syntax::erl_ast::ErlAst;
use crate::erl_syntax::node::erl_fn_clause::ErlFnClause;
use crate::erl_syntax::parsers::misc::{period, semicolon, ws_before, ws_before_mut};
use crate::erl_syntax::parsers::parse_atom::AtomParser;
use crate::erl_syntax::parsers::{AstParserResult, ErlParser, ErlParserError};
use libironclad_error::source_loc::SourceLoc;
use libironclad_util::mfarity::MFArity;
use nom::combinator::{cut, map, not, opt, peek};
use nom::multi::separated_list1;
use nom::sequence::{preceded, terminated, tuple};
use nom::{bytes::complete::tag, error::context};

impl ErlParser {
  fn parse_when_expr_for_fn(input: &str) -> AstParserResult {
    map(
      tuple((ws_before(tag("when")), cut(Self::parse_expr))),
      |(_, g)| g, // ignore 'when' tag, keep guard expr
    )(input)
  }

  /// Function will succeed if an atom is parsed and FN_NAME is true, will return Some<Value>.
  /// Function will succeed if no atom found but also FN_NAME is false, will return None.
  /// Function will fail otherwise.
  fn parse_fnclause_name<const REQUIRE_FN_NAME: bool>(
    input: &str,
  ) -> nom::IResult<&str, Option<String>, ErlParserError> {
    if REQUIRE_FN_NAME {
      // Succeed if FN_NAME=true and there is an atom
      return map(AtomParser::atom, Option::Some)(input);
    }
    // Succeed if FN_NAME=false and there is no atom
    map(peek(not(AtomParser::atom)), |_| Option::None)(input)
  }

  /// Parses a named clause for a top level function
  /// * FN_NAME: true if the parser must require function name
  fn parse_fnclause<const REQUIRE_FN_NAME: bool>(
    input: &str,
  ) -> nom::IResult<&str, ErlFnClause, ErlParserError> {
    map(
      tuple((
        // Function clause name
        ws_before_mut(Self::parse_fnclause_name::<REQUIRE_FN_NAME>),
        // Function arguments
        nom::error::context(
          "function clause arguments",
          Self::parse_parenthesized_list_of_exprs::<{ ErlParser::EXPR_STYLE_MATCHEXPR }>,
        ),
        // Optional: when <guard>
        nom::error::context(
          "when expression in a function clause",
          opt(Self::parse_when_expr_for_fn),
        ),
        nom::error::context(
          "function clause body",
          preceded(
            ws_before(tag("->")),
            // Body as list of exprs
            cut(Self::parse_comma_sep_exprs1::<{ ErlParser::EXPR_STYLE_FULL }>),
          ),
        ),
      )),
      |(maybe_name, args, when_expr, body)| {
        ErlFnClause::new(
          maybe_name,
          args,
          ErlAst::new_comma_expr(&SourceLoc::from_input(input), body),
          when_expr,
        )
      },
    )(input)
  }

  /// Builds a function definition from multiple parsed clauses
  fn _construct_fndef(location: &SourceLoc, fnclauses: Vec<ErlFnClause>) -> Arc<ErlAst> {
    assert!(!fnclauses.is_empty(), "Function clauses list can't be empty, i don't even..."); // unreachable
                                                                                             // println!("Construct fdef: {:?}", fnclauses);

    let arity = fnclauses[0].args.len();
    let fn_name = match &fnclauses[0].name {
      None => "TODO: lambda_name".to_string(),
      Some(s) => s.clone(),
    };
    let funarity = MFArity::new_local(&fn_name, arity);

    if !fnclauses.iter().all(|fnc| fnc.args.len() == arity) {
      panic!("Not all clauses have same arity")
    }

    ErlAst::new_fndef(location, funarity, fnclauses)
  }

  /// Parse function definition
  pub fn parse_fndef(input: &str) -> AstParserResult {
    map(
      terminated(
        separated_list1(
          semicolon,
          // if parse fails under here, will show this context message in error
          context("function clause", cut(Self::parse_fnclause::<true>)),
        ),
        period,
      ),
      |t| Self::_construct_fndef(&SourceLoc::from_input(input), t),
    )(input)
  }

  /// Lambda is an inline function definition
  pub fn parse_lambda(input: &str) -> AstParserResult {
    // Lambda is made of "fun" keyword, followed by multiple ";" separated clauses
    map(
      preceded(
        ws_before(tag("fun")),
        terminated(
          context("", separated_list1(semicolon, Self::parse_fnclause::<false>)),
          ws_before(tag("end")),
        ),
      ),
      |t| Self::_construct_fndef(&SourceLoc::from_input(input), t),
    )(input)
  }
}
