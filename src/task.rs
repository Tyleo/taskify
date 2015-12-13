use std::boxed::FnBox;
use SchedulerTrait;

pub trait Task<TScheduler>: FnBox(&TScheduler) +
                            Send
    where TScheduler: SchedulerTrait { }

impl <TScheduler,
      T> Task<TScheduler> for T
    where TScheduler: SchedulerTrait,
          T: FnBox(&TScheduler) +
             Send { }
