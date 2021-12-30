mod f64;
mod hour;
mod isize;
mod minute;
mod noop;
mod str;

pub trait Result {
    fn get_readable_result(&self) -> String;
    fn get_json_result(&self) -> String;
}
