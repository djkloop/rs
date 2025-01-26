
#[derive(Debug, Clone, Copy)]
struct LuggageId(usize);

#[derive(Debug, Clone, Copy)]
struct Luggage(LuggageId);

struct CheckIn(LuggageId);

struct OnLoad(LuggageId);

struct OffLoad(LuggageId);

struct AwatingPickup(LuggageId);

#[derive(Debug, Clone, Copy)]
struct EndCustody(LuggageId);

impl Luggage {
    fn new(id: LuggageId) -> Self {
        Luggage(id)
    }

    fn check_in(self) -> CheckIn {
        CheckIn(self.0)
    }
}

impl CheckIn {
  fn onload(self) -> OnLoad {
    OnLoad(self.0)
  }
}

impl OnLoad {
    fn offload(self) -> OffLoad {
        OffLoad(self.0)
    }
}

impl OffLoad {
    fn carousel(self) -> AwatingPickup {
        AwatingPickup(self.0)
    }
}

impl AwatingPickup {
    fn pickup(self) -> (Luggage, EndCustody) {
        (Luggage(self.0), EndCustody(self.0))
    }
}



pub fn typestate_a() {
    let id = LuggageId(1);
    let luggage = Luggage::new(id);
    let luggage = luggage.check_in().onload().offload().carousel().pickup();
    println!("{:?}, {:?}", luggage.0, luggage.1);
}

