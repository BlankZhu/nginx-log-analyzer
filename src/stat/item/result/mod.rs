pub mod noop_result;
pub mod enum_result;
pub mod num_result;
pub mod time_result;

pub trait ItemResult {
    fn get_readable_result(&self) -> String;
    fn get_json_result(&self) -> String;
}