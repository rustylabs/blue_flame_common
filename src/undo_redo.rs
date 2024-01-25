use crate::radio_options::object_type;
use blue_engine::{Renderer, ObjectStorage, Window};
use crate::structures::flameobject::{self, Flameobject};
use crate::EditorSettings;


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Action
{
    Create((object_type::ObjectType, u16 /*id*/)),
    Update((flameobject::Settings /*Old*/, flameobject::Settings /*New*/, u16 /*id*/)),
    Delete(Flameobject),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UndoRedo
{
    pub actions         : Vec<Action>,
    //pub current_idx     : u16,
    pub current_idx     : (Option<u16>, bool /*If executing redo for the first time, this will be true, then false*/),
}
impl UndoRedo
{
    // If exeeded length_size then pop from begginning
    fn pop_from_stack_determine(&mut self, editor_settings: &EditorSettings)
    {
        if self.actions.len() >= editor_settings.undoredo_bufsize as usize
        {
            self.actions.remove(0);
            //println!("self.actions: {:?}", self.actions);
        }
    }
    pub fn save_action(&mut self, action: Action, editor_settings: &EditorSettings)
    {
        self.current_idx.1 = true;
        self.pop_from_stack_determine(editor_settings);

        // If we have gone back and we are then adding new stuff, then pop everything ahead before adding
        if let Some(current_idx) = self.current_idx.0
        {
            if self.actions.len() > 0 && (current_idx < self.actions.len() as u16 - 1)
            {
                //println!("(self.actions.len() as u16 - 1) - self.current_idx = {}\tself.actions.len(): {}", (self.actions.len() as u16 - 1) - self.current_idx, self.actions.len());
                for _i in 0..(self.actions.len() as u16 - 1) - current_idx
                {
                    self.actions.pop();
                    //println!("iteration for popping undoredo: {}", _i);
                }
            }
        }

        match action
        {
            Action::Create(values) =>
            {
                self.actions.push(Action::Create(values));
            }
            Action::Update(values) =>
            {
                //println!("\nvalues: {:?}\n\n--------", values);
                self.actions.push(Action::Update(values));
                //println!("len: {}, values: {:?}", self.actions.len(), self.actions);
            }
            Action::Delete(values) =>
            {
                self.actions.push(Action::Delete(values));
                //println!("UndoRedo Delete: {:?}", self.actions);
            }
        }
        self.current_idx.0 = Some(self.actions.len() as u16 - 1);

        //println!("\n\nself.current_val: {:?}\n\n\nself.actions: {:?}", self.current_val, self.actions);
        //println!("self.actions.len(): {}\t\teditor_settings.undoredo_bufsize: {}", self.actions.len(), editor_settings.undoredo_bufsize);
        //println!("self.current_idx: {}", self.current_idx);
    }
    // When user presses ctrl+Z    // 
    // Read the current idx and then go back is how it should work
    pub fn undo(&mut self, flameobjects: &mut Vec<flameobject::Flameobject>,
        project_dir: &str, renderer: &mut Renderer, objects: &mut ObjectStorage, window: &Window)
    {
        //println!("self.actions.len(): {}, flameobjects.len(): {}", self.actions.len(), flameobjects.len());
        // Prevent buffer overflow; If no more undos remaining, return
        if self.actions.len() <= 0
        {
            return;
        }
        if let None = self.current_idx.0
        {   
            return;
        }
        self.current_idx.1 = true;
        //println!("\n\nself.current_val: {:?}\n\n\nself.actions: {:?}", self.current_val, self.actions);
        // Do the current idx action, then go back
        let mut make_current_idx_none = false;
        if let Some(ref mut current_idx) = self.current_idx.0
        {
            // If current_idx > actions then reduce it by 1 otherwise overflow would happen, this is caused by redo incrementing it again by 1 right at the end
            if *current_idx > self.actions.len() as u16 - 1
            {
                *current_idx -= 1;
            }
            match &self.actions[*current_idx as usize]
            {
                // Delete
                Action::Create((_, id)) =>
                {
                    // Prevent out of bounds issue
                    if flameobjects.len() <= 0
                    {
                        return;
                    }
                    let mut remove_idx: usize = 0;
                    for (i, flameobject) in flameobjects.iter_mut().enumerate()
                    {
                        if flameobject.id == *id
                        {
                            remove_idx = i;
                            break;
                        }
                    }
                    crate::object_actions::delete_shape(&flameobjects[remove_idx].settings.label, objects);
                    flameobjects.remove(remove_idx);
                }
                Action::Update(values) =>
                {
                    // Prevent out of bounds issue
                    if flameobjects.len() <= 0
                    {
                        return;
                    }
                    // Put the old value into flameobject
                    for flameobject in flameobjects.iter_mut()
                    {
                        if flameobject.id == values.2
                        {
                            crate::object_actions::delete_shape(&flameobject.settings.label, objects);
                            flameobject.settings = values.0.clone();
                            crate::object_actions::create_shape(&flameobject.settings, project_dir, renderer, objects, window);
                            break;
                        }
                    }
                }
                // Recreate the deleted object
                Action::Delete(values) =>
                {
                    flameobjects.push(values.copy());
                    crate::object_actions::create_shape(&flameobjects[flameobjects.len() - 1].settings, project_dir, renderer, objects, window);
                }
            }
            if *current_idx > 0
            {
                *current_idx -= 1;
            }
            else
            {
                make_current_idx_none = true;
            }
        }
        if make_current_idx_none == true
        {
            self.current_idx.0 = None;
        }


        //println!("undo self.current_idx: {}", self.current_idx);
        
    }
    pub fn redo(&mut self, flameobjects: &mut Vec<flameobject::Flameobject>, project_dir: &str, renderer: &mut Renderer, objects: &mut ObjectStorage, window: &Window)
    {
        //println!("self.actions: {:?}", self.actions);

        if self.actions.len() <= 0 {return;}

        // If we have already incremented do not increment
        //let mut dont_increment = false;


        // if current_idx is not behind the length then we can redo
        // if self.current_idx < self.actions.len() as u16 - 1 {return;}
        // If there is something in self.current_val then we can redo something
        if let Some(current_idx) = self.current_idx.0
        {
            // If nothing else to redo then return
            if current_idx > self.actions.len() as u16 - 1
            {
                return;
            }
        }

        // If we are executing redo for the first time, make this false and if we are doing a create or delete on the current_idx, then increment by 1
        if self.current_idx.1 == true
        {
            if let Some(ref mut current_idx) = self.current_idx.0
            {
                match self.actions[*current_idx as usize]
                {
                    Action::Create(_) => *current_idx += 1,
                    Action::Delete(_) => *current_idx += 1,
                    Action::Update(_) => {},
                }
            }
            // If its None, then we need to assign it to Some(0) so we can get started to redo
            else if let None = self.current_idx.0
            {
                self.current_idx.0 = Some(0);
            }
            self.current_idx.1 = false;
            //dont_increment = true;
        }




        // If its None for current_idx start from position 0 then
        else if let None = self.current_idx.0
        {
            self.current_idx.0 = Some(0);
        }

        //println!("self.actions: {:?}", self.actions);
        // Get previous action and redo on that and then increment current_idx by 1
        if let Some(ref mut current_idx) = self.current_idx.0
        {
            match &self.actions[*current_idx as usize]
            {
                Action::Create(values) =>
                {
                    flameobjects.push(flameobject::Flameobject::init(values.1, Some(values.0)));
                    crate::object_actions::create_shape(&flameobjects[flameobjects.len() - 1].settings, project_dir, renderer, objects, window);
                }
                Action::Update(values) =>
                {
                    for flameobject in flameobjects.iter_mut()
                    {
                        if flameobject.id == values.2
                        {
                            crate::object_actions::delete_shape(&flameobject.settings.label, objects);
                            flameobject.settings = values.1.clone();
                            //string_backups.label = flameobjects[values.2 as usize].settings.label.clone();
                            crate::object_actions::create_shape(&flameobject.settings, project_dir, renderer, objects, window);
                        }
                    }
                }
                // Delete the object
                Action::Delete(values) =>
                {
                    crate::object_actions::delete_shape(&values.settings.label, objects);
                    let mut remove_idx: usize = 0;
                    for (i, flameobject) in flameobjects.iter_mut().enumerate()
                    {
                        if flameobject.id == values.id
                        {
                            remove_idx = i;
                        }
                    }
                    flameobjects.remove(remove_idx);
                }
            }
            *current_idx += 1;
            /*
            if dont_increment == false
            {
                *current_idx += 1;
            }
            */
        }

        

    }
    pub fn clear_buffer(&mut self)
    {
        self.actions = Vec::new();
        self.current_idx = (None, true);
        println!("undo_redo buf is cleared!: {:?}", self.actions);
    }
}

