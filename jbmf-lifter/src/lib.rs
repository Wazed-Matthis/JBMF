pub mod control_flow_graph;
pub mod instruction_info;
pub mod lifter;
pub mod translate;

#[macro_export]
macro_rules! extract_constant_fields {
    ($constant_pool:expr, $index:expr, $variant:path { $($field:ident),+ }) => {
        if let Some($variant { $($field),+ }) = $constant_pool.get($index) {
            ($($field),+)
        } else {
            panic!("Could not extract {} from constant pool at index {}", stringify!($variant), $index.0);
        }
    };
     ($constant_pool:expr, $index:expr, $variant:path => ($($field:ident),+)) => {
         if let Some($variant ($($field),+)) = $constant_pool.get($index) {
            ($($field),+)
        } else {
            unreachable!();
        }
    };
}