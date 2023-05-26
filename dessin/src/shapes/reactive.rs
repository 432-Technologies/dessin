use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct Reactive {
    shape: Option<Arc<Box<()>>>,
}

impl Default for Reactive {
    fn default() -> Self {
        todo!()
    }
}
