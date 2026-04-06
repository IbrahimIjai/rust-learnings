#[derive(Debug, Clone, PartialEq)]
struct TODO {
    id: u8,
    title: String,
    is_completed: bool,
}

struct TODOLIST {
    data: Vec<TODO>,
    next_id: u8,
}

impl TODOLIST {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            next_id: 1,
        }
    }

    fn create_todo(&mut self, title: String) -> u8 {
        let current_id = self.next_id;
        let todo = TODO {
            id: current_id,
            title,
            is_completed: false,
        };

        self.next_id += 1;
        self.data.push(todo);
        current_id
    }

    fn get_all_todos(&self) -> &Vec<TODO> {
        &self.data
    }

    fn get_single_todo (&self, id: u8) -> Option<&TODO> {
        self.data.iter().find(|todo| todo.id == id)
    } 
    
      
}

fn main() {}
