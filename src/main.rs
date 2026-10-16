use std::io;

#[derive(Debug)]
struct Task 
{
    description: String,
    completed: bool,
}

impl Task 
{
    fn complete(&mut self) 
    {
        self.completed = true;
    }
}

fn list_tasks(tasks: &[Task])
{
    for task in tasks.iter()
    {
        println!("task = {}, completed = {}", task.description, task.completed);
    }
}

fn add_task(tasks: &mut Vec<Task>, new_task_description: String)
{
    let new_task = Task 
    {
        description: new_task_description,
        completed: false,
    };

    tasks.push(new_task);
}

fn add_new_task(tasks: &mut Vec<Task>)
{
    println!("Enter new task description: ");
    let mut new_task_desc = String::new();
    io::stdin()
    .read_line(&mut new_task_desc)
    .expect("Failed to read new_task_desc");

    add_task(tasks, new_task_desc.trim().to_string())
}

fn actual_complete_task(tasks: &mut [Task], index: usize)
{
    match tasks.get_mut(index)
    {
        Some(task) => task.complete(),
        None => println!("Invalid task index"),
    }
}

fn complete_task(tasks: &mut [Task])
{
    println!("Enter selected task's index: ");
    let mut selected_task = String::new();
    io::stdin()
    .read_line(&mut selected_task)
    .expect("Failed to read selected_task");

    let index = match selected_task.trim().parse::<usize>()
    {
        Ok(index) => index,
        Err(_) => {
            println!("Expetced a valid number");
            return;
        }
    };

    actual_complete_task(tasks, index)
}

fn main()
{
    let mut tasks: Vec<Task> = vec![
        Task 
        {
            description: String::from("task 1"),
            completed: false,
        },
        Task 
        {
            description: String::from("task 2"),
            completed: true,
        },
    ];

    loop
    {
        println!("\nChoose an option:");
        println!("1. List tasks");
        println!("2. Add task");
        println!("3. Complete task");
        println!("4. Exit");
        
        let mut input = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input!");

        let index = input.trim().parse::<usize>()
                    .expect("Not a number");

        match index 
        {
            1 => list_tasks(&tasks),
            2 => add_new_task(&mut tasks),
            3 => complete_task(&mut tasks),
            4 => break,
            _ => println!("Invalid option"),
        }
    }
}