use std::sync::atomic::{AtomicU64, Ordering};


pub struct NodeMetrics {
    tasks_processed: AtomicU64,
    tasks_failed: AtomicU64,
    average_processing_time: AtomicU64
}

impl NodeMetrics {
    pub fn new() -> Self {
        Self {
            tasks_processed: AtomicU64::new(0),
            tasks_failed: AtomicU64::new(0),
            average_processing_time: AtomicU64::new(0),
        }
    }

    pub fn increment_processed(&self) {
        self.tasks_processed.fetch_add(1, Ordering::SeqCst);
    }

    pub fn increment_failed(&self) {
        self.tasks_processed.fetch_add(1, Ordering::SeqCst);
    }

    pub fn update_processing_time(&self, time_ms: u64) {
        let current_avg = self.average_processing_time.load(Ordering::SeqCst);
        let processed = self.tasks_processed.load(Ordering::SeqCst);
        
        let new_avg = if processed == 0 {
            time_ms
        } else {
            (current_avg * processed + time_ms) / (processed + 1)
        };
        
        self.average_processing_time.store(new_avg, Ordering::SeqCst);
    }

}