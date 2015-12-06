use TaskBox;

pub trait TaskBoxIntoIterator: IntoIterator<Item = TaskBox> { }
impl <T> TaskBoxIntoIterator for T
    where T: IntoIterator<Item = TaskBox> { }
