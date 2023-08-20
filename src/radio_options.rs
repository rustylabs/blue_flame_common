#[derive(PartialEq, Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum GameTypeDimensions{D2, D3}
impl GameTypeDimensions
{
    pub fn elements() -> [(Self, &'static str); 2]
    {
        return [(Self::D2, "2D"), (Self::D3, "3D")];
    }
}

// When user press 'g' for example move the object based on mouse coordinates
pub enum ObjectMouseMovement{Grab, Size, Rotation}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ViewModes{Objects, Scenes, Blueprints}
impl ViewModes
{
    pub fn elements() -> [(Self, &'static str); 3]
    {
        return [(Self::Objects, "Objects"), (Self::Scenes, "Scenes"), (Self::Blueprints, "Blueprints")];
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

pub mod object_type
{
    #[derive(PartialEq, Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
    pub enum ObjectType
    {
        Light(light::Light),
        Shape(shape::Dimension),
        Empty,
    }
    impl ObjectType
    {
        pub fn elements(object_type: Option<Self>) -> [(Self, &'static str); 3]
        {
            let mut list = [(Self::Light(light::Light::Direction), "Light"), (Self::Shape(shape::Dimension::D2(shape::Shape2D::Square)), "Shape"), (Self::Empty, "Empty")];
            match object_type
            {
                Some(object_type) =>
                    match object_type
                    {
                        Self::Light(value)     => list[0].0 = Self::Light(value),
                        Self::Shape(value) => list[1].0 = Self::Shape(value),
                        Self::Empty                   => {},
                    }
                None => return list,
            }

            /*
            match object_type.unwrap()
            {
                Self::Light(value)     => list[0].0 = Self::Light(value),
                Self::Shape(value) => list[1].0 = Self::Shape(value),
                Self::Empty                   => {},
            }
            */

            return list;
        }
        /*
        pub fn elements() -> [(Self, &'static str); 3]
        {
            return [(Self::Light(light::Light::Direction), "Light"), (Self::Shape(shape::Dimension::D2(shape::Shape2D::Square)), "Shape"), (Self::Empty, "Empty")];
        }
        */
        pub fn current_selected_label(&self) -> &'static str
        {
            match self
            {
                Self::Empty         => "Empty",
                Self::Light(_)      => "Light",
                Self::Shape(_)      => "Shape",
            }
        }
    }
    pub mod light
    {
        #[derive(PartialEq, Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
        pub enum Light
        {
            Direction,
        }
        impl Light
        {
            pub fn elements() -> [(Self, &'static str); 1]
            {
                return [(Self::Direction, "Direction")];
            }
        }
    }
    pub mod shape
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
            Square,
            Triangle,
            Line,
        }
        impl Shape2D
        {
            pub fn elements() -> [(Self, &'static str); 3]
            {
                return [(Self::Square, "Square"), (Self::Triangle, "Triangle"), (Self::Line, "Line")];
            }
        }
        #[derive(PartialEq, Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
        pub enum Shape3D
        {
            Cube,
        }
        impl Shape3D
        {
            pub fn elements() -> [(Self, &'static str); 1]
            {
                return [(Self::Cube, "Cube")];
            }
        }
    }

}