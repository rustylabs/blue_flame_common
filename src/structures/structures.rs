use blue_engine::{header::{ObjectStorage, Renderer, Window}, Camera};
use blue_engine_utilities::{egui::egui::{self, Context}};

use crate::radio_options::{ViewModes, object_type::ObjectType, ObjectMouseMovement};

use self::project_config::ProjectConfig;

// Categorisation of structs
use super::{file_explorer::{FileExplorerContent, FilePaths}, flameobject};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct D3Labels
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl D3Labels
{

    pub fn init(init: f32) -> Self
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
            x       : window.as_ref().unwrap().inner_size().width as f32,
            y       : window.as_ref().unwrap().inner_size().height as f32,
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
    pub widget_functions: &'a mut WidgetFunctions,
    pub project_config: &'a mut ProjectConfig,
    pub current_project_dir: &'a mut String,
    //pub editor_modes: &'a mut EditorModes,
    pub window_size: &'a WindowSize,
    pub mouse_functions: &'a mut MouseFunctions,
    pub viewmode: &'a mut ViewModes,
    pub previous_viewmode: &'a mut ViewModes,
    pub enable_shortcuts: &'a mut bool,
    pub file_explorer_contents: &'a mut (bool, Option<Vec<FileExplorerContent>>),
}


// Will contain stuff from closures in order to reduce arguments passed in as it is pain in the ass, can't unfortunately include window due to argument in update_loop
pub struct BlueEngineArgs<'a>
{
    pub renderer: &'a mut Renderer,
    //window: &'a mut Window,
    pub objects: &'a mut ObjectStorage,
    pub input: &'a blue_engine::InputHelper,
    pub ctx: &'a Context,
    pub camera: &'a mut Camera,
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


pub mod scene
{
    use crate::radio_options::GameTypeDimensions;
    //use crate::structures::flameobject::Flameobject;
    use super::flameobject::Flameobject;
    use crate::undo_redo;

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct Scene
    {
        pub id: u16,
        pub label: String,
        pub flameobject_selected_parent_idx: u16,
        pub flameobject_highest_id: u16,  // Highest id number that is used
        pub selected: bool,
        pub settings: Settings,
        pub undo_redo: undo_redo::UndoRedo,
        pub flameobjects: Vec<Flameobject>,
    }
    impl Scene
    {
        pub fn init(id: u16) -> Self
        {
            Self
            {
                id,
                label: format!("Scene {id}"),
                flameobject_highest_id: 0,
                flameobject_selected_parent_idx: 0,
                selected: true,
                settings: Settings::default(),
                undo_redo: undo_redo::UndoRedo{actions: Vec::new(), current_idx: (None, true)},
                flameobjects: Vec::new(),
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
        pub last_scene_filepath: String,
        pub camera_position: CameraPosition,
    }
    impl ProjectConfig
    {
        pub fn init() -> Self
        {
            Self
            {
                last_scene_filepath: String::new(),
                camera_position: CameraPosition::init(),
            }
        }
    }
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct CameraPosition
    {
        pub zoom: u16, // z position of camera
        pub position: [u16; 2] // x, y position
        //rotation: u16, // How much camerea is rotated
    }
    impl CameraPosition
    {
        fn init() -> Self
        {
            Self
            {
                zoom: 100,
                position: [0, 0],
            }
        }
    }
}