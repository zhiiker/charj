use crate::compiler::Compiler;
use inkwell::context::Context;
use parser::parser::parse_program;

pub mod builtin;
pub mod codegen;
pub mod compiler;
pub mod namespace;
pub mod symbol_table;

pub fn compile_program(input: &str, filename: &str) -> Result<String, ()> {
    let context = Context::create();
    let module_name = filename.replace(".cj", "");

    let module = context.create_module(&module_name);
    let builder = context.create_builder();

    let parse_ast = parse_program(input);
    match parse_ast {
        Ok(unit) => {
            let compiler = Compiler::compile(&context, &builder, &module, &unit);
            compiler.run_jit();
            Ok(compiler.module.print_to_string().to_string())
        }
        Err(_) => Err(()),
    }
}

#[cfg(test)]
mod test {
    use crate::compile_program;

    #[test]
    #[rustfmt::skip]
    fn init_parser() {
        let result = compile_program(
            "default$main() {println(\"hello,world\")}",
            "hello.cj"
        );

        assert!(result.is_ok());
        println!("{}", result.unwrap());
    }
}
