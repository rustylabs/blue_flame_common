pub const EMOJIS: Emojis = Emojis::init();

pub struct Emojis
{
    pub trash: char,
    pub addition: Addition,
    pub settings: char,
    pub cancel: char,
    pub save: char,
    pub eye: char,
    pub tick: char,
    pub load: char,
    pub undo_redo: UndoRedo,
    pub file_icons: FileIcons,
    pub arrows: Arrows,
}
impl Emojis
{
    pub const fn init() -> Self
    {
        Self
        {
            trash: '🗑',
            addition: Addition::init(),
            settings: '⚙',
            cancel: '⛔',
            save: '💾',
            eye: '👁',
            tick: '✅',
            load: '↻',
            undo_redo: UndoRedo::init(),
            file_icons: FileIcons::init(),
            arrows: Arrows::init(),
        }
    }
}
// + - 
pub struct Addition
{
    pub plus: char,
    pub minus: char,
}
impl Addition
{
    pub const fn init() -> Self
    {
        Self
        {
            plus: '➕',
            minus: '➖',
        }
    }
}
pub struct FileIcons
{
    pub file: char,
    pub folder: char,
}
impl FileIcons
{
    pub const fn init() -> Self
    {
        Self
        {
            file: '📄',
            folder: '🗀',
        }
    }
}
pub struct UndoRedo
{
    pub undo: char,
    pub redo: char,
}
impl UndoRedo
{
    pub const fn init() -> Self
    {
        Self
        {
            /*
            undo: '↶',
            redo: '↷',
            */
            undo: '↺',
            redo: '↻',
        }
    }
}
pub struct Arrows
{
    pub right: char,
    pub left: char,
    pub down: char,
}
impl Arrows
{
    pub const fn init() -> Self
    {
        Self
        {
            right: '▶',
            left: '◀',
            down: '⬇',
        }
    }
}