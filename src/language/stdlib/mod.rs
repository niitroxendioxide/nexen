pub mod math;
pub mod string;
pub mod logging;
pub mod input;

pub fn register_std_functions(registry: &mut crate::language::binder::FunctionRegistry) {
    registry.register("len", string::str_len);
    registry.register("tonumber", string::str_to_num);
    registry.register("print", logging::print);
    registry.register("fib", math::fib);
    registry.register("rand", math::random);
    registry.register("input", input::std_listen);
}
