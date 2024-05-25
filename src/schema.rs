use core::any::{Any, TypeId};
use core::fmt::Debug;

pub trait Schema: Debug + Any {
    fn schema(&self) -> TypeId {
        self.type_id()
    }
}
