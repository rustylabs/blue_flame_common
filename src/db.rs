// These could be levels, however you want to interpret it as
pub mod flameobjects
{
    //use super::*;
    use std::io::Read;
    use blue_engine::{ObjectStorage, Window, Renderer};
    

    const VERSION: f32 = 0.1;
    //const SAVE_FOLDER: &'static str = "blue_flame";
    //const FILE_NAME: &'static str = "project_save";

    // Destroys all shapes from the scene

    mod alter_shapes
    {
        use blue_engine::{ObjectStorage, Window, Renderer};

        pub fn delete_shapes(flameobjects: &mut Vec<(crate::Flameobject, crate::FlameobjectSettings)>, objects: &mut ObjectStorage)
        {
            // Destroys all shapes from the scene
            for object in flameobjects.iter_mut()
            {
                crate::object_actions::delete_shape(&object.0.label, objects);
            }
        }
        pub fn create_shapes(flameobjects: &mut Vec<(crate::Flameobject, crate::FlameobjectSettings)>,
        /*Game engine shit*/    renderer: &mut Renderer, objects: &mut ObjectStorage, window: &Window)
        {
            for object in flameobjects.iter()
            {
                crate::object_actions::create_shape(object, renderer, objects, window);
                /*
                for i in 0..object.1.object_type.len()
                {
                    if crate::object_settings::object_actions::create_shape(object, i, renderer, objects, window) == true
                    {
                        break;
                    }
                }
                */
            }
        }
    }


    pub fn save(flameobjects: &[(crate::Flameobject, crate::FlameobjectSettings)], file_path: &str)
    {
        let data = postcard::to_stdvec(&(VERSION, flameobjects)).unwrap();

        match std::fs::write(format!("{}", file_path), &data)
        {
            Ok(_)               => println!("File saved!"),
            Err(e)       => println!("Save error: {e}"),
        }

    }
    pub fn load(flameobjects: &mut Vec<(crate::Flameobject, crate::FlameobjectSettings)>, file_path: &str, remove_shapes: bool,
        /*Game engine shit*/ renderer: &mut Renderer, objects: &mut ObjectStorage, window: &Window)
    {

        let mut file = match std::fs::File::open(format!("{}", file_path))
        {
            Ok(d) => {println!("Flameobject: {} loaded!", file_path); d},
            Err(e) => 
                            {
                                println!("Load error on flameobjects: {}: {e}", file_path);
                                
                                if remove_shapes == true
                                {
                                    // Deletes shapes
                                    alter_shapes::delete_shapes(flameobjects, objects);
                                    
                                    // Creates new vector and pushes shit
                                    *flameobjects = Vec::new();
                                    flameobjects.push((crate::Flameobject::init(0), crate::FlameobjectSettings::init()));

                                    // Creates new shapes
                                    alter_shapes::create_shapes(flameobjects, renderer, objects, window);
                                }
                                return;
                            }
        };

        

        let mut data = Vec::new();
        match file.read_to_end(&mut data)
        {
            Ok(_)               => {},
            Err(e)       => println!("read_to_end error {e}"),
        }

        //let value: (f32, Vec<(Object, Object1)>) = match postcard::from_bytes(&file)
        let value: (f32, Vec<(crate::Flameobject, crate::FlameobjectSettings)>) = match postcard::from_bytes(&data)
        {
            Ok(d)      => d,
            Err(e)                                     => {println!("Error on load: {e}"); return;},
        };

        // Deletes shapes
        if remove_shapes == true
        {
            alter_shapes::delete_shapes(flameobjects, objects);
        }

        *flameobjects = Vec::new();
        let version = value.0;
        *flameobjects = value.1;



        // Create all the shapes after loading into memory
        alter_shapes::create_shapes(flameobjects, renderer, objects, window);

        //println!("db version Flameobject {}: {version}", scene.label);
    }
}
