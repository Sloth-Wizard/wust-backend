///
/// Macro file
/// Maybe split into multiple if it becomes too big
/// 

///
/// Return a boolean after checking if the
/// passed data is eligable for the new status
/// 
#[macro_export]
macro_rules! new_status {
    ($a:expr,$typ:ty) => {
        {
            use chrono::{Duration, Utc};
            let now_minus_5_days: $typ = (Utc::now() - Duration::days(5)).naive_utc();
            $a >= now_minus_5_days
        }
    }
}
