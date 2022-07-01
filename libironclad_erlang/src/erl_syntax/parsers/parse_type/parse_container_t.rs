//! Parsing types for containers: list, tuple and map.

use crate::erl_syntax::parsers::defs::ParserResult;
use crate::erl_syntax::parsers::misc::{
  tok, tok_comma, tok_curly_close, tok_curly_open, tok_ellipsis, tok_hash, tok_square_close,
  tok_square_open,
};
use crate::erl_syntax::parsers::parse_type;
use crate::erl_syntax::parsers::parser_error::ErlParserError;
use crate::erl_syntax::parsers::parser_input::ParserInput;
use crate::erl_syntax::token_stream::token_type::TokenType;
use crate::typing::erl_type::map_type::MapMemberType;
use crate::typing::erl_type::{ErlType, ErlTypeImpl};
use crate::typing::typevar::Typevar;
use nom::combinator::map;
use nom::error::context;
use nom::multi::separated_list0;
use nom::sequence::{delimited, pair, separated_pair};

/// Parse a list of types, returns a temporary list-type
pub fn type_of_list(input: ParserInput) -> nom::IResult<ParserInput, ErlType, ErlParserError> {
  map(
    delimited(
      tok_square_open,
      context("type arguments for a list() type", parse_type::comma_sep_typeargs0),
      tok_square_close,
    ),
    |elements| {
      let typevar_types = Typevar::vec_of_typevars_into_types(elements);
      ErlTypeImpl::list_of_types(typevar_types)
    },
  )(input)
}

/// Parse a list of type and ellipsis, creating a nonempty list-type
pub fn type_of_nonempty_list(
  input: ParserInput,
) -> nom::IResult<ParserInput, ErlType, ErlParserError> {
  map(
    delimited(
      tok_square_open,
      context(
        "type arguments for a nonempty_list() type",
        separated_pair(parse_type::alt_typevar_or_type, tok_comma, tok_ellipsis),
      ),
      tok_square_close,
    ),
    |(typevar, _ellip)| ErlTypeImpl::list_of(ErlTypeImpl::new_typevar(typevar), true),
  )(input)
}

/// Parse a tuple of types, returns a temporary tuple-type
pub fn type_of_tuple(input: ParserInput) -> nom::IResult<ParserInput, ErlType, ErlParserError> {
  map(
    delimited(
      tok_curly_open,
      context("a tuple() type", parse_type::comma_sep_typeargs0),
      tok_curly_close,
    ),
    |elements| {
      let typevar_types = Typevar::vec_of_typevars_into_types(elements);
      ErlTypeImpl::new_tuple_move(typevar_types)
    },
  )(input)
}

fn map_member_type(input: ParserInput) -> nom::IResult<ParserInput, MapMemberType, ErlParserError> {
  map(
    separated_pair(
      parse_type::alt_typevar_or_type,
      tok(TokenType::RightDoubleArr),
      parse_type::alt_typevar_or_type,
    ),
    |(key, value)| MapMemberType {
      key: ErlTypeImpl::new_typevar(key),
      value: ErlTypeImpl::new_typevar(value),
    },
  )(input)
}

/// Parses a comma separated list of map field types
fn comma_sep_map_members0(input: ParserInput) -> ParserResult<Vec<MapMemberType>> {
  separated_list0(tok_comma, context("parsing member types of a map type", map_member_type))(input)
}

/// Parse a map of types, returns a map-type
pub fn type_of_map(input: ParserInput) -> nom::IResult<ParserInput, ErlType, ErlParserError> {
  map(
    delimited(
      pair(tok_hash, tok_curly_open),
      context("a map() type", comma_sep_map_members0),
      tok_curly_close,
    ),
    ErlTypeImpl::new_map,
  )(input)
}
