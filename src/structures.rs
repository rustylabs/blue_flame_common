pub mod flameobject
{
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct Flameobject
    {
        pub id          : u16,
        pub visible     : bool,
        pub selected    : bool,
        //label       : (String, issues::Issues),
        pub label       : String, // "Object 0", "Object 1" etc
        pub settings    : Settings,
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
                settings    : Settings::init(),
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
        // When user deletes the objects, we need to re calculate ids
        pub fn recalculate_id(list: &mut  [Self])
        {
            for (i, item) in list.iter_mut().enumerate()
            {
                item.id = i as u16;
            }
        }
        // Checks for warnings and errors for labels and assigns the Issues variables appropriately
    }
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct Settings
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
    impl Settings
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

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
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

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct Scene
    {
        pub id                  : u16,
        pub label               : String,
        pub selected            : bool,
        pub settings            : Settings,
    }
    impl Scene
    {
        pub fn init(id: u16) -> Self
        {
            Self
            {
                id,
                label               : format!("Scene {id}"),
                selected            : true,
                settings            : Settings::default(),
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
            }
        }
    }
}

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