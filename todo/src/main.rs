struct Task{
    id:u32,
    done:bool,
    title:String,
}
struct ToDo{
    tasks:Vec<Task>,
}
impl ToDo{
    fn add(&mut self, title:String){
        let id=self.tasks.len() as u32 + 1;
        self.tasks.push(Task{id,title,done:false});
        println!("Task added");
    }
    fn remove(&mut self, id:u32){
        self.tasks.retain(|x| x.id != id);
        println!("Task removed");
    }
    fn list(&self){
        if self.tasks.is_empty(){
            println!("No tasks");
            return;
        }
        for task in &self.tasks{
            println!("[{}] {}: {}",if task.done{"done"} else {""},task.id, task.title);
        }
    }
    fn done(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.done = true;
                println!("Task marked as done");
                return;
            }
        }
        println!("Task not found");
    }
}
fn main() {
    let mut app = ToDo { tasks: Vec::new() };

    loop {
        println!("\n =TODO=");
        println!("1. Add task");
        println!("2. Remove task");
        println!("3. List tasks");
        println!("4. Mark done");
        println!("5. Quit");
        println!("Enter choice:");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let choice: u32 = input.trim().parse().unwrap_or(0);

        match choice {
            1 => {
                println!("Enter task title:");
                let mut title = String::new();
                std::io::stdin().read_line(&mut title).unwrap();
                app.add(title.trim().to_string());
            }
            2 => {
                println!("Enter task id:");
                let mut id = String::new();
                std::io::stdin().read_line(&mut id).unwrap();
                let id: u32 = id.trim().parse().unwrap_or(0);
                app.remove(id);
            }
            3 => {
                app.list();
            }
            4 => {
                println!("Enter task id:");
                let mut id = String::new();
                std::io::stdin().read_line(&mut id).unwrap();
                let id: u32 = id.trim().parse().unwrap_or(0);
                app.done(id);
            }
            5 => {
                println!("Bye!");
                break;
            }
            _ => println!("Wrong choice"),
        }
    }
}
