use std::fmt;
use std::sync::{Arc, Condvar, Mutex};

pub struct WaitGroup {
    inner: Arc<Inner>,
}

struct Inner {
    cvar: Condvar,
    count: Mutex<usize>,
}

impl Drop for WaitGroup {
    fn drop(&mut self) {
        let mut count = self.inner.count.lock().unwrap();
        *count -= 1;

        if *count == 0 {
            self.inner.cvar.notify_all();
        }
    }
}

impl Clone for WaitGroup {
    fn clone(&self) -> WaitGroup {
        let mut count = self.inner.count.lock().unwrap();
        *count += 1;

        WaitGroup {
            inner: self.inner.clone(),
        }
    }
}

impl Default for WaitGroup {
    fn default() -> Self {
        Self {
            inner: Arc::new(Inner {
                cvar: Condvar::new(),
                count: Mutex::new(1),
            }),
        }
    }
}

impl WaitGroup {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn wait(self) {
        if *self.inner.count.lock().unwrap() == 1 {
            return;
        }

        let inner = self.inner.clone();
        drop(self);

        let count = inner.count.lock().unwrap();
        let _ = inner.cvar.wait_while(count, |cnt| *cnt > 0).unwrap();
    }
}

impl fmt::Debug for WaitGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let count: &usize = &*self.inner.count.lock().unwrap();
        f.debug_struct("WaitGroup").field("count", count).finish()
    }
}
