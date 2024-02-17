use std::path::PathBuf;

use blue_engine::header::{Renderer, ObjectStorage, Window};
use blue_engine_egui::{self, egui::Context};

use crate::{emojis::Emojis, radio_options::{ViewModes, object_type::ObjectType, ObjectMouseMovement}};

use self::project_config::ProjectConfig;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct D3Labels
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl D3Labels
{

    fn init(init: f32) -> Self
    {
        Self
        {
            x: init,
            y: init,
            z: init,
        }
    }
    pub fn elements(&mut self) -> [(&mut f32, u8); 3]
    {
        return [(&mut self.x, b'x'), (&mut self.y, b'y'), (&mut self.z, b'z')];
    }
}

// i.e. is it color that has changed, position etc!
#[derive(Debug)]
pub enum WhatChanged{Color, Position, Rotation}


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Project
{
    pub name        : String,
    pub dir         : String,
    pub game_type   : crate::radio_options::GameTypeDimensions,
    pub status      : bool,
}
impl Project
{
    pub fn init() -> Self
    {
        Self
        {
            name        : String::new(),
            dir         : String::new(),
            game_type   : crate::radio_options::GameTypeDimensions::D2,
            status      : false,
        }
    }
    pub fn change_choice(list: &mut [Self], choice_true: u8)
    {
        for (i, item) in list.iter_mut().enumerate()
        {
            if i as u8 == choice_true
            {
                item.status = true;
            }
            else
            {
                item.status = false;
            }
        }
    }
    pub fn selected_dir(list: &[Self]) -> String
    {
        let mut selected_dir = String::new();

        for item in list.iter()
        {
            if item.status == true
            {
                selected_dir.push_str(&format!("{}", item.dir));
                break;
            }
        }
        return selected_dir;
    }
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

    // Declaring variables/structures
    pub struct WindowSize
    {
        pub x           : f32,
        pub y           : f32,
    }
    impl WindowSize
    {
        pub fn init(window: &Window) -> Self
        {
            Self
            {
                x       : window.inner_size().width as f32,
                y       : window.inner_size().height as f32,
            }
        }
        pub fn return_tuple(&self) -> (f32, f32)
        {
            return (self.x, self.y);
        }
    }
/*
pub struct EditorModes
{
    pub projects        :   (bool, bool /*"New Project" scene window*/,
                        (bool /*2.0 Create new project with "cargo new" (checkbox)*/, String /*2.1 Label for <project_name>*/),
                        (bool /*3 Window for delete project*/, bool /*Delete entire project dir (checkbox)*/),
                        ),
    //main            : (bool, [bool;2]),
    //pub main            : (bool, ViewModes, bool /*Create new object window*/),
    pub main            : (bool, bool /*Create new object window*/),
}
*/
// Functions like powerobject in powerbuilder but for rust
// Contains all the args used for the game editor, (not game engine). scene will be passed in outside of this

// Invoked via shift+A

pub struct MouseFunctions
{
    pub is_right_clicked        : bool, // Has it been right clicked
    pub object_type_captured    : Option<ObjectType>,
    pub captured_coordinates    : (f32, f32), // Captures the coordinates at any given time, can be even used with difference between object and mouse
    pub object_mouse_movement   : Option<ObjectMouseMovement>, // grab to move the object etc
}

pub struct GameEditorArgs<'a>
{
    pub filepaths: &'a mut FilePaths,
    pub string_backups: &'a mut StringBackups,
    pub emojis: &'a Emojis,
    pub widget_functions: &'a mut WidgetFunctions,
    pub project_config: &'a mut ProjectConfig,
    pub current_project_dir: &'a mut String,
    //pub editor_modes: &'a mut EditorModes,
    pub window_size: &'a WindowSize,
    pub mouse_functions: &'a mut MouseFunctions,
    pub viewmode: &'a mut ViewModes,
    pub previous_viewmode: &'a mut ViewModes,
    pub enable_shortcuts: &'a mut bool,
}


// Will contain stuff from closures in order to reduce arguments passed in as it is pain in the ass, can't unfortunately include window due to argument in update_loop
pub struct BlueEngineArgs<'a>
{
    pub renderer: &'a mut Renderer,
    //window: &'a mut Window,
    pub objects: &'a mut ObjectStorage,
    pub input: &'a blue_engine::InputHelper,
    pub ctx: &'a Context,
}

// Used for widgets such as color as dumbass egui devs can't be fucked to have an event to determine if its closed or not
#[derive(Debug)]
pub struct WidgetFunctions
{
    //has_changed         : bool,
    pub has_changed         : Option<WhatChanged>,
    pub flameobject_old     : Option<flameobject::Settings>,
}
impl WidgetFunctions
{
    pub fn clear_everything(&mut self)
    {
        self.has_changed = None;
        self.flameobject_old = None;
    }
}
pub struct StringBackups
{
    pub texture     : String,
    pub label       : String,
}
pub mod flameobject
{
    use crate::radio_options::object_type::{ObjectType, shape::{self, Shape3D}, self};
    use super::D3Labels;

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct Flameobject
    {
        pub id          : u16,
        pub visible     : bool,
        pub selected    : bool,
        //label       : (String, issues::Issues),
        pub settings    : Settings,
    }
    impl Flameobject
    {
        pub fn init(id: u16, object_type: Option<ObjectType>) -> Self
        {
            Self
            {
                id,
                visible     : true,
                selected    : true,
                //label       : (format!("Object {id}"), issues::Issues::init()),

                settings    : Settings::init(id, object_type),
            }
        }
        pub fn copy(&self) -> Self
        {
            Self
            {
                id          : self.id,
                visible     : self.visible,
                selected    : self.selected,
                settings    : self.settings.clone(),
            }
        }
        pub fn change_choice(list: &mut [Self], choice_true: u16)
        {
            for (i, item) in list.iter_mut().enumerate()
            {
                if i as u16 == choice_true
                {
                    item.selected = true;
                }
                else
                {
                    item.selected = false;
                }
            }
        }
        // When user deletes the objects, we need to re calculate ids, Most likely we will fuck this off in the near future
        pub fn recalculate_id(list: &mut [Self])
        {
            for (i, item) in list.iter_mut().enumerate()
            {
                item.id = i as u16;
            }
        }
        // Going to fuck this off as well
        pub fn get_available_id(list: &mut [Self]) -> u16
        {
            let mut lowest_number: u16 = 0; // For unused lowest number used as ID
            let mut found_number = false;
            let len = list.len();

            if len == 0
            {
                return lowest_number;
            }

            while found_number == false
            {
                for (i, item) in list.iter().enumerate()
                {
                    if item.id == lowest_number
                    {
                        lowest_number += 1;
                        break;
                    }
                    // Last element and found no matching numbers
                    if i == (len - 1)
                    {
                        found_number = true;
                    }
                }
            }
            return lowest_number;
        }
        // Checks for warnings and errors for labels and assigns the Issues variables appropriately
    }
    #[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
    pub struct Settings
    {
        pub label               : String, // "Object 0", "Object 1" etc
        pub blueprint_key       : Option<(String, bool)>, // Used as key to modify all objects that has this key and can it get affected by future blueprint saves
        //pub object_type         : [bool; 3],
        pub object_type         : ObjectType,
        //position            : [object_settings::three_d_lables::Fields; 3],
        //pub position            : [f32; 3],
        pub position            : D3Labels,
        pub size                : D3Labels,
        pub rotation            : D3Labels,
        pub texture             : Texture,
        //texture             : [String; 3],
        pub color               : [f32; 4],
    }
    impl Settings
    {
        pub fn init(id: u16, object_type: Option<ObjectType>) -> Self
        {
            use crate::radio_options::object_type::{light, shape};
            //let position = [0f32; 3];
            //const EMPTY: String = String::new();
            let object_type =
            {
                if object_type != None
                {
                    match object_type.unwrap()
                    {
                        ObjectType::Light(lights) => match lights
                        {
                            light::Light::Direction => ObjectType::Light(light::Light::Direction),
                        }
                        ObjectType::Shape(dimensions) => match dimensions
                        {
                            shape::Dimension::D2(shapes) => match shapes
                            {
                                shape::Shape2D::Square => ObjectType::Shape(shape::Dimension::D2(shape::Shape2D::Square)),
                                shape::Shape2D::Triangle => ObjectType::Shape(shape::Dimension::D2(shape::Shape2D::Triangle)),
                                shape::Shape2D::Line => ObjectType::Shape(shape::Dimension::D2(shape::Shape2D::Line)),
                            }
                            shape::Dimension::D3(shapes) => match shapes
                            {
                                shape::Shape3D::Cube => ObjectType::Shape(shape::Dimension::D3(shape::Shape3D::Cube)),
                            }
                        }
                        ObjectType::Empty => ObjectType::Empty,
                    }
                }
                else
                {
                    ObjectType::Shape(shape::Dimension::D2(shape::Shape2D::Square))
                }
            };

            Self
            {
                label               : format!("Object {id}"),
                blueprint_key       : None,
                //object_type         : [true /*Square*/, false /*Triangle*/, false /*Line*/],
                object_type,         //: ObjectType::Shape(shape::Dimension::D2(shape::Shape2D::Square)),
                //position            : [0f32; 3],
                position            : D3Labels::init(0f32),
                size                : D3Labels::init(30f32),
                rotation            : D3Labels::init(0f32),
                //texture             : [EMPTY; 3],
                texture             : Texture::init(),
                color               : [1f32; 4],
            }
        }
    
    }

    #[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
    pub struct Texture
    {
        pub file_location   : String,
        //pub mode            : [bool; 3]
        pub mode            : crate::radio_options::Texture,
    }
    impl Texture
    {
        pub fn init() -> Self
        {
            Self
            {
                file_location   : String::new(),
                //mode            : [true /*Clamp*/, false /*Triangle*/, false /*Line*/],
                mode            : crate::radio_options::Texture::Clamp,
            }
        }
    }
}

pub mod scene
{
    use crate::radio_options::GameTypeDimensions;
    use crate::structures::flameobject::Flameobject;
    use crate::undo_redo;

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct Scene
    {
        pub id                  : u16,
        pub label               : String,
        pub flameobject_selected_parent_idx : u16,
        pub flameobject_highest_id  : u16,  // Highest id number that is used
        pub selected            : bool,
        pub settings            : Settings,
        pub undo_redo           : undo_redo::UndoRedo,
        pub flameobjects        : Vec<Flameobject>,
    }
    impl Scene
    {
        pub fn init(id: u16) -> Self
        {
            Self
            {
                id,
                label               : format!("Scene {id}"),
                flameobject_highest_id  : 0,
                flameobject_selected_parent_idx : 0,
                selected            : true,
                settings            : Settings::default(),
                undo_redo           : undo_redo::UndoRedo{actions: Vec::new(), current_idx: (None, true)},
                flameobjects        : Vec::new(),
            }
        }
        pub fn change_choice(list: &mut [Self], choice_true: usize)
        {
            for (i, item) in list.iter_mut().enumerate()
            {
                if i == choice_true
                {
                    item.selected = true;
                }
                else
                {
                    item.selected = false;
                }
            }
        }
        // When user deletes the scenes, we need to re calculate ids
        pub fn recalculate_id(list: &mut  [Self])
        {
            for (i, item) in list.iter_mut().enumerate()
            {
                item.id = i as u16;
            }
        }
    }
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct Settings
    {
        pub background_color        : u32,
        pub high_power_mode         : bool,
        pub game_type_dimensions    : GameTypeDimensions,
        pub undo_redo_length        : u16,
    }
    impl Settings
    {
        pub fn default() -> Self
        {
            Self
            {
                background_color        : 0x4d4d4d,         // Similar to Godot's background color for 2D
                high_power_mode         : true,
                game_type_dimensions    : GameTypeDimensions::D2,
                undo_redo_length        : 36,
            }
        }
    }
}

/*
pub mod loaded_scene
{
    use super::{scene::Scene, flameobject::Flameobject};
    
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct LoadedScene
    {
        pub scene           : Scene,
        pub flameobjects    : Vec<Flameobject>,
    }
    impl LoadedScene
    {
        pub fn init() -> Self
        {
            Self
            {
                scene           : Scene::init(0),
                flameobjects    : Vec::new(),
            }
        }
    }
}
*/

// Individual project info, gets saved in individual project info
pub mod project_config
{
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct ProjectConfig
    {
        pub last_scene_filepath          : String,
    }
    impl ProjectConfig
    {
        pub fn init() -> Self
        {
            Self
            {
                last_scene_filepath      : String::new(),
            }
        }
    }
}