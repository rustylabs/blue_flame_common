pub mod mapper;
pub mod db;
pub mod object_actions;

pub struct StandardFields
{
    pub selected    : bool,
    pub label       : String,
}
impl StandardFields
{
    pub fn is_selected(&self) -> bool
    {
        return self.selected;
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Texture
{
    pub file_location   : String,
    pub mode            : [bool; 3]
}
impl Texture
{
    pub fn init() -> Self
    {
        Self
        {
            file_location   : String::new(),
            mode            : [true /*Clamp*/, false /*Triangle*/, false /*Line*/],
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Scene
{
    pub id                  : u16,
    pub label               : String,
    pub dir_save            : String,
    pub selected            : bool,
}
impl Scene
{
    pub fn init(id: u16) -> Self
    {
        Self
        {
            id,
            label               : format!("Scene {id}"),
            dir_save            : format!(""),
            selected            : true,
        }
    }
    // Returns full filepath of saved db of Scene for &str args
    pub fn filepath(&self) -> String
    {
        return format!("{}/{}", self.dir_save, self.label);
    }
    pub fn change_choice(list: &mut [(Self, SceneSettings)], choice_true: usize)
    {
        for (i, item) in list.iter_mut().enumerate()
        {
            if i == choice_true
            {
                item.0.selected = true;
            }
            else
            {
                item.0.selected = false;
            }
        }
    }
    // When user deletes the scenes, we need to re calculate ids
    pub fn recalculate_id(list: &mut  [(Self, SceneSettings)])
    {
        for (i, item) in list.iter_mut().enumerate()
        {
            item.0.id = i as u16;
        }
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SceneSettings
{
    pub background_color        : u32,
    pub high_power_mode         : bool,
}
impl SceneSettings
{
    pub fn default() -> Self
    {
        Self
        {
            background_color        : 0x4d4d4d,         // Similar to Godot's background color for 2D
            high_power_mode         : true,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Flameobject
{
    pub id          : u16,
    pub visible     : bool,
    pub selected    : bool,
    //label       : (String, issues::Issues),
    pub label       : String // "Object 0", "Object 1" etc
}
impl Flameobject
{
    pub fn init(id: u16) -> Self
    {
        Self
        {
            id,
            visible     : true,
            selected    : true,
            //label       : (format!("Object {id}"), issues::Issues::init()),
            label       : format!("Object {id}"),
        }
    }
    pub fn change_choice(list: &mut [(Self, FlameobjectSettings)], choice_true: u16)
    {
        for (i, item) in list.iter_mut().enumerate()
        {
            if i as u16 == choice_true
            {
                item.0.selected = true;
            }
            else
            {
                item.0.selected = false;
            }
        }
    }
    // When user deletes the objects, we need to re calculate ids
    pub fn recalculate_id(list: &mut  [(Self, FlameobjectSettings)])
    {
        for (i, item) in list.iter_mut().enumerate()
        {
            item.0.id = i as u16;
        }
    }
    // Checks for warnings and errors for labels and assigns the Issues variables appropriately

}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FlameobjectSettings
{
    pub object_type         : [bool; 3],
    //position            : [object_settings::three_d_lables::Fields; 3],
    pub position            : [f32; 3],
    pub size                : [f32; 3],
    pub rotation            : [f32; 3],
    pub texture             : Texture,
    //texture             : [String; 3],
    pub color               : [f32; 4],
}
impl FlameobjectSettings
{
    pub fn init() -> Self
    {
        //let position = [0f32; 3];
        //const EMPTY: String = String::new();

        Self
        {
            object_type         : [true /*Square*/, false /*Triangle*/, false /*Line*/],
            position            : [0f32; 3],
            size                : [30f32; 3],
            rotation            : [0f32; 3],
            //texture             : [EMPTY; 3],
            texture             : Texture::init(),
            color               : [1f32; 4],
        }
    }
}
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



// Maps numbers with names i.e. 0 => Square etc!
/*
pub mod mapper
{
    // position means position of array/Vector
    // What shape, i.e. circle, triangle etc
    pub fn object_type(position: usize) -> &'static str
    {
        let shapes: &[&'static str] = &["Square", "Triangle", "Line"];
        return shapes[position];
    }
    // x, y, z
    pub fn three_d_lables(position: usize) -> u8
    {
        let axis = [b'x', b'y', b'z'];
        return axis[position];
    }
    pub mod texture
    {
        pub fn text(position: usize) -> &'static str
        {
            let textures: &[&'static str] = &["Clamp", "Repeat", "Mirror Repeat"];
            return textures[position];
        }
        pub fn enumm(position: usize) -> blue_engine::TextureMode
        {
            let textures = &[blue_engine::TextureMode::Clamp, blue_engine::TextureMode::Repeat, blue_engine::TextureMode::MirrorRepeat];
            return textures[position];
        }
    }
    pub fn view_mode(position: usize) -> &'static str
    {
        //let mut view_modes = object_settings::radio_options::init(&["Flameobject", "Scene"]);
        let view_modes = ["Flameobject", "Scene"];
        return view_modes[position];
    }
    pub fn game_type(position: usize) -> &'static str
    {
        //let mut view_modes = object_settings::radio_options::init(&["Flameobject", "Scene"]);
        let game_types = ["2D", "3D"];
        return game_types[position];
    }
}
*/