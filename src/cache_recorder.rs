use crate::config;

// Counter for cache hits and misses
pub struct CacheHit {
    pub cache_hit: u32,
    pub cache_miss: u32,
}

impl CacheHit {
    // Constrcut a new CacheHit instance
    pub fn new() -> Self {
        Self {
            cache_hit: 0,
            cache_miss: 0,
        }
    }

    // Increment the cache hit counter
    pub fn hit(&mut self) {
        if config::CONFIG.mainconfig.record_cache_stats {
            self.cache_hit += 1;
        }
    }

    // Decrement the cache miss counter
    pub fn miss(&mut self) {
        if config::CONFIG.mainconfig.record_cache_stats {
            self.cache_miss += 1;
        }
    }

    // Calculate the cache hit rate
    pub fn hit_rate(&self) -> f32 {
        if self.cache_miss == 0 && self.cache_hit == 0 {
            0.0
        } else {
            (self.cache_hit as f32) / (self.cache_hit as f32 + self.cache_miss as f32)
        }
    }

    // Reset counter
    pub fn reset(&mut self) {
        self.cache_hit = 0;
        self.cache_miss = 0;
    }
}
