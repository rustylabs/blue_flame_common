// Deals with changing shapes on the scene, it does NOT affect the shape that is stored on game editor's variables

use blue_engine::{primitive_shapes::{triangle, square}, Renderer, ObjectSettings, ObjectStorage, Window};
use crate::structures::{flameobject, structures::BlueEngineArgs};
use crate::radio_options::object_type::{ObjectType, shape, light};


// Either puts new shape or changes shape
pub fn create_shape(flameobject_settings: &flameobject::Settings, project_dir: &str, blue_engine_args: &mut BlueEngineArgs, window: &Window)
{
    match flameobject_settings.object_type
    {
        ObjectType::Empty => return,
        ObjectType::Shape(dimension) => match dimension
        {
            shape::Dimension::D2(shape) => match shape
            {
                shape::Shape2D::Square => square(flameobject_settings.label.clone(), ObjectSettings::default(), blue_engine_args.renderer, blue_engine_args.objects),
                shape::Shape2D::Triangle => triangle(flameobject_settings.label.clone(), ObjectSettings::default(), blue_engine_args.renderer, blue_engine_args.objects),
                shape::Shape2D::Line => return,
            }
            shape::Dimension::D3(shape) => match shape
            {
                shape::Shape3D::Cube => {println!("todo!: cube()"); return},
            }
        }
        ObjectType::Light(light) => match light
        {
            light::Light::Direction => return,
        }
    }
    update_shape(flameobject_settings, project_dir, blue_engine_args, window);

    fn update_shape(flameobject_settings: &flameobject::Settings, project_dir: &str, blue_engine_args: &mut BlueEngineArgs, window: &Window)
    {
        update_shape::size(flameobject_settings, blue_engine_args, window);
        update_shape::position(flameobject_settings, blue_engine_args);
        update_shape::color(flameobject_settings, blue_engine_args);
        for i in 0..3
        {
            /*
            update_shape::rotation(&flameobject.label,
                match mapper::ThreeDLabels::value(i) 
                {
                    ThreeDLabels::X(_, axis)       => axis,
                    ThreeDLabels::Y(_, axis)       => axis,
                    ThreeDLabels::Z(_, axis)       => axis,
                }
            , *rotation, objects)
            */
        }
        update_shape::texture(flameobject_settings, project_dir, blue_engine_args);
        
    }
}
// Destroys old hashmap stored in game engine
pub fn delete_shape(label: &str, blue_engine_args: &mut BlueEngineArgs)
{
    blue_engine_args.objects.remove(label);
    println!("delete_shape(): {}", label);
}
pub mod update_shape
{
    use blue_engine::Window;
    use crate::structures::{flameobject::{self}, structures::BlueEngineArgs};


    pub fn size(flameobject_settings: &flameobject::Settings, blue_engine_args: &mut BlueEngineArgs, window: &Window)
    {
        blue_engine_args.objects
            .get_mut(&flameobject_settings.label)
            .unwrap()
            .resize(flameobject_settings.size.x, flameobject_settings.size.y, flameobject_settings.size.z, window.as_ref().unwrap().inner_size());
    }
    pub fn position(flameobject_settings: &flameobject::Settings, blue_engine_args: &mut BlueEngineArgs)
    {
        blue_engine_args.objects
            .get_mut(&flameobject_settings.label)
            .unwrap()
            .set_position([flameobject_settings.position.x, flameobject_settings.position.y, flameobject_settings.position.z]);
            //.position(flameobject_settings.position.x, flameobject_settings.position.y, flameobject_settings.position.z);
    }
    pub fn color(flameobject_settings: &flameobject::Settings, blue_engine_args: &mut BlueEngineArgs)
    {
        blue_engine_args.objects
            .get_mut(&flameobject_settings.label)
            .unwrap()
            //.set_uniform_color(flameobject_settings.color[0], flameobject_settings.color[1], flameobject_settings.color[2], flameobject_settings.color[3])
            .set_color(flameobject_settings.color[0], flameobject_settings.color[1], flameobject_settings.color[2], flameobject_settings.color[3]);
    }
    pub fn rotation(flameobject_label: &str, axis: blue_engine::RotateAxis, rotation: f32, blue_engine_args: &mut BlueEngineArgs)
    {
        let previous_rotation = blue_engine_args.objects
            .get(flameobject_label)
            .unwrap()
            .rotation;

        

        /*
        objects
            .get_mut(flameobject_label)
            .unwrap()
            .rotate(rotation, axis);
        */
    }
    pub fn texture(flameobject_settings: &flameobject::Settings, project_dir: &str, blue_engine_args: &mut BlueEngineArgs)
    {
        use crate::radio_options::Texture;
        
        let texture_mode = match flameobject_settings.texture.mode
        {
            Texture::Clamp          => blue_engine::TextureMode::Clamp,
            Texture::Repeat         => blue_engine::TextureMode::Repeat,
            Texture::MirrorRepeat   => blue_engine::TextureMode::MirrorRepeat,
        };

        let texture = blue_engine_args.renderer.build_texture(
            "Main Player",
            //blue_engine::TextureData::Bytes(match std::fs::read(&flameobject.1.texture.file_location)
            blue_engine::TextureData::Bytes(match std::fs::read(crate::filepath_handling::relativepath_to_fullpath(&flameobject_settings.texture.file_location, project_dir))
            {
                Ok(v) => v,
                Err(_) => blue_engine::utils::default_resources::DEFAULT_TEXTURE.to_vec(),//{println!("TextureData error: {e}"); blue_engine::utils::default_resources::DEFAULT_TEXTURE.to_vec()}
            }),
                //std::fs::read("/mnt/Windows10/Users/Nishant/Desktop/My made programs/Projects/Game Engine/Example projects/final_test/assets/main_player.png").unwrap()),
            texture_mode,
        );
        
        blue_engine_args.objects
            .get_mut(&flameobject_settings.label)
            .unwrap()
            .set_texture(texture.unwrap());

    }
}