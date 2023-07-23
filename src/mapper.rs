// position means position of array/Vector
// What shape, i.e. circle, triangle etc

#[derive(Debug, Clone, Copy)]
pub enum ObjectType
{
    Square(&'static str),
    Triangle(&'static str),
    Line(&'static str),
}
impl ObjectType
{
    pub fn value(i: usize) -> Self
    {
        //let mut view_modes = object_settings::radio_options::init(&["Objects", "Scenes"]);
        let values = [ObjectType::Square("Square"), ObjectType::Triangle("Triangle"), ObjectType::Line("Line")];
        return values[i];
    }
}