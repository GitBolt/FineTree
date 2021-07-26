use std::io;
use std::collections::HashMap;
use colored::*;

struct FileTree{
    root: String,
    folders: HashMap<String, Vec<String>>,
    location: String,

}
impl FileTree{
    fn new(root_name:String) -> Self {
        FileTree{
            root: root_name.clone(),
            folders: HashMap::new(),
            location: root_name,
        }
    }
    
    fn ls(&self){
        if self.folders.get(&self.location) != None{
            let files = self.folders.get(&self.location).unwrap();
            let mut display_file = String::from("");
            for file in files{
                display_file.push_str(&format!("{} ", file.replace("\n", "")));
            }
            println!("{}", display_file.blue());
        }
    }

    fn mkdir(&mut self, input:String){
        let arg_vec: Vec<&str> = input.split(" ").collect();
        if arg_vec.len() < 2{
            println!("mkdir: missing operhand");
        }else{
            self.folders.insert(arg_vec[1].to_string(),Vec::new());
        }
    }

    fn touch(&mut self, input:String){
        let arg_vec: Vec<&str> = input.split(" ").collect();
        if arg_vec.len() < 2{
            println!("touch: missing file operand");
        }else{
            self.folders.entry(self.location.clone()).or_insert_with(Vec::new).push(arg_vec[1].to_string());
        }   
    }
    fn cd(&mut self, input:String){
        let arg_vec: Vec<&str> = input.split(" ").collect();
        if arg_vec.len() < 2{
            self.location = String::from(&self.root);
        }else{
            if self.folders.contains_key(arg_vec[1]){
                self.location = arg_vec[1].to_string();
            }
            // else if arg_vec[1] == ".."{
            //     self.location =        
            // }
            else{ 
                println!("cd: {}: No such file or directory", arg_vec[1].to_string().replace("\n", ""));
            }
        }
    }
}

fn main(){

    print_beginning();

    println!("Enter the root folder name of your project:");
    let mut root_name: String = String::new();
    io::stdin().read_line(&mut root_name).ok().expect("Failed to get your input, try again.");
    let mut filetree:FileTree = FileTree::new(root_name);
    let success_message = "Created the project root with name".to_string();
    let root_name = &filetree.root.replace("\n", "");
    let continue_message = "Continue with created rest of the file tree and type 'done' to complete.".to_string();
    println!("{} {}.\n{}", success_message.green(), root_name.green().bold().underline(), continue_message.green());
    let mut completed:bool = false;  

    while completed == false{
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).ok().expect("Failed to get your input, try again.");
         
        if input.starts_with("done"){
            println!("Completed your file structure!");
            completed = true;

        }else if input.starts_with("ls"){
            filetree.ls();

        }else if input.starts_with("mkdir"){
            filetree.mkdir(input)

        }else if input.starts_with("cd"){
            filetree.cd(input)

        }else if input.starts_with("touch"){
            filetree.touch(input)

        }
        else if input != "\n"{
            println!("{}", format!("{}: command not found", input.replace("\n", "")).red());
        }
    }
}


fn print_beginning(){
    println!("{}", r"
         _____  _                       _         _      _         __ _ _         _                 
        |  __ \(_)                     | |       (_)    | |       / _(_) |       | |                
        | |  | |_ ___  ___ ___  _ __ __| |   _ __ _  ___| |__    | |_ _| | ___   | |_ _ __ ___  ___ 
        | |  | | / __|/ __/ _ \| '__/ _` |  | '__| |/ __| '_ \   |  _| | |/ _ \  | __| '__/ _ \/ _ \
        | |__| | \__ \ (_| (_) | | | (_| |  | |  | | (__| | | |  | | | | |  __/  | |_| | |  __/  __/
        |_____/|_|___/\___\___/|_|  \__,_|  |_|  |_|\___|_| |_|  |_| |_|_|\___|   \__|_|  \___|\___|                                                                                                                                                 

        ".cyan().bold());

    println!("{}", "Commands".white().underline());
    println!("{}", "ls, mkdir, cd, touch".magenta());

}
