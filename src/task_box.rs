use std::boxed::FnBox;
use Scheduler;

pub type TaskBox = Box<FnBox(&Scheduler) + Send>;
