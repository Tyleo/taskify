use deque::{ BufferPool, Stealer, Stolen, Worker };
use SchedulerTrait;
use SimpleBeginScheduleTrait;
use TaskBox;
use TaskBoxIntoIterator;

pub struct Scheduler {
    worker: Worker<TaskBox<Scheduler>>,
    stealers: Vec<Stealer<TaskBox<Scheduler>>>,
}

impl Scheduler {
    pub fn new(worker: Worker<TaskBox<Scheduler>>,
               stealers: Vec<Stealer<TaskBox<Scheduler>>>) -> Scheduler {
        Scheduler { worker: worker,
                    stealers: stealers }
    }
}

impl<'a> SimpleBeginScheduleTrait<'a,
                                  Scheduler> for Scheduler { }

impl SchedulerTrait for Scheduler {
    type TTaskBoxParam = Scheduler;
    type TScheduleReturn = ();
    type TScheduleMultipleReturn = ();

    fn schedule(&self,
                task_box: TaskBox<Scheduler>) -> Self::TScheduleReturn {
        self.worker.push(task_box)
    }

    fn schedule_multiple<TTaskBoxIntoIterator>(&self,
                                               task_boxes: TTaskBoxIntoIterator) -> Self::TScheduleMultipleReturn
        where TTaskBoxIntoIterator: TaskBoxIntoIterator<Scheduler> {
        for task_box in task_boxes {
            self.worker.push(task_box)
        }
    }
}
