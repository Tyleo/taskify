pub trait EndScheduleTrait {
    type TEndScheduleReturn;

    fn end_schedule(self) -> Self::TEndScheduleReturn;
}
