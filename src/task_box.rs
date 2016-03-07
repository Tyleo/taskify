use Task;

pub type TaskBox<TScheduler> = Box<Task<TScheduler, Output = ()>>;
