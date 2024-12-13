# Performance Test Report

**Test Execution Time**: 2024-12-13 15:41:42  
**Total Test Duration**: 9.14 seconds

## 1. LRU Cache Mode Test Results

| Mode | Total Requests | Avg Response Time | 95th Percentile | Cache Hit Rate |
|------|---------------|-------------------|-----------------|----------------|
| Hot Items | 1000 | 11.39 ms | 22.82 ms | 66.60% |
| Sequential | 1000 | 12.08 ms | 22.97 ms | - |
| Random | 1000 | 11.64 ms | 22.27 ms | 34.40% |

## 2. Cache Pollution Test Results

| Phase | Avg Response Time | Cache Hit Rate |
|-------|-------------------|----------------|
| Phase 1 (Hot Data) | 1.43 ms | 90.00% |
| Phase 3 (After Pollution) | 1.39 ms | 79.17% |

## 3. Tag Search Performance

| Tag | Avg Response Time | 95th Percentile |
|-----|-------------------|-----------------|
| Tag1 | 14.46 ms | 26.67 ms |
| Tag2 | 14.88 ms | 26.98 ms |
| Tag3 | 14.97 ms | 26.07 ms |
| Tag4 | 15.76 ms | 29.43 ms |
| Tag5 | 15.45 ms | 28.09 ms |
| Tag6 | 15.33 ms | 29.99 ms |
| Tag7 | 15.89 ms | 27.12 ms |
| Tag8 | 15.77 ms | 29.40 ms |
| Tag9 | 16.22 ms | 34.93 ms |
| Tag10 | 16.01 ms | 29.39 ms |

## 4. Article List Performance

| Metric | Value |
|--------|-------|
| Avg Response Time | 2.93 ms |
| 95th Percentile | 4.67 ms |
| Min Response Time | 2.20 ms |
| Max Response Time | 4.88 ms |

## 5. In-Cache Performance

### Warm-up Phase
| Metric | Value |
|--------|-------|
| Avg Response Time | 1.59 ms |
| 95th Percentile | 1.88 ms |

### Test Phase
| Metric | Value |
|--------|-------|
| Avg Response Time | 11.73 ms |
| 95th Percentile | 21.98 ms |
| Min Response Time | 1.37 ms |
| Max Response Time | 44.85 ms |
| Cache Hit Rate | 100.00% |