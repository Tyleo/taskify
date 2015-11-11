use TaskBox;

pub trait TaskBoxIntoIterator: IntoIterator<Item = TaskBox> + Sync + Send { }
impl <T> TaskBoxIntoIterator for T
    where T : IntoIterator<Item = TaskBox> + Sync + Send { }
