// Gets linked from scene as vector

use crate::radio_options::object_type::ObjectType;
use super::D3Labels;

// e.g. sprite animation for 2D
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub enum Shape2D3DSepcificSettings
{
    D2(shape_2d_3d_specific_settings::shape_2d_settings::Shape2DSettings),
    D3(shape_2d_3d_specific_settings::shape_3d_settings::Shape3DSettings),
}


pub mod shape_2d_3d_specific_settings
{
    use super::*;
    pub mod shape_2d_settings
    {
        use super::*;
        #[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
        pub struct AnimatedSprites
        {
            pub sprites: Vec<Texture>,
            pub animation_speed: f32,
        }
        impl AnimatedSprites
        {
            pub fn init() -> Self
            {
                Self
                {
                    sprites: vec![Texture::init()],
                    animation_speed: 5f32,
                }
            }
        }

        #[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
        pub struct Shape2DSettings
        {
            pub animated_sprites: Option<AnimatedSprites>,
        }
        impl Shape2DSettings
        {
            pub fn init() -> Self
            {
                Self
                {
                    animated_sprites: None,
                }
            }
        }
    }
    pub mod shape_3d_settings
    {
        #[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
        pub struct Shape3DSettings
        {
    
        }
    }

}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Flameobject
{
    pub id: u16,
    pub visible: bool,
    pub selected: bool,
    //label       : (String, issues::Issues),
    pub settings: Settings,
    pub children: Vec<Self>,
}
impl Flameobject
{
    pub fn init(id: u16, object_type: Option<ObjectType>, is_blueprint: bool,) -> Self
    {
        Self
        {
            id,
            visible: true,
            selected: true,
            //label       : (format!("Object {id}"), issues::Issues::init()),

            settings: Settings::init(id, object_type, is_blueprint),
            children: Vec::new(),
        }
    }
    /*
    pub fn copy(&self) -> Self
    {
        let children;
        if let Some(ref value) = self.children
        {
            children = Some(value.clone());
        }
        else
        {
            children = None;
        }

        Self
        {
            id: self.id,
            visible: self.visible,
            selected: self.selected,
            settings: self.settings.clone(),
            children,
        }
    }
    */
    pub fn change_choice(flameobjects: &mut[Self], change_choice_id: u16)
    {
        for flameobject in flameobjects.iter_mut()
        {
            if flameobject.id == change_choice_id
            {
                flameobject.selected = true;
            }
            else
            {
                flameobject.selected = false;
            }

            Self::change_choice(&mut flameobject.children, change_choice_id);
        }
    }

    // If multiple objects are selected, None will be returned
    pub fn get_selected_id(flameobjects: &[Self]) -> Option<u16>
    {
        let counter: u16 = 0;
        let mut id: Option<u16> = None;

        fn selection_state_counter(flameobjects: &[Flameobject], id: &mut Option<u16>, mut counter: u16) -> u16
        {
            for flameobject in flameobjects.iter()
            {

                counter = selection_state_counter(&flameobject.children, id, counter);
                if counter > 1
                {
                    return counter;
                }

                if flameobject.selected == true
                {
                    counter += 1;

                    if counter > 1
                    {
                        return counter;
                    }
                    else
                    {
                        *id = Some(flameobject.id);
                    }
                }
            }
            return counter;
        }

        if selection_state_counter(flameobjects, &mut id, counter) > 1
        {
            id = None;
        }

        return id;
    }


    pub fn get_available_id(flameobjects: &[Self], mut id_start: Option<u16>) -> u16
    {
        let mut available_id: u16 = 0;
        let mut found_available_id: bool = true;

        // If it is not recursive call, start counting from 0
        if let None = id_start {id_start = Some(0)};

        if let Some(id_start) = id_start
        {
            for mut i in id_start..u16::MAX
            {
                found_available_id = true;

                for flameobject in flameobjects.iter()
                {
                    // id from child object could not be found so assign it to i then check for rest of objects if it has id or not
                    i = Self::get_available_id(&flameobject.children, Some(i));

                    if flameobject.id == i
                    {
                        found_available_id = false;
                        break;
                    }
                }
                if found_available_id == true
                {
                    available_id = i;
                    break;
                }
            }
        }

        return available_id;
    }

    // Checks for warnings and errors for labels and assigns the Issues variables appropriately
}
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Settings
{
    pub label_key: String, // "Object 0", "Object 1" used as key for engine, remains static and does not change
    pub label: String,
    pub blueprint_key: Option<(String, bool)>, // Used as key to modify all objects that has this key and can it get affected by future blueprint saves
    //pub object_type         : [bool; 3],
    pub object_type: ObjectType,
    //position            : [object_settings::three_d_lables::Fields; 3],
    //pub position            : [f32; 3],
    pub position: D3Labels,
    pub size: D3Labels,
    pub rotation: D3Labels,
    pub texture: Texture,
    //texture             : [String; 3],
    pub color: [f32; 4],
    pub particle_systems: Option<Vec<ParticleSystem>>,
    pub shape_2d_3d_specific_settings: Shape2D3DSepcificSettings,
    pub box_colliders: Option<Vec<BoxCollider>>,
    pub linked_code: String,

}
impl Settings
{
    pub fn init(id: u16, object_type: Option<ObjectType>, is_blueprint: bool) -> Self
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

        let label_name;
        if is_blueprint == true
        {
            label_name = format!("Blueprint {id}");
        }
        else
        {
            label_name = format!("Object {id}");
        }

        Self
        {
            label_key: label_name.clone(),
            label: label_name.clone(),
            blueprint_key: None,
            //object_type         : [true /*Square*/, false /*Triangle*/, false /*Line*/],
            object_type,         //: ObjectType::Shape(shape::Dimension::D2(shape::Shape2D::Square)),
            //position            : [0f32; 3],
            position: D3Labels::init(0f32),
            size: D3Labels::init(0.5f32),
            rotation: D3Labels::init(0f32),
            //texture             : [EMPTY; 3],
            texture: Texture::init(),
            color: [1f32; 4],
            particle_systems: None,
            shape_2d_3d_specific_settings: Shape2D3DSepcificSettings::D2(shape_2d_3d_specific_settings::shape_2d_settings::Shape2DSettings::init()),
            box_colliders: None,
            linked_code: String::new(),
        }
    }

}


#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct ParticleSystem
{
    pub enabled: bool,
    pub texture: Texture,
    pub rotation: D3Labels,
    pub size: D3Labels,
    pub offset: D3Labels,
}
impl ParticleSystem
{
    pub fn init() -> Self
    {
        Self
        {
            enabled: false,
            texture: Texture::init(),
            rotation: D3Labels::init(0f32),
            size: D3Labels::init(0f32),
            offset: D3Labels::init(0f32),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct BoxCollider
{
    pub enabled: bool,
    pub is_trigger: bool,
    pub offset: D3Labels, // How far away from the object
    pub size: D3Labels,
    pub rotation: D3Labels,
    pub mimic_object: bool, // If enabled, you cannot change size or rotation and will use the same rotation and size as the object 
}
impl BoxCollider
{
    pub fn init(size: D3Labels, rotation: D3Labels) -> Self
    {
        Self
        {
            enabled: false,
            is_trigger: false,
            offset: D3Labels::init(0f32),
            size,
            rotation,
            mimic_object: true,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Texture
{
    pub file_location: String,
    //pub mode            : [bool; 3]
    pub mode: crate::radio_options::Texture,
}
impl Texture
{
    pub fn init() -> Self
    {
        Self
        {
            file_location: String::new(),
            //mode            : [true /*Clamp*/, false /*Triangle*/, false /*Line*/],
            mode: crate::radio_options::Texture::Clamp,
        }
    }
}