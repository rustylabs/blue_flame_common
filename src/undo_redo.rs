use crate::radio_options::object_type;
use crate::structures::flameobject;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Action
{
    Create(object_type::ObjectType),
    Update(flameobject::Flameobject),
    Delete(Vec<flameobject::Flameobject>),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UndoRedo
{
    pub length_size     : u16,
    pub actions         : Vec<Action>,
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
        match action
        {
            Action::Create(values) =>
            {
                self.actions.push(Action::Create(values));
                println!("UndoRedo: {:?}", self.actions);
            }
            Action::Update(values) =>
            {
                self.actions.push(Action::Update(values));
            }
            Action::Delete(values) =>
            {
                self.actions.push(Action::Delete(values));
                println!("UndoRedo Delete: {:?}", self.actions);
            }
        }
    }
    // When user presses ctrl+Z
    pub fn undo(&mut self)
    {
        
    }
}
