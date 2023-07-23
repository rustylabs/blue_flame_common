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

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ViewModes{Objects, Scenes}
impl ViewModes
{
    pub fn label(value: &Self) -> &'static str
    {
        match value
        {
            Self::Objects         => "Objects",
            Self::Scenes          => "Scenes",
        }
    }
    pub fn elements() -> [Self; 2]
    {
        return [Self::Objects, Self::Scenes];
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

#[derive(PartialEq, Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum ObjectType{Square, Triangle, Line}
impl ObjectType
{
    pub fn label(value: &Self) -> &'static str
    {
        match value
        {
            Self::Square      => "Square",
            Self::Triangle    => "Triangle",
            Self::Line        => "Line",
        }
    }
    pub fn elements() -> [Self; 3]
    {
        return [Self::Square, Self::Triangle, Self::Line];
    }
}

mod object_type
{
    #[derive(PartialEq, Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
    pub enum ObjectType
    {
        Light,
        Shape(shape::Dimension),
        Empty,
    }
    mod light
    {
        #[derive(PartialEq, Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
        pub enum Light
        {
            Direction,
        }
    }
    mod shape
    {
        #[derive(PartialEq, Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
        pub enum Dimension
        {
            D2(Shape2D),
            D3(Shape3D),
        }
        #[derive(PartialEq, Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
        pub enum Shape2D
        {
            Triangle,
            Square,
            Line,
        }
        #[derive(PartialEq, Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
        pub enum Shape3D
        {
            Cube,
        }
    }

}