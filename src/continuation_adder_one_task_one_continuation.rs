use ContinuationAdderOneTaskMultipleContinuations;
use ContinuationAdderTrait;
use LooseContinuation;
use LooseContinuationIntoIterator;
use ScheduleOneTaskOneContinuation;
use Scheduler;
use ScheduleTrait;
use Task;
use TaskBox;
use TaskBoxIntoIterator;

pub struct ContinuationAdderOneTaskOneContinuation<'a> {
    scheduler: &'a Scheduler,
    task_box: TaskBox,
    continuation_box: TaskBox,
}

impl <'a> ContinuationAdderOneTaskOneContinuation<'a> {
    pub fn new(scheduler: &'a Scheduler,
               task_box: TaskBox,
               continuation_box: TaskBox) -> ContinuationAdderOneTaskOneContinuation<'a>  {
        ContinuationAdderOneTaskOneContinuation { scheduler: scheduler,
                                                  task_box: task_box,
                                                  continuation_box: continuation_box }
    }

    fn convert_to_continuation_adder_one_task_multiple_continuations(self) -> ContinuationAdderOneTaskMultipleContinuations<'a> {
        ContinuationAdderOneTaskMultipleContinuations::new(self.scheduler,
                                                           self.task_box,
                                                           Vec::new())
    }

    fn convert_to_schedule(self) -> ScheduleOneTaskOneContinuation<'a> {
        ScheduleOneTaskOneContinuation::new(self.scheduler,
                                            self.task_box,
                                            self.continuation_box)
    }
}


impl <'a> ContinuationAdderTrait<ContinuationAdderOneTaskMultipleContinuations<'a>,
                                 ContinuationAdderOneTaskMultipleContinuations<'a>> for ContinuationAdderOneTaskOneContinuation<'a> {
    fn add_continuation<TTask>(self,
                               continuation: TTask) -> ContinuationAdderOneTaskMultipleContinuations<'a>
        where TTask: 'static +
                     Task {
        let continuation_box = Box::new(continuation);
        self.add_continuation_box(continuation_box)
    }

    fn add_continuation_box(self,
                            continuation_box: TaskBox) -> ContinuationAdderOneTaskMultipleContinuations<'a> {
        self.convert_to_continuation_adder_one_task_multiple_continuations()
            .add_continuation_box(continuation_box)
    }

    fn add_continuation_boxes<TTaskBoxIntoIterator>(self,
                                                    continuation_boxes: TTaskBoxIntoIterator) -> ContinuationAdderOneTaskMultipleContinuations<'a>
        where TTaskBoxIntoIterator: 'static +
                                    TaskBoxIntoIterator {
        self.convert_to_continuation_adder_one_task_multiple_continuations()
            .add_continuation_boxes(continuation_boxes)
    }

    // fn add_loose_continuation(self, loose_continuation: LooseContinuation) -> ContinuationAdderOneTaskMultipleContinuations<'a> {
    //     self.convert_to_continuation_adder_one_task_multiple_continuations()
    //         .add_loose_continuation(loose_continuation)
    // }

    // fn add_loose_continuations<TLooseContinuationIntoIterator: 'static + LooseContinuationIntoIterator>(self, loose_continuations: TLooseContinuationIntoIterator) -> ContinuationAdderOneTaskMultipleContinuations<'a> {
    //     self.convert_to_continuation_adder_one_task_multiple_continuations()
    //         .add_loose_continuations(loose_continuations)
    // }
}

impl <'a> ScheduleTrait for ContinuationAdderOneTaskOneContinuation<'a> {
    fn schedule(self) {
        self.convert_to_schedule()
            .schedule()
    }
}
