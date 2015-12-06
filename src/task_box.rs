use Task;

pub type TaskBox = Box<Task<Output = ()>>;
