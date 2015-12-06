pub trait SchedulerTrait {
    type TScheduleReturn;

    fn schedule(&self) -> Self::TScheduleReturn;
}
