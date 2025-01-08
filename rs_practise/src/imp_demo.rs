struct Temperature {
    degrees_f: f64,
}

impl Temperature {
    fn freezing(&self) -> Self {
        Self {
            degrees_f: self.degrees_f + 1.0,
        }
    }

    fn show_temp(&self) {
        println!("temp = {:?}", self.degrees_f);
    }
}

pub fn imp_demo() {
    let temp = Temperature { degrees_f: 32.0 };
    temp.show_temp();
    let f = temp.freezing();
    f.show_temp();
}
