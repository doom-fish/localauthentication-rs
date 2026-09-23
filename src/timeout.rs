use std::time::Duration;

use crate::ffi;

pub fn set_sync_timeout(timeout: Option<Duration>) {
    let seconds = timeout.map_or(f64::INFINITY, |timeout| timeout.as_secs_f64());
    unsafe { ffi::la_set_sync_timeout(seconds) };
}

#[must_use]
pub fn sync_timeout() -> Option<Duration> {
    let seconds = unsafe { ffi::la_get_sync_timeout() };
    seconds
        .is_finite()
        .then(|| Duration::try_from_secs_f64(seconds).unwrap_or(Duration::MAX))
}

#[cfg(test)]
mod tests {
    use core::ffi::c_char;
    use std::ptr;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Mutex;
    use std::thread;
    use std::time::{Duration, Instant};

    use super::{set_sync_timeout, sync_timeout};
    use crate::la_error::{from_status, LAError};

    static SERIAL: Mutex<()> = Mutex::new(());
    static LATE_SUCCESSES: AtomicUsize = AtomicUsize::new(0);

    unsafe extern "C" {
        fn la_bridge_probe_await(
            delay_milliseconds: u32,
            late_success: Option<unsafe extern "C" fn()>,
            out_timeout_hook_ran: *mut u8,
            error_out: *mut *mut c_char,
        ) -> i32;
    }

    unsafe extern "C" fn record_late_success() {
        LATE_SUCCESSES.fetch_add(1, Ordering::SeqCst);
    }

    fn probe(delay_milliseconds: u32) -> (Result<(), LAError>, bool) {
        let mut hook_ran = 0_u8;
        let mut error = ptr::null_mut();
        let status = unsafe {
            la_bridge_probe_await(
                delay_milliseconds,
                Some(record_late_success),
                &raw mut hook_ran,
                &raw mut error,
            )
        };
        let result = if status == crate::ffi::status::OK {
            Ok(())
        } else {
            Err(from_status(status, error))
        };
        (result, hook_ran != 0)
    }

    #[test]
    fn the_timeout_is_configurable_and_defaults_to_thirty_seconds() {
        let _serial = SERIAL
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        assert_eq!(sync_timeout(), Some(Duration::from_secs(30)));
        set_sync_timeout(Some(Duration::from_millis(1500)));
        assert_eq!(sync_timeout(), Some(Duration::from_millis(1500)));
        set_sync_timeout(None);
        assert_eq!(sync_timeout(), None);
        set_sync_timeout(Some(Duration::from_secs(30)));
    }

    #[test]
    fn timed_out_calls_are_cancelled_and_late_results_still_run_their_hook() {
        let _serial = SERIAL
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let before = LATE_SUCCESSES.load(Ordering::SeqCst);
        set_sync_timeout(Some(Duration::from_millis(50)));
        let started = Instant::now();
        let (result, hook_ran) = probe(400);
        set_sync_timeout(Some(Duration::from_secs(30)));

        assert!(matches!(result, Err(LAError::TimedOut(_))), "{result:?}");
        assert!(hook_ran);
        assert!(started.elapsed() < Duration::from_millis(350));

        let deadline = Instant::now() + Duration::from_secs(5);
        while LATE_SUCCESSES.load(Ordering::SeqCst) == before && Instant::now() < deadline {
            thread::sleep(Duration::from_millis(20));
        }
        assert_eq!(LATE_SUCCESSES.load(Ordering::SeqCst), before + 1);
    }

    #[test]
    fn calls_that_finish_in_time_skip_both_hooks() {
        let _serial = SERIAL
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let before = LATE_SUCCESSES.load(Ordering::SeqCst);
        let (result, hook_ran) = probe(10);
        assert!(result.is_ok(), "{result:?}");
        assert!(!hook_ran);
        thread::sleep(Duration::from_millis(100));
        assert_eq!(LATE_SUCCESSES.load(Ordering::SeqCst), before);
    }
}
