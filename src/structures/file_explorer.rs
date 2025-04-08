use std::path::PathBuf;

// Built in file explorer for egui
// Defines where all the file paths are
pub struct FilePaths
{
    pub all_projects_manager: PathBuf, // ~/.config/blue_flame/blue_flame_common // Manages all projects and what is saved
    pub current_scene: String,
    pub project_config: String, // <current_project_dir>/blue_flame/project.conf
    pub library: PathBuf,
}
impl FilePaths
{
    pub fn init() -> Self
    {
        // Creating dirs
        // ~/.config.blue_flame
        let mut all_projects_manager: PathBuf =  match dirs::home_dir()
        {
            Some(v)  => v,
            //None                     => {println!("Unable to obtain home dir"); PathBuf::new()}
            None => panic!("Unable to obtain home dir")
        };
        all_projects_manager.push(".config");
        all_projects_manager.push("blue_flame");

        println!("config_dir: {:?}", all_projects_manager);
        match std::fs::create_dir(&all_projects_manager)
        {
            Ok(_) => println!("Config dir created succesfully in {}", all_projects_manager.display()),
            Err(e) => println!("Unable to create config dir due to {e}"),
        }

        let mut library: PathBuf =  match dirs::home_dir()
        {
            Some(v)         => v,
            None                     => {println!("Unable to obtain home dir"); PathBuf::new()}
        };
        
        library.push(".local/share/blue_flame/blue_flame_common");
        println!("library: {:?}", library);

        Self
        {
            all_projects_manager,
            project_config: String::new(),
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