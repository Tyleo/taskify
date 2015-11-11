use std::boxed::FnBox;
use TaskScheduler;

pub trait Task: FnBox(&TaskScheduler) + Sync + Send { }
impl <T> Task for T
    where T: FnBox(&TaskScheduler) + Sync + Send { }
