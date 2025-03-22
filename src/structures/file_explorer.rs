use std::{fs::DirEntry, path::PathBuf};

// Built in file explorer for egui
#[derive(Debug)]
pub struct FileExplorerContent
{
    pub subdir_level: u16,
    pub is_collapsed: bool,
    pub selected: bool,
    pub actual_content: DirEntry,
    pub childrens_content: Option<Vec<Self>>, // Sub dir
}
// Defines where all the file paths are
pub struct FilePaths
{
    pub projects        : PathBuf, // ~/.config/blue_flame/blue_flame_common
    pub project_config  : &'static str, // <current_project_dir>/blue_flame/project.conf
    pub current_scene   : String,
    pub library         : PathBuf,
}
impl FilePaths
{
    pub fn init() -> Self
    {
        // Creating dirs
        // ~/.config.blue_flame
        let mut projects: PathBuf =  match dirs::home_dir()
        {
            Some(v)         => v,
            //None                     => {println!("Unable to obtain home dir"); PathBuf::new()}
            None                     => panic!("Unable to obtain home dir")
        };
        projects.push(".config");
        projects.push("blue_flame");

        println!("config_dir: {:?}", projects);
        match std::fs::create_dir(&projects)
        {
            Ok(_)       => println!("Config dir created succesfully in {}", projects.display()),
            Err(e)      => println!("Unable to create config dir due to {e}"),
        }

        let mut library: PathBuf =  match dirs::home_dir()
        {
            Some(v)         => v,
            None                     => {println!("Unable to obtain home dir"); PathBuf::new()}
        };
        
        library.push(".local/share/blue_flame/blue_flame_common");
        println!("library: {:?}", library);

        let project_config: &'static str = "blue_flame/project.conf";

        Self
        {
            projects,
            project_config,
            current_scene: String::new(),
            library,
        }
    }
    // Creates the folder for the project
    /*
    fn create_project_config(&self)
    {
        match std::fs::create_dir(format!("{}", self.scenes.display()))
        {
            Ok(_)       => println!("Config dir for project created succesfully in {}", self.scenes.display()),
            Err(e)      => println!("Unable to create config dir for project due to: {e}"),
        }
    }
    */
}