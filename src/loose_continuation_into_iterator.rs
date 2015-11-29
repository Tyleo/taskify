use LooseContinuation;

pub trait LooseContinuationIntoIterator: IntoIterator<Item = LooseContinuation> + Send { }
impl <T> LooseContinuationIntoIterator for T
    where T : IntoIterator<Item = LooseContinuation> + Send { }