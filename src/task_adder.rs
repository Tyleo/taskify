use ContinuationAdder;
use ContinuationAdderTrait;
use ScheduleTrait;
use LooseContinuation;
use LooseContinuationIntoIterator;
use Task;
use TaskAdderTrait;
use TaskBox;
use TaskBoxIntoIterator;

pub struct TaskAdder;

impl TaskAdder {
    fn convert_to_continuation_adder(self) -> ContinuationAdder {
        ContinuationAdder
    }
}

impl TaskAdderTrait<ContinuationAdder, ContinuationAdder, TaskAdder, TaskAdder> for TaskAdder {
    fn add_task<TTask: 'static + Task>(self, task: TTask) -> TaskAdder {
        self
    }

    fn add_task_box(self, task_box: TaskBox) -> TaskAdder {
        self
    }

    fn add_task_boxes<TTaskBoxIntoIterator: 'static + TaskBoxIntoIterator>(self, task_boxes: TTaskBoxIntoIterator) -> TaskAdder {
        self
    }
}

impl ContinuationAdderTrait<ContinuationAdder, ContinuationAdder> for TaskAdder {
    fn add_continuation<TTask: 'static + Task>(self, continuation: TTask) -> ContinuationAdder {
        self.convert_to_continuation_adder()
            .add_continuation(continuation)
    }

    fn add_continuation_box(self, continuation_box: TaskBox) -> ContinuationAdder {
        self.convert_to_continuation_adder()
            .add_continuation_box(continuation_box)
    }

    fn add_continuation_boxes<TTaskBoxIntoIterator: 'static + TaskBoxIntoIterator>(self, continuation_boxes: TTaskBoxIntoIterator) -> ContinuationAdder {
        self.convert_to_continuation_adder()
            .add_continuation_boxes(continuation_boxes)
    }

    fn add_loose_continuation(self, loose_continuation: LooseContinuation) -> ContinuationAdder {
        self.convert_to_continuation_adder()
            .add_loose_continuation(loose_continuation)
    }

    fn add_loose_continuations<TLooseContinuationIntoIterator: 'static + LooseContinuationIntoIterator>(self, loose_continuations: TLooseContinuationIntoIterator) -> ContinuationAdder {
        self.convert_to_continuation_adder()
            .add_loose_continuations(loose_continuations)
    }
}

impl ScheduleTrait for TaskAdder {
    fn schedule(self) {
        self.convert_to_continuation_adder()
            .schedule();
    }
}
