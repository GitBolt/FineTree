use std::io;
use std::fmt::Debug;
use colored::*;


// FoF: Folder or File
#[derive(PartialEq)]
#[derive(Debug, Clone)]
enum FoF{
    Folder{name:String, files:Vec<FoF>},
    File{name:String, extension:String},
}

struct FileTree{
    root: String,
    tree: Vec<FoF>,
    location: FoF,
}

fn get_name(en: &FoF) -> &String {
  use FoF::*;
  match en {
    Folder{name, files:_} => name,
    File{name, extension:_} => name,
  }
}


fn get_files(e: &FoF) -> &Vec<FoF> {
  use FoF::*;
  match e {
    Folder{name:_, files} => files,
    _ => panic!("Dang it, something went wrong"),
  }
}


impl FileTree{

    fn new(root_name:String) -> Self{
      let root_name = root_name.replace("\n", "");
      FileTree{
        root: root_name.clone(),
        tree: vec![FoF::Folder{name: root_name.clone(), files: Vec::new()}],
        location: FoF::Folder{name:root_name.clone(), files:Vec::new()},
      }
    }

    fn ls(&mut self){
      let mut files = String::new();
      let mut file = Vec::new();
      for x in self.tree.iter(){
        if get_name(x) == get_name(&self.location){
          file = get_files(x).clone();
          break
        }
      }
      for f in file.iter(){
        files.push_str(&format!("{} ", get_name(&f)));
      }
      println!("{}", files)
    }

    fn mkdir(&mut self, input:String){
      let arg_vec: Vec<&str> = input.split(" ").collect();
      if arg_vec.len() < 2 || arg_vec[1] == "\n"{
        println!("mkdir: missing operhand");
      }else{
        let new_dir = FoF::Folder{name:arg_vec[1].to_string().replace("\n", ""), files:Vec::new()};
        if get_name(&self.location) == &self.root{
          self.tree.push(new_dir);
        }else{
    
          let folder_index = self.tree.iter().position(|r| r == &self.location).unwrap();
          if let FoF::Folder{name:_, files} = &mut self.tree[folder_index]{
            files.push(new_dir);
          };
        }

      }
    }

    fn touch(&mut self, input:String){
        let arg_vec: Vec<&str> = input.split(" ").collect();
        if arg_vec.len() < 2 || arg_vec[1] == "\n"{
          println!("touch: missing file operhand");
        }else{
          let file_name = arg_vec[1].to_string().replace("\n", "");
          let split = file_name.split(".").collect::<Vec<&str>>().clone();
          let new_file = FoF::File{name:file_name.to_string(), extension:split[1].to_string()};
          if get_name(&self.location) == &self.root{           
            self.tree.push(new_file);
          }else{
            let file_index = self.tree.iter().position(|r| r == &self.location).unwrap();
            if let FoF::Folder{name:_, files} = &mut self.tree[file_index]{
              files.push(new_file);
            };
          }
        }
      }

    fn cd(&mut self, input:String){
      let arg_vec: Vec<&str> = input.split(" ").collect();
      
      if arg_vec.len() < 2{
        for file in self.tree.iter(){
          if get_name(&file) == get_name(&self.location){
            self.location = file.clone();
          } 
        }
      }
      else{
        let dir_name = arg_vec[1].replace("\n", "");
        let mut check:bool = false;
        for file in self.tree.iter(){
          if get_name(&file) == &dir_name{
            check = true;
            self.location = file.clone();
          }
        }
        if check == false{
          println!("cd: {}: No such file or directory", dir_name)
        }
      }
    }
}



fn main(){

    print_beginning();

    println!("Enter the root folder name of your project:");
    let mut root_name: String = String::new();
    io::stdin().read_line(&mut root_name).ok().expect("Failed to get your input, try again.");
    let mut filetree:FileTree = FileTree::new(root_name.replace("\n", ""));

    println!(
    "{} {}.\n{}",
    "Created the project root with name".green(), 
    &filetree.root.green().bold().underline(), 
    "Continue creating rest of the file tree and type 'done' to complete.".green(),
    );

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

        } else if input.starts_with("show"){
            println!("{:?}", filetree.tree)
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
