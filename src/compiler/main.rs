use std::path::PathBuf;
use std::fs;
use std::error::Error;

mod lexer;
mod parser;
mod ast;
mod type_checker;
mod ir;
mod codegen;

#[derive(Debug)]
struct CompilerOptions {
    input_file: PathBuf,
    output_file: PathBuf,
    optimization_level: u8,
    target: Target,
}

#[derive(Debug)]
enum Target {
    Native,
    Wasm,
    IR,
}

struct Compiler {
    options: CompilerOptions,
}

impl Compiler {
    fn new(options: CompilerOptions) -> Self {
        Self { options }
    }

    fn compile(&self) -> Result<(), Box<dyn Error>> {
        // 1. Read source file
        let source = fs::read_to_string(&self.options.input_file)?;

        // 2. Lexical analysis
        let tokens = lexer::tokenize(&source)?;

        // 3. Parsing
        let ast = parser::parse(tokens)?;

        // 4. Type checking and semantic analysis
        let typed_ast = type_checker::check(ast)?;

        // 5. IR generation
        let ir = ir::lower(typed_ast)?;

        // 6. Optimization passes
        let optimized_ir = if self.options.optimization_level > 0 {
            ir::optimize(ir, self.options.optimization_level)
        } else {
            ir
        };

        // 7. Code generation
        match self.options.target {
            Target::Native => codegen::emit_native(optimized_ir, &self.options.output_file)?,
            Target::Wasm => codegen::emit_wasm(optimized_ir, &self.options.output_file)?,
            Target::IR => codegen::emit_ir(optimized_ir, &self.options.output_file)?,
        }

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // TODO: Parse command line arguments
    let options = CompilerOptions {
        input_file: PathBuf::from("input.strx"),
        output_file: PathBuf::from("output.wasm"),
        optimization_level: 2,
        target: Target::Wasm,
    };

    let compiler = Compiler::new(options);
    compiler.compile()?;

    Ok(())
} 