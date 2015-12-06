use deque::{ BufferPool, Stealer, Stolen, Worker };
use SchedulerTrait;
use SimpleBeginScheduleTrait;
use TaskBox;
use TaskBoxIntoIterator;

pub struct Scheduler {
    worker: Worker<TaskBox>,
    stealers: Vec<Stealer<TaskBox>>,
}

impl Scheduler {
    pub fn new(worker: Worker<TaskBox>,
               stealers: Vec<Stealer<TaskBox>>) -> Scheduler {
        Scheduler { worker: worker,
                    stealers: stealers }
    }
}

impl<'a> SimpleBeginScheduleTrait<'a,
                                  Scheduler> for Scheduler { }

impl SchedulerTrait for Scheduler {
    type TScheduleReturn = ();
    type TScheduleMultipleReturn = ();

    fn schedule(&self,
                task_box: TaskBox) -> Self::TScheduleReturn {
        self.worker.push(task_box)
    }

    fn schedule_multiple<TTaskBoxIntoIterator>(&self,
                                               task_boxes: TTaskBoxIntoIterator) -> Self::TScheduleMultipleReturn
        where TTaskBoxIntoIterator: TaskBoxIntoIterator {
        for task_box in task_boxes {
            self.worker.push(task_box)
        }
    }
}
