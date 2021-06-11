use super::Item;

pub struct IsizeItem {
    title: String,
    data: Vec<isize>,
}

impl Item for IsizeItem {
    fn add(&mut self, elem: &str) {
        let parse_isize = elem.parse::<isize>();
        match parse_isize {
            Ok(value) => {
                self.data.push(value);
            }
            Err(err) => {
                eprintln!(
                    "failed parse IsizeItem[{}] from: [{}] , detail: [{}]",
                    &self.title, &elem, &err
                );
            }
        }
    }

    fn count(&self) -> usize {
        self.data.len()
    }

    fn title(&self) -> &str {
        self.title.as_str()
    }

    fn get_result(&self) -> Box<dyn super::result::ItemResult> {
        todo!()
    }
}

pub struct F64Item {
    title: String,
    data: Vec<f64>,
}

impl Item for F64Item {
    fn add(&mut self, elem: &str) {
        let parse_isize = elem.parse::<f64>();
        match parse_isize {
            Ok(value) => {
                self.data.push(value);
            }
            Err(err) => {
                eprintln!(
                    "failed parse F64Item[{}] from: [{}] , detail: [{}]",
                    &self.title, &elem, &err
                );
            }
        }
    }

    fn count(&self) -> usize {
        self.data.len()
    }

    fn title(&self) -> &str {
        self.title.as_str()
    }
    fn get_result(&self) -> Box<dyn super::result::ItemResult> {
        todo!()
    }
}
