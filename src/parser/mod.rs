mod error;
mod string_literal;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(kagami, "/parser/kagami.rs");

pub use self::kagami::KagamiModuleParser;
pub use error::Errors;
