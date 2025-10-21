pub mod math;
pub mod string;
pub mod logging;

pub fn register_std_functions(registry: &mut crate::language::binder::FunctionRegistry) {
    registry.register("len", string::str_len);
    registry.register("print", logging::print);
    registry.register("fib", math::fib);
    registry.register("rand", math::random);
}
