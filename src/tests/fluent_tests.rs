use fluent::*;
use Scheduler;
use rand::StdRng;

#[test]
fn schedule_one() {
    let num_threads = 8;
    let schedulers = Scheduler::<StdRng>::new_batch(num_threads);

    for scheduler in &schedulers {
        scheduler
            .begin_schedule()
            .add_task(
                |_: &Scheduler<StdRng>| {

                }
            )
            .end_schedule();
    }
}
