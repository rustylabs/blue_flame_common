// position means position of array/Vector
// What shape, i.e. circle, triangle etc
pub fn object_type(position: usize) -> &'static str
{
    let shapes: &[&'static str] = &["Square", "Triangle", "Line"];
    return shapes[position];
}
// x, y, z
pub mod three_d_lables
{
    use blue_engine::RotateAxis;

    pub fn label(position: usize) -> u8
    {
        let axis = [b'x', b'y', b'z'];
        return axis[position];
    }
    pub fn enumm(position: usize) -> RotateAxis
    {
        let axis = [RotateAxis::X, RotateAxis::Y, RotateAxis::Z];
        return axis[position];
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
pub fn view_mode(position: usize) -> &'static str
{
    //let mut view_modes = object_settings::radio_options::init(&["Objects", "Scenes"]);
    let view_modes = ["Objects", "Scenes"];
    return view_modes[position];
}
pub fn game_type(position: usize) -> &'static str
{
    //let mut view_modes = object_settings::radio_options::init(&["Objects", "Scenes"]);
    let game_types = ["2D", "3D"];
    return game_types[position];
}