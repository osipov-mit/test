use gmeta::{InOut, Metadata};
use gstd::prelude::*;

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = InOut<(), ()>;
    type Handle = InOut<String, Result<String, u16>>;
    type Others = InOut<(), ()>;
    type Reply = ();
    type Signal = ();
    type State = InOut<(), ()>;
}
