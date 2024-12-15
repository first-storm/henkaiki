# Performance Test Report
December 15, 2024

## Test Environment
- CPU: Ryzen AI 9 365

## Test Duration
Total duration: 5.41 seconds

## LRU Cache Pattern Analysis

| Pattern    | Requests | Success Rate | Avg Response (ms) | P95 Response (ms) | Cache Hit Rate |
|------------|----------|--------------|-------------------|-------------------|----------------|
| Hot_items  | 1000     | 100%         | 18.31            | 42.45            | 64.10%         |
| Sequential | 1000     | 100%         | 19.95            | 42.10            | 0.00%          |
| Random     | 1000     | 100%         | 16.27            | 35.88            | 33.50%         |

## Tag Search Performance

| Tag        | Avg Response (ms) | P95 Response (ms) | Samples |
|------------|-------------------|-------------------|---------|
| 技术       | 8.90             | 19.69            | 50      |
| 开发       | 9.07             | 19.11            | 50      |
| 云计算     | 9.18             | 19.64            | 50      |
| 人工智能    | 8.85             | 20.67            | 50      |
| 后端       | 9.14             | 18.11            | 50      |
| 教程       | 10.41            | 21.66            | 50      |
| 编程       | 9.94             | 24.08            | 50      |
| 软件       | 9.92             | 21.55            | 50      |
| 数据库     | 9.31             | 20.63            | 50      |
| 前端       | 9.94             | 26.41            | 50      |

## Cache Pollution Analysis

| Phase  | Requests | Avg Response (ms) | P95 Response (ms) |
|--------|----------|-------------------|-------------------|
| Phase1 | 500      | 0.96             | 1.39             |
| Phase2 | 200      | 1.09             | 1.48             |
| Phase3 | 500      | 0.92             | 1.23             |

## Cache Performance Metrics
- Initial Cache Hit Rate: 90.00%
- Final Cache Hit Rate: 79.17%