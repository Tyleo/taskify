use std::boxed::FnBox;
use TaskScheduler;

pub type TaskBox = Box<FnBox(&TaskScheduler) + Sync + Send>;
