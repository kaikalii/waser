use wasmer::{imports, Function, Instance, Store, Value};

pub struct Module {
    start_input: Function,
    put_input: Function,
    start_output: Function,
    get_output: Function,
    store: Store,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Compile(#[from] wasmer::CompileError),
    #[error("{0}")]
    Instantiation(#[from] wasmer::InstantiationError),
    #[error("{0}")]
    Export(#[from] wasmer::ExportError),
    #[error("{0}")]
    RuntimeError(#[from] wasmer::RuntimeError),
    #[error("start_output return value must be a single i64")]
    StartOutputReturn,
    #[error("get_output return value must be a single i32")]
    GetOutputReturn,
}

#[allow(clippy::result_large_err)]
impl Module {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let mut store = Store::default();
        let module = wasmer::Module::new(&store, bytes)?;
        let imports = imports! {};
        let instance = Instance::new(&mut store, &module, &imports)?;

        let start_input = instance.exports.get_function("start_input")?.clone();
        let put_input = instance.exports.get_function("put_input")?.clone();
        let start_output = instance.exports.get_function("start_output")?.clone();
        let get_output = instance.exports.get_function("get_output")?.clone();

        Ok(Self {
            start_input,
            put_input,
            start_output,
            get_output,
            store,
        })
    }
    pub fn request(&mut self, input: impl AsRef<[u8]>) -> Result<Vec<u8>, Error> {
        fn inner(module: &mut Module, input: &[u8]) -> Result<Vec<u8>, Error> {
            module.start_input.call(&mut module.store, &[])?;
            for byte in input {
                module
                    .put_input
                    .call(&mut module.store, &[Value::I32(*byte as i32)])?;
            }
            let len = match module.start_output.call(&mut module.store, &[])?.as_ref() {
                [Value::I64(len)] => *len as usize,
                _ => return Err(Error::StartOutputReturn),
            };
            let mut buffer = Vec::with_capacity(len);
            for _ in 0..len {
                match module.get_output.call(&mut module.store, &[])?.as_ref() {
                    [Value::I32(byte)] => buffer.push(*byte as u8),
                    _ => return Err(Error::GetOutputReturn),
                }
            }
            Ok(buffer)
        }
        inner(self, input.as_ref())
    }
}
