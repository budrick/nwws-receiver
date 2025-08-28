#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum PVtecClass {
    Operational,
    Test,
    Experimental,
    ExperimentalInOperational,
}
