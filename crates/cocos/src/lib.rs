pub mod app;
pub mod executor;
pub mod graphics_context;
pub mod scene;

pub use app::*;
pub use executor::*;
pub use graphics_context::*;
pub use scene::*;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
