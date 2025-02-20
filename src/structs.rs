
#[derive(Clone)]
pub struct QueryContext {
  pub(crate) name: String
}

pub type Callback = fn();