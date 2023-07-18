#[derive(PartialEq, Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum GameTypeDimensions{D2, D3}
impl GameTypeDimensions
{
    pub fn label(value: &Self) -> &'static str
    {
        match value
        {
            Self::D2        => "2D",
            Self::D3        => "3D",
        }
    }
    pub fn elements() -> [Self; 2]
    {
        return [Self::D2, Self::D3];
    }
}



#[derive(PartialEq, Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum Texture{Clamp, Repeat, MirrorRepeat}
impl Texture
{
    pub fn label(value: &Self) -> &'static str
    {
        match value
        {
            Self::Clamp         => "Clamp",
            Self::Repeat        => "Repeat",
            Self::MirrorRepeat  => "MirrorRepeat",
        }
    }
    pub fn elements() -> [Self; 3]
    {
        return [Self::Clamp, Self::Repeat, Self::MirrorRepeat];
    }
}