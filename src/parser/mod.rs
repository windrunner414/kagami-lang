pub use self::kagami::KagamiModuleParser;
pub use error::Errors;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(kagami, "/parser/kagami.rs");
mod error;
mod string_literal;
