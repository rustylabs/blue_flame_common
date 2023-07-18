// These could be levels, however you want to interpret it as
pub mod flameobjects
{
    //use super::*;
    use std::io::Read;
    use blue_engine::{ObjectStorage, Window, Renderer};
    use crate::structures::{flameobject::Flameobject, loaded_scene::LoadedScene};
    

    const VERSION: f32 = 0.1;
    //const SAVE_FOLDER: &'static str = "blue_flame";
    //const FILE_NAME: &'static str = "project_save";

    // Destroys all shapes from the scene

    mod alter_shapes
    {
        use super::*;

        pub fn delete_shapes(flameobjects: &mut Vec<Flameobject>, objects: &mut ObjectStorage)
        {
            // Destroys all shapes from the scene
            for flameobject in flameobjects.iter_mut()
            {
                crate::object_actions::delete_shape(&flameobject.label, objects);
            }
        }
        pub fn create_shapes(flameobjects: &mut Vec<Flameobject>, project_dir: &str,
        /*Game engine shit*/    renderer: &mut Renderer, objects: &mut ObjectStorage, window: &Window)
        {
            for flameobject in flameobjects.iter()
            {
                crate::object_actions::create_shape(flameobject, project_dir, renderer, objects, window);
                /*
                for i in 0..flameobject.1.object_type.len()
                {
                    if crate::object_settings::object_actions::create_shape(flameobject, i, renderer, objects, window) == true
                    {
                        break;
                    }
                }
                */
            }
        }
    }


    pub fn save(loaded_scene: &LoadedScene, filepath: &str, project_dir: &str) -> bool
    {
        let data = postcard::to_stdvec(&(VERSION, loaded_scene)).unwrap();

        match std::fs::write(format!("{}", crate::filepath_handling::relativepath_to_fullpath(filepath, project_dir)), &data)
        {
            Ok(_)               => {println!("Scene saved!"); return true},
            Err(e)       => {println!("Scene save error: {e}"); return false},
        }

    }
    pub fn load(loaded_scene: &mut LoadedScene, project_dir: &str, filepath: &str, remove_shapes: bool,
        /*Game engine shit*/ renderer: &mut Renderer, objects: &mut ObjectStorage, window: &Window) -> bool // Making sure there was no issue with loading file due to error in filepath
    {

        let mut file = match std::fs::File::open(format!("{}", crate::filepath_handling::relativepath_to_fullpath(filepath, project_dir)))
        {
            Ok(d) => {println!("Flameobject: {} loaded!", filepath); d},
            Err(e) => 
                            {
                                println!("Load error on flameobjects: {}: {e}", filepath);
                                return false;
                                
                                /*
                                if remove_shapes == true
                                {
                                    // Deletes shapes
                                    alter_shapes::delete_shapes(&mut loaded_scene.flameobjects, objects);
                                    
                                    // Creates new vector and pushes shit
                                    loaded_scene.flameobjects = Vec::new();
                                    loaded_scene.flameobjects.push(Flameobject::init(0));

                                    // Creates new shapes
                                    alter_shapes::create_shapes(&mut loaded_scene.flameobjects, project_dir, renderer, objects, window);
                                }
                                return;
                                */
                            }
        };

        

        let mut data = Vec::new();
        match file.read_to_end(&mut data)
        {
            Ok(_)               => {},
            Err(e)       => println!("read_to_end error {e}"),
        }

        //let value: (f32, Vec<(Object, Object1)>) = match postcard::from_bytes(&file)
        let value: (f32, LoadedScene) = match postcard::from_bytes(&data)
        {
            Ok(d)      => d,
            Err(e)                  => {println!("Error on load: {e}"); return false;},
        };

        // Deletes shapes
        if remove_shapes == true
        {
            alter_shapes::delete_shapes(&mut loaded_scene.flameobjects, objects);
        }

        loaded_scene.flameobjects = Vec::new();
        let version = value.0;
        *loaded_scene = value.1;



        // Create all the shapes after loading into memory
        alter_shapes::create_shapes(&mut loaded_scene.flameobjects, project_dir, renderer, objects, window);

        //println!("db version Flameobject {}: {version}", scene.label);
        return true;
    }
}

