use std::io;
#[derive(Debug)]
enum Status {
    Active,
    Inactive,
    Suspended,
    Graduated,
    Probation,
}

#[derive(Debug)]
struct Student {
    id: u8,
    name: String,
    age: u32,
    status: Status,
}
#[derive(Debug)]
struct StudentList {
    data: Vec<Student>,
    next_id: u8,
}

impl StudentList {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            next_id: 1,
        }
    }

    fn add_student(&mut self, name: String, age: u32) -> u8 {
        let current_id = self.next_id;

        let student = Student {
            id: current_id,
            name,
            age,
            status: Status::Active,
        };

        self.data.push(student);
        self.next_id += 1;
        current_id
    }

    fn get_all_students(&self) -> &Vec<Student> {
        &self.data
    }

    fn get_single_student(&self, id: u8) -> Option<&Student> {
        self.data.iter().find(|student| student.id == id)
    }

    fn update_status(&mut self, id: u8, new_status: Status) {
        if let Some(student) = self.data.iter_mut().find(|s| s.id == id) {
            student.status = new_status;
        }
    }

    fn graduate_student(&mut self, id: u8) {
        self.update_status(id, Status::Graduated);
    }

    fn delete_student(&mut self, id: u8) {
        self.data.retain(|student| student.id != id);
    }
}

fn main() {
    println!("Welcome to the school");

    println!("Add new student");

    let mut students = StudentList::new();

    println!("\n\n\n....................Commands:..............");
    println!("View: to see all students");
    println!("Add: to add student");
    println!("Del: to delete student");
    println!("Update: to update student status");
    println!("Graduate: to graduate student");
    println!("Exit: to exi");
    println!("................................................\n\n");

    let mut command = String::new();

    loop {
        take_command(&mut command);

        if command == "view" {
            println!("\n\nresponse................\n");

            if students.get_all_students().is_empty() {
                println!("No students found. Please add students.");
            } else {
                for student in students.get_all_students() {
                    println!(
                        "ID: {}, Name: {}, Age: {}, Status: {:?}",
                        student.id, student.name, student.age, student.status
                    );
                }
            }
            println!("\n................End response\n");
        } else if command == "del" {
            println!("\n\nresponse................\n");
            let mut student_id = String::new();

            println!("Enter student ID to delete:");
            io::stdin()
                .read_line(&mut student_id)
                .expect("Failed to read line");

            students.delete_student(student_id.trim().parse().unwrap());

            println!(" Student deleted successfully");
            println!("\n................End response\n");
        } else if command == "add" {
            println!("\n\nresponse................\n");
            println!("Enter student details:");
            let mut student_name = String::new();
            let mut student_age = String::new();

            println!("Enter student name:");
            io::stdin()
                .read_line(&mut student_name)
                .expect("Failed to read line");

            println!("Enter student age:");
            io::stdin()
                .read_line(&mut student_age)
                .expect("Failed to read line");

            let age: u32 = student_age.trim().parse().unwrap();

            students.add_student(student_name.trim().to_string(), age);
            println!(" Student added successfully");
            println!("\n................End response\n");
        } else if command == "update" {
            println!("\n\nresponse................\n");
            let mut student_id = String::new();
            let mut new_status = String::new();

            println!("Enter student ID:");
            io::stdin()
                .read_line(&mut student_id)
                .expect("Failed to read line");

            println!("Enter new status (Active, Inactive, Suspended, Graduated, Probation):");
            io::stdin()
                .read_line(&mut new_status)
                .expect("Failed to read line");

            let status = match new_status.trim() {
                "Active" => Status::Active,
                "Inactive" => Status::Inactive,
                "Suspended" => Status::Suspended,
                "Graduated" => Status::Graduated,
                "Probation" => Status::Probation,
                _ => {
                    println!("Invalid status. Please try again.");
                    continue;
                }
            };

            students.update_status(student_id.trim().parse().unwrap(), status);
            println!(" Student status updated successfully");
            println!("\n................End response\n");
        } else if command == "graduate" {
            let mut student_id = String::new();
            println!("\n\nresponse................\n");
            println!("Enter student ID to graduate:");
            io::stdin()
                .read_line(&mut student_id)
                .expect("Failed to read line");

            students.graduate_student(student_id.trim().parse().unwrap());
            println!(" Student graduated successfully");
            println!("\n................End response\n");
        } else if command == "exit" {
            println!("\n\nresponse................\n");
            println!("Goodbye!");
            println!("\n................End response\n");
            break;
        } else {
            println!("Unknown command: {}", command);
        }
    }
}

fn take_command(cmd: &mut String) {
    println!("Enter command:");
    cmd.clear();
    io::stdin().read_line(cmd).expect("Failed to read line");
    *cmd = cmd.trim().to_string();
}
