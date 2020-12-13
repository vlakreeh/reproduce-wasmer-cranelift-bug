#[cfg(test)]
mod tests {
    use wasmer::{Cranelift, Module, Store};
    use wasmer_engine_native::Native;

    #[test]
    fn it_doesnt_crash() {
        let mut cranelift = Cranelift::new();
        let native = Native::new(&mut cranelift);
        let store = Store::new(&native.engine());
        
        // Error with an undefined symbol "wasmer_probestack",
        // the binary used is clang from https://wapm.io/package/clang
        Module::from_binary(&store, include_bytes!("clang.wasm")).expect("unable to create module");
    }
}
