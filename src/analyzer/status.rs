use crate::error::InvalidStatusDataError;
use crate::item::Item;
use crate::result;

pub struct Status {
    items: Vec<Box<dyn Item>>,
}

impl Status {
    pub fn new() -> Status {
        Status { items: Vec::new() }
    }

    pub fn add_item(&mut self, item: Box<dyn Item>) {
        self.items.push(item);
    }

    pub fn get_results(&self) -> Vec<Box<dyn result::Result>> {
        let mut ret = Vec::new();

        for item in self.items.iter() {
            ret.push(item.get_result());
        }

        ret
    }

    pub fn add(&mut self, data: Vec<String>) -> Result<(), InvalidStatusDataError> {
        if data.len() != self.items.len() {
            // todo: more info
            return Err(InvalidStatusDataError {});
        }

        let mut index: usize = 0;
        for elem in data.iter() {
            let add_res = self.items[index].add(elem);
            match add_res {
                Ok(_) => {}
                Err(err) => todo!(),
            }

            index += 1;
        }

        Ok(())
    }
}
