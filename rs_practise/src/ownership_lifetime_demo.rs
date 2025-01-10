use std::fmt::Debug;

#[derive(Debug)]
struct Cards {
    inner: Vec<IdCard>,
}

#[derive(Debug, PartialEq)]
enum City {
    Beijing,
    Shanghai,
    Guangzhou,
}

#[derive(Debug)]
struct IdCard {
    city: City,
    name: String,
    age: u8,
}

impl IdCard {
    pub fn new(city: City, name: String, age: u8) -> Self {
        Self {
            city,
            name: name.to_string(),
            age,
        }
    }
}

fn new_ids() -> Cards {
    Cards {
        inner: vec![
            IdCard::new(City::Beijing, "张三".to_string(), 20),
            IdCard::new(City::Shanghai, "李四".to_string(), 21),
            IdCard::new(City::Guangzhou, "王五".to_string(), 22),
        ],
    }
}

#[derive(Debug)]
struct YoungPeople<'a> {
    inner: Vec<&'a IdCard>,
}

impl<'a> YoungPeople<'a> {
    fn living_in_shanghai(&self) -> Self {
        Self {
            inner: self
                .inner
                .iter()
                .filter(|people| people.city == City::Shanghai)
                .map(|people| *people)
                .collect(),
        }
    }
}

pub fn ownership_lifetime_demo() {
    let ids = new_ids();
    let young_people = YoungPeople {
        inner: ids.inner.iter().filter(|people| people.age <= 21).collect(),
    };

    println!("------------");
    for people in ids.inner.iter() {
        println!("people name, age: {:?}, {:?}", people.name, people.age);
    }
    println!("------------");
    for people in young_people.inner.iter() {
        println!("people name, age: {:?}, {:?}", people.name, people.age);
    }
    println!("------------");
    let young_people_in_shanghai = young_people.living_in_shanghai();
    for people in young_people_in_shanghai.inner.iter() {
        println!("people name, age: {:?}, {:?}", people.name, people.age);
    }
    println!("------------");
}
