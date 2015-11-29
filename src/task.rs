use std::boxed::FnBox;
use Scheduler;

pub trait Task: FnBox(&Scheduler) + Send { }
impl <T> Task for T
    where T: FnBox(&Scheduler) + Send { }
