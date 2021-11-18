use wasmer::{Function, FunctionType, ImportObject, Instance, Module, RuntimeError, Store, Type, Val, imports};
use crate::api;
use crate::types::Address;

lazy_static::lazy_static! {
    static ref STORE: Store = Store::default();
}

pub fn run(module_wat: &str, fnc: &str, params: Vec<Val>) -> Result<Box<[Val]>, Box<dyn std::error::Error>> {
    let resolver: ImportObject = imports! {
        "env" => {
            "call" => Function::new(&STORE, &FunctionType::new(vec![Type::I32], vec![Type::I32]), api::call),
        },
    };
    let module = Module::new(&STORE, &module_wat)?;
    let instance = Instance::new(&module, &resolver)?;
    for exp in instance.exports.iter() {
        println!("{}", exp.0);
    }
    match instance.exports.get_function(fnc)?.call(&params) {
        Ok(value) => Ok(value),
        Err(error) => Err(Box::new(std::io::Error::new::<RuntimeError>(std::io::ErrorKind::InvalidData, error)))
    }
}

pub fn insert(address: Address, module_wat: &str) {
    
}