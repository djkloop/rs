
#[derive(Debug)]
struct  Employee<State> {
    name: String,
    state: State,
}

impl<State> Employee<State> {
    fn transition<NextState>(self, next_state: NextState) -> Employee<NextState> {
        Employee { name: self.name, state: next_state }
    }
}

#[derive(Debug)]
struct Agreement;
#[derive(Debug)]
struct Signature;
#[derive(Debug)]
struct Training;
#[derive(Debug)]
struct FailedTraining {
    score: u8,
}

#[derive(Debug)]
struct OnboardingComplete {
    score: u8,
}

impl Employee<Agreement> {
    fn new(name: String) -> Self {
        Employee { name, state: Agreement }
    }

    fn read_agreement(self) -> Employee<Signature> {
        self.transition(Signature)
    }
}

impl Employee<Signature> {
    fn sign(self) -> Employee<Training> {
        self.transition(Training)
    }
}

impl Employee<Training> {
    fn train(self, score: u8) -> Result<Employee<OnboardingComplete>, Employee<FailedTraining>> {
        if score >= 80 {
            Ok(self.transition(OnboardingComplete { score }))
        } else {
            Err(self.transition(FailedTraining { score }))
        }
    }
}


pub fn typestate_demo() {
    let employee = Employee::new("John Doe".to_string());
    let onboarding = employee.read_agreement();
    let training = onboarding.sign();
    let is_complete = training.train(80);

    match is_complete {
        Ok(complete) => println!("Employee onboarding complete: {:?}", complete.state.score),
        Err(failed) => println!("Employee onboarding failed: {:?}", failed.state.score),
    }
}


