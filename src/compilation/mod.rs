mod compiler;
mod template;

pub use compiler::P1XX;
pub use template::generate_main;
pub use compiler::CompilationError as Error;