use SchedulerTrait;
use TaskBox;

pub trait TaskBoxIntoIterator<TScheduler>: IntoIterator<Item = TaskBox<TScheduler>> +
                                           Send
    where TScheduler: SchedulerTrait { }

impl <TScheduler,
      T> TaskBoxIntoIterator<TScheduler> for T
    where TScheduler: SchedulerTrait,
          T: IntoIterator<Item = TaskBox<TScheduler>> +
             Send { }
