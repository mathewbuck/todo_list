use std::env;
// save state so when it's opened it recalls the list additions. 
// have it generate a txt file log [x] 
// Use termion to start adding color to this beesh
struct TodoItem {
  name: String,
  completed: char
}

impl TodoItem {
  fn new(name: String) -> TodoItem {
    return TodoItem{
      name: name,
      completed: ' '
    };
  }
}

struct TodoList {
  list: Vec<TodoItem>
}

impl TodoList {
  fn new() -> TodoList {
    return TodoList{list: Vec::new()};
  }

  fn add_to_list(&mut self, name: String) {
    let todo_item = TodoItem::new(name);
    self.list.push(todo_item);
  }

fn print(&self) {
    for (index, item) in self.list.iter().enumerate() {
          println!("{} [{}] - {}", index, item.completed, item.name);
     }
  }

  fn mark_done(&mut self, index: usize) {
    if self.list[index].completed == ' ' {
      self.list[index].completed = 'x';
    } else {
      self.list[index].completed = ' ';
    }      
  }

  fn remove_task(&mut self, index: usize) {
    self.list.remove(index);
  }
}

enum Command {
  Get, 
  Add(String),
  Done(usize),
  Remove(usize)
}

fn main() {
  let arguments: Vec<String> = env::args().collect();
  let mut todo_list = TodoList::new();

  let command = match arguments[1].as_str() {
    "get" => Command::Get,
    "add" => Command::Add(arguments[2].clone()),
    "done" => Command::Done(arguments[2].parse().expect("error converting to int")),
    "remove" => Command::Remove(arguments[2].parse().expect("error parsing for a remove")),
    _ => panic!("You must provide an accepted command")
  };
  
  todo_list.add_to_list("Test to see if it works".to_string());
  todo_list.add_to_list("Customize todo list".to_string());
  todo_list.add_to_list("Do karate in the garage".to_string());
  todo_list.mark_done(0);

  // karate chop - add ascii art
  

  println!("{:#?}", arguments);
  
  println!("Super McAwesome Todo list");

  match command {
    Command::Get => todo_list.print(),
    Command::Add(task) => {
    todo_list.add_to_list(task);
    todo_list.print();
    }
    Command::Done(index) => {
      todo_list.mark_done(index);
      todo_list.print();
      }
      Command::Remove(index) => {
        todo_list.remove_task(index);
        todo_list.print();
      }
  }
}

