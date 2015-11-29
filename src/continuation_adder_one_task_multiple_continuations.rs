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
}

impl <'a> ContinuationAdderOneTaskMultipleContinuations<'a> {
    pub fn new(scheduler: &'a Scheduler) -> ContinuationAdderOneTaskMultipleContinuations<'a>  {
        ContinuationAdderOneTaskMultipleContinuations { scheduler: scheduler }
    }

    fn convert_to_schedule(self) -> ScheduleOneTaskMultipleContinuations {
        ScheduleOneTaskMultipleContinuations
    }
}

impl <'a> ContinuationAdderTrait<ContinuationAdderOneTaskMultipleContinuations<'a>,
                                 ContinuationAdderOneTaskMultipleContinuations<'a>> for ContinuationAdderOneTaskMultipleContinuations<'a> {
    fn add_continuation<TTask: 'static + Task>(self, continuation: TTask) -> ContinuationAdderOneTaskMultipleContinuations<'a> {
        self
    }

    fn add_continuation_box(self, continuation_box: TaskBox) -> ContinuationAdderOneTaskMultipleContinuations<'a> {
        self
    }

    fn add_continuation_boxes<TTaskBoxIntoIterator: 'static + TaskBoxIntoIterator>(self, continuation_boxes: TTaskBoxIntoIterator) -> ContinuationAdderOneTaskMultipleContinuations<'a> {
        self
    }

    fn add_loose_continuation(self, loose_continuation: LooseContinuation) -> ContinuationAdderOneTaskMultipleContinuations<'a> {
        self
    }

    fn add_loose_continuations<TLooseContinuationIntoIterator: 'static + LooseContinuationIntoIterator>(self, loose_continuations: TLooseContinuationIntoIterator) -> ContinuationAdderOneTaskMultipleContinuations<'a> {
        self
    }
}

impl <'a> ScheduleTrait for ContinuationAdderOneTaskMultipleContinuations<'a> {
    fn schedule(self) {
        self.convert_to_schedule()
            .schedule()
    }
}
