use crate::error::InvalidStatusDataError;
use crate::item::Item;

pub struct Calculator {
    items: Vec<Box<dyn Item>>,
}

impl Calculator {
    pub fn new() -> Calculator {
        Calculator { items: Vec::new() }
    }

    pub fn add_item(&mut self, item: Box<dyn Item>) {
        self.items.push(item);
    }

    pub fn get_results(&self) -> Vec<String> {
        let mut ret = Vec::new();

        for item in self.items.iter() {
            ret.push(item.get_result());
        }

        ret
    }

    pub fn add_data(&mut self, data: Vec<String>) -> Result<(), InvalidStatusDataError> {
        if data.len() != self.items.len() {
            // todo: more info
            let detail = format!(
                "given data' length({}) doesn't fit items length({})",
                data.len(),
                self.items.len()
            );
            return Err(InvalidStatusDataError { detail });
        }

        let mut index: usize = 0;
        for datum in data.iter() {
            let add_res = self.items[index].add(datum);
            match add_res {
                Ok(_) => {}
                Err(err) => {
                    let detail = format!(
                        "given data({}) doesn't fit the item {}({:?})",
                        err.data, err.item_title, err.typ
                    );
                    return Err(InvalidStatusDataError { detail });
                }
            }

            index += 1;
        }

        Ok(())
    }

    pub fn get_titles(&self) -> String {
        let titles: Vec<String> = self.items.iter().map(|item| item.get_title()).collect();
        titles.join(",")
    }
}
