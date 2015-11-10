use Task;

pub trait TaskIntoIterator: IntoIterator<Item = Task> + Sync + Send { }
impl <T> TaskIntoIterator for T
    where T : IntoIterator<Item = Task> + Sync + Send { }
