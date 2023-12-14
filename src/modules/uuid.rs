use pgrx::prelude::*;
use uuid::Uuid;

#[pg_extern]
pub fn str_uuid() -> String {
    Uuid::new_v4().to_string()
}

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use pgrx::prelude::*;
}
