use core::marker::PhantomData;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OperationState {
    Idle,
    Busy,
    WaitingForStcToClear(bool),
}

/// Si4703 device driver
#[derive(Debug)]
pub struct Si4703<I2C, IC> {
    pub(crate) i2c: I2C,
    pub(crate) seeking_state: OperationState,
    pub(crate) tuning_state: OperationState,
    pub(crate) _ic: PhantomData<IC>,
}
