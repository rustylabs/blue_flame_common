use crate::radio_options::object_type;
use blue_engine::{Renderer, ObjectStorage, Window};
use serde::de::value;
use crate::structures::flameobject::{self, Flameobject};
use crate::EditorSettings;
use crate::structures::{StringBackups, scene::Scene, WidgetFunctions};


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
    pub actions         : Vec<Action>,
    //pub current_idx     : u16,
    pub current_idx     : Option<u16>,
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
        self.pop_from_stack_determine(editor_settings);
        // If we have gone back and we are then adding new stuff, then pop everything ahead before adding
        if let Some(current_idx) = self.current_idx
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
                self.actions.push(Action::Update(values));
                //println!("len: {}, values: {:?}", self.actions.len(), self.actions);
            }
            Action::Delete(values) =>
            {
                self.actions.push(Action::Delete(values));
                //println!("UndoRedo Delete: {:?}", self.actions);
            }
        }
        self.current_idx = Some(self.actions.len() as u16 - 1);
        //println!("self.actions.len(): {}\t\teditor_settings.undoredo_bufsize: {}", self.actions.len(), editor_settings.undoredo_bufsize);
        //println!("self.current_idx: {}", self.current_idx);
    }
    // When user presses ctrl+Z
    pub fn undo(&mut self, flameobjects: &mut Vec<flameobject::Flameobject>, string_backups: &mut StringBackups, flameobject_selected_parent_idx: &mut u16,
        project_dir: &str, renderer: &mut Renderer, objects: &mut ObjectStorage, window: &Window)
    {
        //println!("undo called!");

        let mut make_non_current_idx = false;

        let len = self.actions.len();
        // Prevent buffer overflow; If no more undos remaining, return
        if len <= 0 || flameobjects.len() <= 0
        {
            return;
        }
        if let Some(ref mut current_idx) = self.current_idx
        {
            
            // If current_idx is 0, then make it None so this shit doesn't execute again!s
            if *current_idx <= 0 {make_non_current_idx = true;}
            // Get previous action and undo on that
            match &self.actions[*current_idx as usize]
            {
                Action::Create(_) =>
                {
                    let flameobjects_len = flameobjects.len();
                    if flameobjects_len > 0
                    {
                        //println!("flameobjects[flameobjects_len - 1].settings.label: {}", flameobjects[flameobjects_len - 1].settings.label);
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
            if *current_idx > 0
            {
                *current_idx -= 1;
            }
        }
        if make_non_current_idx == true {self.current_idx = None};
        //println!("undo self.current_idx: {}", self.current_idx);
        
    }
    pub fn redo(&mut self, flameobjects: &mut Vec<flameobject::Flameobject>, string_backups: &mut StringBackups, flameobject_selected_parent_idx: &mut u16, widget_functions: &mut WidgetFunctions, project_dir: &str, editor_settings: &EditorSettings,
        renderer: &mut Renderer, objects: &mut ObjectStorage, window: &Window)
    {
        println!("redo called!");

        // If current_idx is None
        let mut dont_increment_current_idx = true;
        if self.current_idx == None
        {
            self.current_idx = Some(0);
            dont_increment_current_idx = false;
        }
        

        if let Some(ref mut current_idx) = self.current_idx
        {
            // if current_idx is behind the length then we can redo
            if self.actions.len() > 0 && ((*current_idx < self.actions.len() as u16 - 1) || dont_increment_current_idx == false)
            {
                if dont_increment_current_idx == true {*current_idx += 1};
                // Get previous action and undo on that
                match &self.actions[*current_idx as usize]
                {
                    Action::Create(values) =>
                    {
                        let len = flameobjects.len() as u16;
                        let id = flameobject::Flameobject::get_available_id(flameobjects);
                        //println!("id: {}", id);
                
                        flameobjects.push(flameobject::Flameobject::init(id, Some(*values)));
                        flameobject::Flameobject::change_choice(flameobjects, len);
                        *flameobject_selected_parent_idx = flameobjects.len() as u16 - 1;
                        crate::object_actions::create_shape(&flameobjects[*flameobject_selected_parent_idx as usize].settings, project_dir, renderer, objects, window);
                        string_backups.label = flameobjects[*flameobject_selected_parent_idx as usize].settings.label.clone();
                        /*
                        for (i, flameobject) in scene.flameobjects.iter().enumerate()
                        {
                            if flameobject.selected == true
                            {
                                scene.flameobject_selected_parent_idx = i as u16;
                                blue_flame_common::object_actions::create_shape(&flameobject.settings, project_dir, renderer, objects, window);
                            }
                        }
                        */
                
                        if flameobjects.len() > 0
                        {
                            widget_functions.flameobject_old = Some(flameobjects[flameobjects.len() - 1].settings.clone());
                        }
                        else
                        {
                            widget_functions.flameobject_old = None;
                        }
                    }
                    Action::Update(values) =>
                    {
                        crate::object_actions::delete_shape(&flameobjects[values.1 as usize].settings.label, objects);
                        flameobjects[values.1 as usize].settings = values.0.clone();
                        string_backups.label = flameobjects[values.1 as usize].settings.label.clone();
                        crate::object_actions::create_shape(&flameobjects[values.1 as usize].settings, project_dir, renderer, objects, window);
                    }
                    Action::Delete(_) =>
                    {

                    }
                }
            }
        }

    }
    pub fn clear_buffer(&mut self)
    {
        self.actions = Vec::new();
        self.current_idx = None;
        println!("undo_redo buf is cleared!: {:?}", self.actions);
    }
}

