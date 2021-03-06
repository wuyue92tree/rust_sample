use rustpython::rustpython_vm as vm;

fn main() -> vm::PyResult<()> {
    vm::Interpreter::default().enter(|vm| {
        let scope = vm.new_scope_with_builtins();

        let code_obj = vm
            .compile(
                r#"print("Hello World! Yue.")"#,
                vm::compile::Mode::Exec,
                "<embedded>".to_owned(),
            )
            .map_err(|err| vm.new_syntax_error(&err))?;

        vm.run_code_obj(code_obj, scope)?;

        Ok(())
    })
}