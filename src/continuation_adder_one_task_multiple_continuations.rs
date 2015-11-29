use ContinuationAdderTrait;
use LooseContinuation;
use LooseContinuationIntoIterator;
use ScheduleOneTaskMultipleContinuations;
use Scheduler;
use ScheduleTrait;
use Task;
use TaskBox;
use TaskBoxIntoIterator;

pub struct ContinuationAdderOneTaskMultipleContinuations<'a> {
    scheduler: &'a Scheduler,
    task_box: TaskBox,
    continuation_boxes: Vec<TaskBox>,
}

impl <'a> ContinuationAdderOneTaskMultipleContinuations<'a> {
    pub fn new(scheduler: &'a Scheduler,
               task_box: TaskBox,
               continuation_boxes: Vec<TaskBox>) -> ContinuationAdderOneTaskMultipleContinuations<'a>  {
        ContinuationAdderOneTaskMultipleContinuations { scheduler: scheduler,
                                                        task_box: task_box,
                                                        continuation_boxes: continuation_boxes }
    }

    fn convert_to_schedule(self) -> ScheduleOneTaskMultipleContinuations<'a> {
        ScheduleOneTaskMultipleContinuations::new(self.scheduler,
                                                  self.task_box,
                                                  self.continuation_boxes)
    }
}

impl <'a> ContinuationAdderTrait<ContinuationAdderOneTaskMultipleContinuations<'a>,
                                 ContinuationAdderOneTaskMultipleContinuations<'a>> for ContinuationAdderOneTaskMultipleContinuations<'a> {
    fn add_continuation<TTask>(self,
                               continuation: TTask) -> ContinuationAdderOneTaskMultipleContinuations<'a>
        where TTask: 'static +
                     Task {
        let continuation_box = Box::new(continuation);
        self.add_continuation_box(continuation_box)
    }

    fn add_continuation_box(self,
                            continuation_box: TaskBox) -> ContinuationAdderOneTaskMultipleContinuations<'a> {
        let mut mut_self = self;
        mut_self.continuation_boxes.push(continuation_box);
        mut_self
    }

    fn add_continuation_boxes<TTaskBoxIntoIterator>(self,
                                                    continuation_boxes: TTaskBoxIntoIterator) -> ContinuationAdderOneTaskMultipleContinuations<'a>
        where TTaskBoxIntoIterator: 'static +
                                    TaskBoxIntoIterator {
        let mut mut_self = self;
        for continuation_box in continuation_boxes {
            mut_self.continuation_boxes.push(continuation_box);
        }
        mut_self
    }

    // fn add_loose_continuation(self, loose_continuation: LooseContinuation) -> ContinuationAdderOneTaskMultipleContinuations<'a> {
    //     self
    // }

    // fn add_loose_continuations<TLooseContinuationIntoIterator: 'static + LooseContinuationIntoIterator>(self, loose_continuations: TLooseContinuationIntoIterator) -> ContinuationAdderOneTaskMultipleContinuations<'a> {
    //     self
    // }
}

impl <'a> ScheduleTrait for ContinuationAdderOneTaskMultipleContinuations<'a> {
    fn schedule(self) {
        self.convert_to_schedule()
            .schedule()
    }
}
