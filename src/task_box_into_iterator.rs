use TaskBox;

pub trait TaskBoxIntoIterator: IntoIterator<Item = TaskBox> + Send { }
impl <T> TaskBoxIntoIterator for T
    where T : IntoIterator<Item = TaskBox> + Send { }
