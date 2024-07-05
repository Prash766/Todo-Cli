use clap::{ Arg, Parser};
use clap::Command;
use std::io::Write;
use std::fs::OpenOptions;


#[derive(Debug ,Parser)]
#[command(author, version, about, long_about = None)]
pub struct Todo{
    ///First arguement
    id:usize,
    ///second arguement
    task: String,
    ///Third arguement
    description : String,
    ///Fourth arguement
    deadline : String,

}

fn writing_to_file(Todo {id,task , description, deadline}: Todo)-> std::io::Result<()>{
    let mut file = OpenOptions::new()
    .create(true)
    .read(true)
    .append(true)
    .open("todo.txt")?;
if file.metadata()?.len() > 0 {
    writeln!(file)?;
  
}
    let formatted_string= format!("ID :{}\nTask: {}\nDescription: {}\nDeadline: {}" ,id, task, description, deadline );
    file.write_all(formatted_string.as_bytes())?;
  Ok(())

}

fn main() {

    let m = Command::new("todo")
    .author("Prash")
    .version("0.1.0")
    .about("A simple todo list")
    .arg(Arg::new("id").help("enter the id of the task u want to register"))
    .arg(Arg::new("task").short('t').help("Name of the task"))
    .arg(Arg::new("description").short('d').help("Description of the task"))
    .arg(Arg::new("deadline").short('e').help("Deadline of the task"))
    .get_matches();

    let id:Option<&usize>= m.get_one("id");
    let task:Option<&String>= m.get_one("task");
    let description:Option<&String>= m.get_one("description");
    let deadline:Option<&String>= m.get_one("deadline");
    let args = Todo{
        id: id.unwrap().to_owned(),
        task: task.unwrap().to_owned(),
        description: description.unwrap().to_owned(),
        deadline: deadline.unwrap().to_owned(),

    };

        let result = writing_to_file(args);
    match result {
        Ok(_) => println!("to do added"),
        Err(e) => eprintln!("error: {}", e),
    }


}
