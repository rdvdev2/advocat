mod compiler;
mod template;

pub use compiler::CompilationError as Error;
pub use compiler::P1XX;
pub use template::generate_main;
