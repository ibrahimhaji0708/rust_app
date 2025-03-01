pub(crate) mod fridge {
    pub mod freezer {
        pub struct Freezer {
            pub id: String, // Freezer
            pub temp: i16,
        }

        impl Freezer {
            pub fn freezing(&mut self) {
                self.temp -= 3;
                println!("{} is freezing!", self.id);
            }
        }
    }

    pub mod cooler {
        pub struct Cooler {
            pub id: String, // Cooler
            pub temp: u32,
        }

        impl Cooler {
            pub fn refrigerate(&mut self) {
                self.temp += 2;
                println!("{} is refrigerating at {}°C", self.id, self.temp);
            }

            pub fn cool(&mut self) {
                self.temp += 5;
                println!("{} is cooling at {}°C", self.id, self.temp);
            }

            pub fn const_cool(&mut self) {
                self.temp += 7;
                println!("{} is cooling at constant {}°C", self.id, self.temp);
            }
        }
    }
}

trait Cool {
    fn cool(&self);
}

impl Cool for fridge::freezer::Freezer {
    fn cool(&self) {
        println!("The {} is freezing at {}°C", self.id, self.temp);
    }
}

impl Cool for fridge::cooler::Cooler {
    fn cool(&self) {
        println!("The {} is cooling at {}°C", self.id, self.temp);
    }
}

pub fn fridge_operations() {
    let freezer = fridge::freezer::Freezer {
        id: String::from("Freezer"),
        temp: 0,
    };
    let mut cooler = fridge::cooler::Cooler {
        id: String::from("Cooler"),
        temp: 2,
    };

    cooler.refrigerate();
    cooler.cool();
    cooler.const_cool();

    freezer.cool();
    cooler.cool();
}
