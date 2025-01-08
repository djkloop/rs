#[derive(Debug, Clone, Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}

#[derive(Debug, Clone, Copy)]
struct Employee {
    position: Position,
    work_hours: i32,
}

fn print_employee(employee: Employee) {
    println!("{:?}", employee);
}

pub fn derive_demo() {
    let manager = Employee {
        position: Position::Manager,
        work_hours: 40,
    };

    // let supervisor = Employee {
    //     position: Position::Supervisor,
    //     work_hours: 35,
    // };

    // let worker = Employee {
    //     position: Position::Worker,
    //     work_hours: 30,
    // };

    print_employee(manager);
    print_employee(manager);
    // print_employee(&supervisor);
    // print_employee(&worker);
}
