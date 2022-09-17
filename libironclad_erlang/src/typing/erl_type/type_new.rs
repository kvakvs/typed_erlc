//! Constructors for complex ErlTypes

use crate::literal::Literal;
use crate::typing::erl_integer::ErlInteger;
use crate::typing::erl_type::binary_type::{BinaryTypeHeadElement, BinaryTypeTailElement};
use crate::typing::erl_type::typekind::TypeKind;
use crate::typing::erl_type::{ErlType, TypeImpl};
use crate::typing::fn_clause_type::FnClauseType;
use crate::typing::fn_type::FnType;
use crate::typing::record_field_type::RecordFieldType;
use libironclad_util::mfarity::MFArity;
use std::iter;
use std::sync::Arc;

//
// Constructors and Generators
//
impl TypeImpl {
  /// Wrap an inner type together with a typevariable name
  #[inline]
  pub(crate) fn new_wrap_named(typevar: String, kind: TypeKind) -> ErlType {
    TypeImpl { typevar: Some(typevar), kind }.into()
  }

  /// Create a type variable without a type (`any()` is assumed)
  #[inline]
  pub(crate) fn new_named_any(typevar: String) -> ErlType {
    TypeImpl { typevar: Some(typevar), kind: TypeKind::Any }.into()
  }

  /// Create a type variable without a name
  #[inline]
  pub(crate) fn new_unnamed(kind: TypeKind) -> ErlType {
    TypeImpl { typevar: None, kind }.into()
  }

  /// Attach a type variable name to a type
  pub(crate) fn set_name(&self, typevar: String) -> ErlType {
    TypeImpl { typevar: Some(typevar), kind: self.kind.clone() }.into()
  }

  /// Try match type name and arity vs known basic types
  pub(crate) fn from_name(
    maybe_module: Option<String>,
    type_name: String,
    args: &[ErlType],
  ) -> ErlType {
    #[allow(clippy::single_match)]
    match args.len() {
      0 => match type_name.as_ref() {
        "any" => return TypeImpl::any(),
        "none" => return TypeImpl::none(),

        "number" => return TypeImpl::number(),
        "integer" => return TypeImpl::integer(),
        "float" => return TypeImpl::float(),

        "atom" => return TypeImpl::atom(),
        "boolean" => return TypeImpl::boolean(),

        "list" => return TypeImpl::any_list(),
        "nil" => return TypeImpl::nil(),

        "tuple" => return TypeImpl::any_tuple(),

        "pid" => return TypeImpl::pid(),
        "port" => return TypeImpl::port(),
        "reference" => return TypeImpl::reference(),
        _ => {}
      },
      _ => {}
    }
    // We were not able to find a basic type of that name and arity
    TypeImpl::new_unnamed(TypeKind::UserDefinedType {
      name: MFArity::new_opt(maybe_module, &type_name, args.len()),
      args: args.to_vec(),
    })
  }
}
