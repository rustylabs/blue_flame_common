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
// x, y, z
use blue_engine::RotateAxis;
#[derive(Debug, Clone, Copy)]
pub enum ThreeDLabels
{
    X(u8, RotateAxis),
    Y(u8, RotateAxis),
    Z(u8, RotateAxis),
}
impl ThreeDLabels
{
    pub fn value(i: usize) -> Self
    {
        let values = [ThreeDLabels::X(b'x', RotateAxis::X), ThreeDLabels::Y(b'y', RotateAxis::Y), ThreeDLabels::Z(b'z', RotateAxis::Z)];
        return values[i];
    }
}
use blue_engine::TextureMode;
#[derive(Debug, Clone, Copy)]
pub enum Texture
{
    Clamp(&'static str, TextureMode),
    Repeat(&'static str, TextureMode),
    MirrorRepeat(&'static str, TextureMode),
}
impl Texture
{
    pub fn value(i: usize) -> Self
    {
        let values = [Texture::Clamp("Clamp", TextureMode::Clamp), Texture::Repeat("Repeat", TextureMode::Repeat), Texture::MirrorRepeat("MirrorRepeat", TextureMode::MirrorRepeat)];
        return values[i];
    }
}

pub mod texture
{
    use blue_engine::TextureMode;
    
    pub fn label(position: usize) -> &'static str
    {
        let textures = ["Clamp", "Repeat", "Mirror Repeat"];
        return textures[position];
    }
    pub fn enumm(position: usize) -> TextureMode
    {
        let textures = [TextureMode::Clamp, TextureMode::Repeat, TextureMode::MirrorRepeat];
        return textures[position];
    }
}
#[derive(Debug, Clone, Copy)]
pub enum ViewModes
{
    Objects(&'static str),
    Scenes(&'static str),
}
impl ViewModes
{
    pub fn value(i: usize) -> Self
    {
        let values = [ViewModes::Objects("Objects"), ViewModes::Scenes("Scenes")];
        return values[i];
    }
}

/*
pub fn view_mode(position: usize) -> &'static str
{
    //let mut view_modes = object_settings::radio_options::init(&["Objects", "Scenes"]);
    let view_modes = ["Objects", "Scenes"];
    return view_modes[position];
}
*/
pub fn game_type(i: usize) -> &'static str
{
    //let mut view_modes = object_settings::radio_options::init(&["Objects", "Scenes"]);
    let game_types = ["2D", "3D"];
    return game_types[i];
}