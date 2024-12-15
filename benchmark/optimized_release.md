# Performance Test Report
December 15, 2024

## Test Environment
- CPU: Ryzen AI 9 365
- Architecture: march=native
- Build Configuration:
  ```toml
  [profile.release]
  opt-level = 3
  lto = "fat"
  codegen-units = 1
  panic = "abort"
  strip = true

  [profile.release.build-override]
  opt-level = 3
  codegen-units = 1
  ```

## Test Duration
Total duration: 5.61 seconds

## LRU Cache Pattern Analysis

| Pattern    | Requests | Success Rate | Avg Response (ms) | P95 Response (ms) | Cache Hit Rate |
|------------|----------|--------------|-------------------|-------------------|----------------|
| Hot_items  | 1000     | 100%         | 19.81            | 44.49            | 66.10%         |
| Sequential | 1000     | 100%         | 18.88            | 41.80            | 0.00%          |
| Random     | 1000     | 100%         | 5.50             | 12.43            | 33.30%         |

## Tag Search Performance

| Tag        | Avg Response (ms) | P95 Response (ms) | Samples |
|------------|------------------|-------------------|---------|
| 人工智能    | 9.94             | 21.49            | 50      |
| 后端       | 9.88             | 22.30            | 50      |
| 教程       | 10.43            | 18.50            | 50      |
| 云计算     | 10.16            | 23.98            | 50      |
| 前端       | 10.73            | 22.91            | 50      |
| 软件       | 10.38            | 22.17            | 50      |
| 编程       | 11.09            | 25.50            | 50      |
| 技术       | 10.94            | 19.38            | 50      |
| 数据库     | 10.57            | 24.09            | 50      |
| 开发       | 11.49            | 47.85            | 50      |

## Cache Pollution Analysis

| Phase  | Requests | Avg Response (ms) | P95 Response (ms) |
|--------|----------|-------------------|-------------------|
| Phase1 | 500      | 0.95             | 1.26             |
| Phase2 | 200      | 1.09             | 1.49             |
| Phase3 | 500      | 0.92             | 1.22             |

## Cache Performance Metrics
- Initial Cache Hit Rate: 90.00%
- Final Cache Hit Rate: 79.17%

## Note
These optimization options do not help with anything other than Random access and may even have adverse effects. They can also cause instability, so it is recommended not to enable these optimization options.