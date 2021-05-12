pub trait Stat {
    fn add(&mut self, elem: String);
    fn count(&self) -> usize;
    fn title(&self) -> &String;
    fn get_result(&mut self) -> String;
}