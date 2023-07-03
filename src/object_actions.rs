use blue_engine::{primitive_shapes::{triangle, square}, Renderer, ObjectSettings, ObjectStorage, Window};
use crate::{Flameobject, FlameobjectSettings};


// Either puts new shape or changes shape
pub fn create_shape(flameobject: &(Flameobject, FlameobjectSettings), project_dir: &str, renderer: &mut Renderer, objects: &mut ObjectStorage, window: &Window)
{
    for (i, shape) in flameobject.1.object_type.iter().enumerate()
    {
        if *shape == true
        {
            match i
            {
                0       => square(flameobject.0.label.clone(), ObjectSettings::default(), renderer, objects).unwrap(),
                1       => triangle(flameobject.0.label.clone(), ObjectSettings::default(), renderer, objects).unwrap(),
                2       => println!("todo!: line()"),

                _       => panic!("Shape number is out of bounds"),
            }
            update_shape(flameobject, project_dir, objects, window, renderer);
            break;
        }
    }

    fn update_shape(flameobject: &(Flameobject, FlameobjectSettings), project_dir: &str, objects: &mut ObjectStorage, window: &Window, renderer: &mut Renderer)
    {
        update_shape::size(flameobject, objects, window);
        update_shape::position(flameobject, objects);
        update_shape::color(flameobject, objects);
        for (i, rotation) in flameobject.1.rotation.iter().enumerate()
        {
            update_shape::rotation(&flameobject.0.label, crate::mapper::three_d_lables::enumm(i), *rotation, objects)
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
    use crate::{Flameobject, FlameobjectSettings};

    pub fn size(flameobject: &(Flameobject, FlameobjectSettings), objects: &mut ObjectStorage, window: &Window)
    {
        objects
            .get_mut(&flameobject.0.label)
            .unwrap()
            .resize(flameobject.1.size[0], flameobject.1.size[1], flameobject.1.size[2], window.inner_size());
    }
    pub fn position(flameobject: &(Flameobject, FlameobjectSettings), objects: &mut ObjectStorage)
    {
        objects
            .get_mut(&flameobject.0.label)
            .unwrap()
            .position(flameobject.1.position[0], flameobject.1.position[1], flameobject.1.position[2]);
    }
    pub fn color(flameobject: &(Flameobject, FlameobjectSettings), objects: &mut ObjectStorage)
    {
        objects
            .get_mut(&flameobject.0.label)
            .unwrap()
            .set_uniform_color(flameobject.1.color[0], flameobject.1.color[1], flameobject.1.color[2], flameobject.1.color[3])
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
    pub fn texture(flameobject: &(Flameobject, FlameobjectSettings), project_dir: &str, objects: &mut ObjectStorage, renderer: &mut Renderer)
    {
        //let mut texture_mode: Result<blue_engine::TextureMode, &'static str> = blue_engine::TextureMode::Clamp;
        let mut texture_mode: Option<blue_engine::TextureMode> = None;

        for (i, t) in flameobject.1.texture.mode.iter().enumerate()
        {
            if *t == true
            {
                texture_mode = Some(crate::mapper::texture::enumm(i));
                break;
            }
        }

        let texture = renderer.build_texture(
            "Main Player",
            //blue_engine::TextureData::Bytes(match std::fs::read(&flameobject.1.texture.file_location)
            blue_engine::TextureData::Bytes(match std::fs::read(crate::filepath_handling::relativepath_to_fullpath(&flameobject.1.texture.file_location, project_dir))
            {
                Ok(v)       => v,
                Err(e)        => {println!("TextureData error: {e}"); blue_engine::utils::default_resources::DEFAULT_TEXTURE.to_vec()}
            }),
                //std::fs::read("/mnt/Windows10/Users/Nishant/Desktop/My made programs/Projects/Game Engine/Example projects/final_test/assets/main_player.png").unwrap()),
            texture_mode.unwrap(),
        );
        
        objects
            .get_mut(&flameobject.0.label)
            .unwrap()
            .set_texture(texture.unwrap())
            .unwrap();

    }
}