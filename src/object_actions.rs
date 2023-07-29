use blue_engine::{primitive_shapes::{triangle, square}, Renderer, ObjectSettings, ObjectStorage, Window};
use crate::structures::flameobject::Flameobject;
use crate::radio_options::object_type::{ObjectType, shape, light};


// Either puts new shape or changes shape
pub fn create_shape(flameobject: &Flameobject, project_dir: &str, renderer: &mut Renderer, objects: &mut ObjectStorage, window: &Window)
{
    match flameobject.settings.object_type
    {
        ObjectType::Empty => return,
        ObjectType::Shape(dimension) => match dimension
        {
            shape::Dimension::D2(shape) => match shape
            {
                shape::Shape2D::Square => square(flameobject.label.clone(), ObjectSettings::default(), renderer, objects).unwrap(),
                shape::Shape2D::Triangle => triangle(flameobject.label.clone(), ObjectSettings::default(), renderer, objects).unwrap(),
                shape::Shape2D::Line => return,
            }
            shape::Dimension::D3(shape) => match shape
            {
                shape::Shape3D::Cube => {println!("todo!: cube()"); return},
            }
        }
        ObjectType::Light(light)                 => match light
        {
            light::Light::Direction                 => return,
        }
    }
    update_shape(flameobject, project_dir, objects, window, renderer);

    fn update_shape(flameobject: &Flameobject, project_dir: &str, objects: &mut ObjectStorage, window: &Window, renderer: &mut Renderer)
    {
        update_shape::size(flameobject, objects, window);
        update_shape::position(flameobject, objects);
        update_shape::color(flameobject, objects);
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
        update_shape::texture(flameobject, project_dir, objects, renderer);
        
    }
}
// Destroys old hashmap stored in game engine
pub fn delete_shape(label: &str, objects: &mut ObjectStorage)
{
    objects
        .remove(label);
}
pub mod update_shape
{
    use blue_engine::{ObjectStorage, Window, Renderer};
    use crate::{structures::flameobject::Flameobject};

    pub fn size(flameobject: &Flameobject, objects: &mut ObjectStorage, window: &Window)
    {
        objects
            .get_mut(&flameobject.label)
            .unwrap()
            .resize(flameobject.settings.size.x, flameobject.settings.size.y, flameobject.settings.size.z, window.inner_size());
    }
    pub fn position(flameobject: &Flameobject, objects: &mut ObjectStorage)
    {
        objects
            .get_mut(&flameobject.label)
            .unwrap()
            .position(flameobject.settings.position.x, flameobject.settings.position.y, flameobject.settings.position.z);
    }
    pub fn color(flameobject: &Flameobject, objects: &mut ObjectStorage)
    {
        objects
            .get_mut(&flameobject.label)
            .unwrap()
            .set_uniform_color(flameobject.settings.color[0], flameobject.settings.color[1], flameobject.settings.color[2], flameobject.settings.color[3])
            .unwrap();
    }
    pub fn rotation(flameobject_label: &str, axis: blue_engine::RotateAxis, rotation: f32, objects: &mut ObjectStorage)
    {
        let previous_rotation = objects
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
    pub fn texture(flameobject: &Flameobject, project_dir: &str, objects: &mut ObjectStorage, renderer: &mut Renderer)
    {
        use crate::radio_options::Texture;
        
        let texture_mode = match flameobject.settings.texture.mode
        {
            Texture::Clamp          => blue_engine::TextureMode::Clamp,
            Texture::Repeat         => blue_engine::TextureMode::Repeat,
            Texture::MirrorRepeat   => blue_engine::TextureMode::MirrorRepeat,
        };

        let texture = renderer.build_texture(
            "Main Player",
            //blue_engine::TextureData::Bytes(match std::fs::read(&flameobject.1.texture.file_location)
            blue_engine::TextureData::Bytes(match std::fs::read(crate::filepath_handling::relativepath_to_fullpath(&flameobject.settings.texture.file_location, project_dir))
            {
                Ok(v)       => v,
                Err(e)        => {println!("TextureData error: {e}"); blue_engine::utils::default_resources::DEFAULT_TEXTURE.to_vec()}
            }),
                //std::fs::read("/mnt/Windows10/Users/Nishant/Desktop/My made programs/Projects/Game Engine/Example projects/final_test/assets/main_player.png").unwrap()),
            texture_mode,
        );
        
        objects
            .get_mut(&flameobject.label)
            .unwrap()
            .set_texture(texture.unwrap())
            .unwrap();

    }
}