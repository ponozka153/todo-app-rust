use std::{fs::{self, OpenOptions}, io::{self, BufRead, Read, Write}, process::Command};

fn main() {
    let path = &"./todos.txt";


    println!("Welcum to my todo app, first thing I made in Rust!\n");
    input_handle(path)

}

fn print_commands(){
    //print!("You can use commands such as:\n todos - will show all your todos\n add - will add todo\n complete - will mark a todo as completed\n exit\n\n");
    println!("You can use commands such as:");
    println!(" todos - will show all your todos");
    println!(" add - will add todo");
    println!(" complete - will mark a todo as completed");
    println!(" exit");
    println!("");
}

fn input_handle(path: &str){
    loop {
    print_commands();
    let mut input = String::new();
    println!("Choose your command: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    match input.trim().to_lowercase().as_str(){
        "todos" =>{
            clean_console();
            println!("{}\n", read_todos(path));
        }

        "add" =>{
            add_todo(path)
        }

        "complete" =>{
            complete_todo(path)
        }

        "exit" =>{
            std::process::exit(0)
        }

        _ =>{
            println!("Wrong command!")
        }
    }
}
}

fn read_todos(path: &str) -> String{

    //Jestli soubor neexistuje, vytvořit
    if let Err(_) = fs::metadata(path){
        fs::write(path, "").expect("Nepovedlo se vytvořit soubor");
    }

    //read file
    let file = fs::File::open(path).expect("Nepovedlo se otevřít soubor");
    let reader = io::BufReader::new(file);

    let mut i: u8 = 0;
    let mut result: Vec<Vec<String>> = Vec::new();
    //pro každý \n
    for line in reader.lines(){
        //Když to našlo new line bez error
        match line{
            Ok(line_content) =>{
                // 0: Text | 1
                //println!("{}: {}", i, line_content);
                //["text ", " 1"]
                let mut split: Vec<String> = line_content.split("|").map(String::from).collect();
                if split.len() >= 1 {
                    //"text", smaže to tu mezeru, nemůžu použít replace cause to by dalo do pyč první space, což nechcu, chcu last
                    split[0] = split[0][..split[0].len() - 1].to_string();
                    //"1", smaže to tu mezeru
                    split[1] = split[1].replace(" ", "");
                    //println!("{}: {:?}", i, split[1]);
                    result.push(split);
                } else {
                    println!("{}: Invalid line format, skipping", i);
                }
            }
            //error
            Err(err)=> panic!("{}", err)
        }
        i += 1
    }

    //result
    //[["text", "1"], ["text", "0"] etc...]

    let mut todos: String;

    if result.len() < 1 {
        todos = "None".to_string();
    } else {
        todos = "".to_string();
        let mut i = 1;
        for todo in &result{
            let todo_formatted: String;
            if todo[1] == "1"{
                todo_formatted = format!("{} >>> COMPLETED", todo[0])
            } else {
                todo_formatted = format!("{} >>> INCOMPLETE", todo[0])
            }
            todos = format!("{}\n{}: {}", todos, i, todo_formatted);
            i += 1
        }
    }
    todos
}

fn add_todo(path: &str){
        //Jestli soubor neexistuje, vytvořit
        if let Err(_) = fs::metadata(path){
            fs::write(path, "").expect("Nepovedlo se vytvořit soubor");
        }

        let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .expect("Nepovedlo se otevřít soubor");

        //co bylo v tom todo souboru předtím
        let mut prevcontent = String::new();
        file.read_to_string(&mut prevcontent).expect("Nepovedlo se přečíst content souboru");
        //println!("{}", prevcontent);
    
        //input reader
        let mut input = String::new();
        println!("");
        println!("What todo would you like to add?: \n");
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let content: String;
        if prevcontent.trim() == ""{
            content = format!("{} | 0", input.trim());
        } else {
            content = format!("\n{} | 0", input.trim());
        }

        file.write_all(content.as_bytes()).expect("Nepovedlo se zapsat todo do souboru");

        clean_console();
        //{:?} cause it add " before and after the value
        println!("{:?} added succesfully!", input.trim());
        println!("");
}

fn complete_todo(path: &str){
        //Jestli soubor neexistuje, vytvořit
        if let Err(_) = fs::metadata(path){
            fs::write(path, "").expect("Nepovedlo se vytvořit soubor");
        }
    
        let mut file = OpenOptions::new()
        .read(true)
        .truncate(false) //truncate == smaže to alles nahned
        .open(path)
        .expect("Nepovedlo se otevřít soubor");

        //co bylo v tom todo souboru předtím
        let mut prevcontent = String::new();
        file.read_to_string(&mut prevcontent).expect("Nepovedlo se přečíst content souboru");

        //input reader
        let mut input = String::new();
        println!("");
        println!("What number would you like to mark as completed: \n");
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let lines: Vec<&str> = prevcontent.lines().collect();
        let num: usize = input.trim().to_string().parse().unwrap();

        //-1 cause chcem aby když se písne 1, tak aby to vzalo první (ve Vecu nultou) věc
        let choosen_line = lines[num - 1];
        //println!("line: {}", choosen_line);

        let mut choosen_line_split: Vec<String> = choosen_line.split("|").map(String::from).collect();
        choosen_line_split[1] = "1".to_string();

        //cant do i: u8 cause num is a usize, said the compiler
        let mut i = 1;
        let mut connected_vec: Vec<String> = Vec::new();
        for line in &lines {
            if i == num{
                connected_vec.push(format!("{} | {}", choosen_line_split[0].trim(), choosen_line_split[1]));
            } else {
                connected_vec.push(line.to_string());
            }
            i += 1
        }

        let connected_string: String = connected_vec.join("\n");
        //println!("Connected: {:}", connected_string);

        let mut file = OpenOptions::new()
        .write(true)
        .truncate(true) //truncate == smaže to alles nahned
        .open(path)
        .expect("Nepovedlo se otevřít soubor");

        file.write_all(connected_string.as_bytes()).expect("Nepovedlo se přečíst content souboru");

        clean_console();
        println!("Marked as completed!");
        println!("");
}

fn clean_console(){
    Command::new("cmd")
    .args(&["/C", "cls"])
    .status().expect("Failed to execute cls command");
}