pub mod mapper;
pub mod db;
pub mod object_actions;
pub mod radio_options;
pub mod structures;



// Deals anything to do with file paths
pub mod filepath_handling
{
    use std::{env, path::PathBuf, path::Path};
    use regex::Regex;

    use blue_engine::StringBufferTrait;
    // Translates shit like ~ and $HOME to actual paths
    fn variables_conversion(filepath: &mut String)
    {
        #[cfg(target_os = "linux")]
        let home_variables = ["~", "$HOME"];

        #[cfg(target_os = "windows")]
        let home_variables = ["%userprofile%"];

        for var in home_variables.iter()
        {
            *filepath = filepath.replace(&format!("{}", var), &format!("{}", dirs::home_dir().unwrap().display()));
        }

        #[cfg(target_os = "linux")]
        let username_variables = ["$USER"];

        #[cfg(target_os = "windows")]
        let username_variables = ["%username%"];
        
        for var in username_variables.iter()
        {
            *filepath = filepath.replace(&format!("{}", var), &format!("{}", env::var("USER").unwrap_or_else(|_| env::var("USERNAME").unwrap())));
        }

        
    }
    // Convert from fullpath to relativepath
    pub fn fullpath_to_relativepath(filepath: &str, project_dir: &str) -> String
    {
        // return if path is already relative
        if Path::is_relative(&PathBuf::from(format!("{filepath}"))) == true
        {
            return filepath.as_string();
        }
        //println!("filepath: {filepath}");

        let regex = Regex::new(r"//+").unwrap();

        let mut relativepath = PathBuf::new();
        let mut filepath = String::from(format!("{}", regex.replace_all(&filepath, "/")));
        if filepath.ends_with('/')
        {
            filepath.pop();
        }
        variables_conversion(&mut filepath);
        let filepath: Vec<&str> = filepath.split("/").collect();

        //let project_dir = env::current_dir().unwrap().display().to_string();
        let mut project_dir = String::from(format!("{}", regex.replace_all(&project_dir, "/")));
        if project_dir.ends_with('/')
        {
            project_dir.pop();
        }
        let project_dir: Vec<&str> = project_dir.split("/").collect();
    
        // Increments if both variables have the same dir to determine difference between the two
        let mut len_samedir: i16 = 0;
        let mut runonce_gobackdir = true;

    
        for (token_filepath, token_project_dir) in filepath.iter().zip(project_dir.iter().cycle())
        {
            if token_filepath == token_project_dir
            {
                len_samedir += 1;
                continue;
            }

            if runonce_gobackdir == true
            {
                let length = project_dir.len() as i16 - len_samedir;
                // If filepath is less than the project path then execute
                if length > 0
                {
                    for _ in 0..length
                    {
                        relativepath.push("..");
                    }
                }
    
                runonce_gobackdir = false;
            }
            relativepath.push(format!("{token_filepath}").as_str());
        }

        return relativepath.display().to_string();

        
    }
    // Convert from relativepath to fullpath
    pub fn relativepath_to_fullpath(filepath: &str, project_dir: &str) -> String
    {
        let mut filepath = String::from(format!("{}", filepath));
        variables_conversion(&mut filepath);

        // If filepath is already fullpath then return
        if PathBuf::from(&filepath).is_relative() == false
        {
            return filepath.to_string();
        }

        //let project_dir = env::current_dir().unwrap().display().to_string();
        let project_dir = String::from(format!("{project_dir}"));
        let project_dir: Vec<&str> = project_dir.split("/").collect();

        let filepath: Vec<&str> = filepath.split("/").collect();
    
        let mut fullpath = PathBuf::new();
    
        fullpath.push("/");
        for token in project_dir.iter()
        {
            fullpath.push(token);
        }
        for token in filepath.iter()
        {
            // To prevent extra slashes '/'
            if token == &""
            {
                continue;
            }
            else if token == &".."
            {
                fullpath.pop();
                continue;
            }
            fullpath.push(token);
        }


        return fullpath.display().to_string();
    }
}