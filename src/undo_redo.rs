use crate::radio_options::object_type;
use blue_engine::{primitive_shapes::{triangle, square}, Renderer, ObjectSettings, ObjectStorage, Window};
use serde::de::value;
use crate::structures::flameobject;
use crate::structures::StringBackups;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Action
{
    Create(object_type::ObjectType),
    Update((flameobject::Settings, u16 /*flameobject_selected_parent_idx*/)),
    Delete((u16 /*flameobject_selected_parent_idx*/, Vec<(flameobject::Flameobject, u16 /*index*/)>)),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UndoRedo
{
    pub length_size     : u16,      // This is the limit of how many vector elements you are allowed to store before poping from the beggining
    pub actions         : Vec<Action>,
    pub current_idx     : u16,
}
impl UndoRedo
{
    // If exeeded length_size then pop from begginning
    fn pop_from_stack_determine(&mut self)
    {
        if self.actions.len() >= self.length_size as usize
        {
            self.actions.remove(0);
        }
    }
    pub fn save_action(&mut self, action: Action)
    {
        self.pop_from_stack_determine();
        // If we have gone back and we are then adding new stuff, then pop everything ahead before adding
        if self.actions.len() > 0 && (self.current_idx < self.actions.len() as u16 - 1)
        {
            //println!("(self.actions.len() as u16 - 1) - self.current_idx = {}\tself.actions.len(): {}", (self.actions.len() as u16 - 1) - self.current_idx, self.actions.len());
            for _i in 0..(self.actions.len() as u16 - 1) - self.current_idx
            {
                self.actions.pop();
                //println!("iteration for popping undoredo: {}", _i);
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
                self.actions.push(Action::Update(values));
                //println!("len: {}, values: {:?}", self.actions.len(), self.actions);
            }
            Action::Delete(values) =>
            {
                self.actions.push(Action::Delete(values));
                //println!("UndoRedo Delete: {:?}", self.actions);
            }
        }
        self.current_idx = self.actions.len() as u16 - 1;
        //println!("self.current_idx: {}", self.current_idx);
    }
    // When user presses ctrl+Z
    pub fn undo(&mut self, flameobjects: &mut Vec<flameobject::Flameobject>, string_backups: &mut StringBackups, flameobject_selected_parent_idx: &mut u16,
        project_dir: &str, renderer: &mut Renderer, objects: &mut ObjectStorage, window: &Window)
    {
        //println!("undo called!");

        let len = self.actions.len();
        // Prevent buffer overflow; If no more undos remaining, return
        if len <= 0 || flameobjects.len() <= 0
        {
            return;
        }
        // Get previous action and undo on that
        match &self.actions[self.current_idx as usize]
        {
            Action::Create(_) =>
            {
                let flameobjects_len = flameobjects.len();
                if flameobjects_len > 0
                {
                    crate::object_actions::delete_shape(&flameobjects[flameobjects_len - 1].settings.label, objects);
                }
                else
                {
                    return;
                }
                
                flameobjects.pop();
                if flameobjects.len() > 0
                {
                    *flameobject_selected_parent_idx = flameobjects.len() as u16 - 1
                }
                else
                {
                    *flameobject_selected_parent_idx = 0;
                }
                
            }
            Action::Update(values) =>
            {
                //println!("undo Update called!");
                crate::object_actions::delete_shape(&flameobjects[values.1 as usize].settings.label, objects);
                flameobjects[values.1 as usize].settings = values.0.clone();
                string_backups.label = flameobjects[values.1 as usize].settings.label.clone();
                crate::object_actions::create_shape(&flameobjects[values.1 as usize].settings, project_dir, renderer, objects, window);
            }
            Action::Delete(values) =>
            {
                for value in values.1.iter().rev()
                {
                    let flameobjects_len = flameobjects.len();
                    // If not out of range
                    if !(flameobjects_len > 0 && value.1 > flameobjects_len as u16 - 1)
                    {
                        flameobjects.insert(value.1 as usize, value.0.copy());
                    }
                    else
                    {
                        flameobjects.push(value.0.copy());
                    }
                    crate::object_actions::create_shape(&flameobjects[value.1 as usize].settings, project_dir, renderer, objects, window);
                }
                *flameobject_selected_parent_idx = values.0;
            }
        }
        if self.current_idx > 0
        {
            self.current_idx -= 1;
        }
        //println!("undo self.current_idx: {}", self.current_idx);
        
    }
    pub fn redo(&mut self)
    {
        println!("redo called!");
    }
    pub fn clear_buffer(&mut self)
    {
        self.actions = Vec::new();
        self.current_idx = 0;
        println!("undo_redo buf is cleared!: {:?}", self.actions);
    }
}
