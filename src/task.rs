use std::boxed::FnBox;
use TaskScheduler;

pub type Task = Box<FnBox(&TaskScheduler) + Sync + Send>;
