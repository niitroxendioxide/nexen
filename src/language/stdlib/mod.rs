pub mod math;
pub mod string;
pub mod logging;

pub fn register_std_functions(registry: &mut crate::language::binder::FunctionRegistry) {
    // math::register_math_functions(registry);

    registry.register("len", string::str_len);
    registry.register("println", logging::print);
}
