import requests
import time
import random
import concurrent.futures
import statistics
from collections import defaultdict
import json

# Base configuration
BASE_URL = 'http://127.0.0.1:8080'
ARTICLE_IDS = list(range(1, 1001))
TAGS = ['技术', '编程', '教程', '开发', '软件', '云计算', '人工智能', '数据库', '前端', '后端']
CACHE_SIZE = 500
MAX_WORKERS = 20

class LRUCacheTester:
    def __init__(self):
        self.session = requests.Session()
        self.results = defaultdict(dict)

    def clear_cache(self):
        url = f'{BASE_URL}/api/v1/articles/cache'
        try:
            response = self.session.delete(url)
            return response.status_code == 200
        except requests.RequestException as e:
            print(f"Failed to clear cache: {e}")
            return False

    def reset_cache_stats(self):
        url = f'{BASE_URL}/api/v1/articles/cache/stats/reset'
        try:
            response = self.session.post(url)
            return response.status_code == 200
        except requests.RequestException as e:
            print(f"Failed to reset cache stats: {e}")
            return False

    def get_cache_stats(self):
        url = f'{BASE_URL}/api/v1/articles/cache/stats'
        try:
            response = self.session.get(url)
            if response.status_code == 200:
                return response.json().get('data', {})
            return None
        except requests.RequestException as e:
            print(f"Failed to get cache stats: {e}")
            return None

    def make_request(self, url):
        start_time = time.time()
        try:
            response = requests.get(url)
            end_time = time.time()
            return {
                'response_time': end_time - start_time,
                'status_code': response.status_code,
                'url': url
            }
        except requests.RequestException as e:
            end_time = time.time()
            print(f"Request failed: {e}")
            return {
                'response_time': end_time - start_time,
                'status_code': None,
                'url': url
            }

    def test_lru_pattern(self):
        print("\n=== Testing LRU Cache Patterns ===")
        self.clear_cache()
        test_patterns = {
            'hot_items': self._generate_hot_items_pattern(),
            'sequential': self._generate_sequential_pattern(),
            'random': self._generate_random_pattern()
        }

        for pattern_name, urls in test_patterns.items():
            print(f"\nTesting pattern: {pattern_name}")
            self.clear_cache()
            self.reset_cache_stats()

            with concurrent.futures.ThreadPoolExecutor(max_workers=MAX_WORKERS) as executor:
                results = list(executor.map(self.make_request, urls))

            response_times = [r['response_time'] for r in results if r['status_code'] == 200]
            if response_times:
                avg_response_time = statistics.mean(response_times)
                p95_response_time = statistics.quantiles(response_times, n=20)[18]
            else:
                avg_response_time = p95_response_time = 0

            cache_stats = self.get_cache_stats()

            self.results[f'lru_{pattern_name}'] = {
                'total_requests': len(results),
                'successful_requests': len(response_times),
                'avg_response_time': avg_response_time,
                'p95_response_time': p95_response_time,
                'cache_stats': cache_stats
            }

    def _generate_hot_items_pattern(self, total_requests=1000):
        hot_articles = random.sample(ARTICLE_IDS, int(len(ARTICLE_IDS) * 0.2))
        urls = []
        for _ in range(total_requests):
            if random.random() < 0.8:
                article_id = random.choice(hot_articles)
            else:
                article_id = random.choice(ARTICLE_IDS)
            urls.append(f'{BASE_URL}/api/v1/articles/{article_id}')
        return urls

    def _generate_sequential_pattern(self, total_requests=1000):
        urls = []
        current_position = 0
        while len(urls) < total_requests:
            article_id = ARTICLE_IDS[current_position % len(ARTICLE_IDS)]
            urls.append(f'{BASE_URL}/api/v1/articles/{article_id}')
            current_position += 1
        return urls

    def _generate_random_pattern(self, total_requests=1000):
        return [f'{BASE_URL}/api/v1/articles/{random.choice(ARTICLE_IDS)}' for _ in range(total_requests)]

    def test_tag_search(self):
        print("\n=== Testing Tag Search Performance ===")
        self.clear_cache()
        self.reset_cache_stats()

        def test_tag(tag):
            results = []
            for _ in range(50):
                url = f'{BASE_URL}/api/v1/articles/tags/{tag}?limit=10&page=0'
                result = self.make_request(url)
                results.append(result)
            return tag, results

        with concurrent.futures.ThreadPoolExecutor(max_workers=MAX_WORKERS) as executor:
            futures = [executor.submit(test_tag, tag) for tag in TAGS]
            all_results = [future.result() for future in concurrent.futures.as_completed(futures)]

        tag_stats = {}
        for tag, results in all_results:
            response_times = [r['response_time'] for r in results if r['status_code'] == 200]
            if response_times:
                tag_stats[tag] = {
                    'avg_response_time': statistics.mean(response_times),
                    'p95_response_time': statistics.quantiles(response_times, n=20)[18],
                    'samples': len(response_times)
                }
            else:
                tag_stats[tag] = {
                    'avg_response_time': 0,
                    'p95_response_time': 0,
                    'samples': 0
                }

        self.results['tag_search'] = tag_stats

    def test_cache_pollution(self):
        print("\n=== Testing Cache Pollution ===")
        self.clear_cache()
        self.reset_cache_stats()

        hot_articles = random.sample(ARTICLE_IDS, 50)
        results_phase1 = []
        for _ in range(500):
            article_id = random.choice(hot_articles)
            url = f'{BASE_URL}/api/v1/articles/{article_id}'
            result = self.make_request(url)
            results_phase1.append(result)

        stats_phase1 = self.get_cache_stats()

        pollution_articles = random.sample([x for x in ARTICLE_IDS if x not in hot_articles], 200)
        results_phase2 = []
        for article_id in pollution_articles:
            url = f'{BASE_URL}/api/v1/articles/{article_id}'
            result = self.make_request(url)
            results_phase2.append(result)

        results_phase3 = []
        for _ in range(500):
            article_id = random.choice(hot_articles)
            url = f'{BASE_URL}/api/v1/articles/{article_id}'
            result = self.make_request(url)
            results_phase3.append(result)

        stats_phase3 = self.get_cache_stats()

        def calculate_phase_stats(results):
            times = [r['response_time'] for r in results if r['status_code'] == 200]
            if times:
                return {
                    'avg_response_time': statistics.mean(times),
                    'p95_response_time': statistics.quantiles(times, n=20)[18],
                    'requests': len(results)
                }
            else:
                return {
                    'avg_response_time': 0,
                    'p95_response_time': 0,
                    'requests': len(results)
                }

        self.results['cache_pollution'] = {
            'phase1': calculate_phase_stats(results_phase1),
            'phase2': calculate_phase_stats(results_phase2),
            'phase3': calculate_phase_stats(results_phase3),
            'initial_cache_stats': stats_phase1,
            'final_cache_stats': stats_phase3
        }

    def run_all_tests(self):
        test_start_time = time.time()
        self.test_lru_pattern()
        self.test_tag_search()
        self.test_cache_pollution()
        test_duration = time.time() - test_start_time
        self.results['test_summary'] = {
            'total_duration': test_duration,
            'timestamp': time.strftime('%Y-%m-%d %H:%M:%S')
        }
        self.generate_report()

    def generate_report(self):
        report = "\n=== Test Report ===\n"
        report += f"Test Timestamp: {self.results['test_summary']['timestamp']}\n"
        report += f"Total Test Duration: {self.results['test_summary']['total_duration']:.2f} seconds\n\n"

        report += "LRU Cache Patterns:\n"
        for pattern, data in self.results.items():
            if pattern.startswith('lru_'):
                report += f"  Pattern: {pattern.replace('lru_', '').capitalize()}\n"
                report += f"    Total Requests: {data['total_requests']}\n"
                report += f"    Successful Requests: {data['successful_requests']}\n"
                report += f"    Average Response Time: {data['avg_response_time'] * 1000:.2f} ms\n"
                report += f"    P95 Response Time: {data['p95_response_time'] * 1000:.2f} ms\n"
                cache_stats = data['cache_stats']
                if cache_stats:
                    report += f"    Cache Hit Rate: {cache_stats.get('hit_rate', 0) * 100:.2f}%\n"
        
        report += "\nTag Search Performance:\n"
        for tag, stats in self.results.get('tag_search', {}).items():
            report += f"  Tag: {tag}\n"
            report += f"    Average Response Time: {stats['avg_response_time'] * 1000:.2f} ms\n"
            report += f"    P95 Response Time: {stats['p95_response_time'] * 1000:.2f} ms\n"
            report += f"    Samples: {stats['samples']}\n"

        report += "\nCache Pollution:\n"
        pollution = self.results.get('cache_pollution', {})
        for phase, stats in pollution.items():
            if phase.startswith('phase'):
                report += f"  {phase.capitalize()}:\n"
                report += f"    Average Response Time: {stats.get('avg_response_time', 0) * 1000:.2f} ms\n"
                report += f"    P95 Response Time: {stats.get('p95_response_time', 0) * 1000:.2f} ms\n"
                report += f"    Total Requests: {stats.get('requests', 0)}\n"
        
        report += f"\nCache Stats:\n"
        init_cache = pollution.get('initial_cache_stats', {})
        final_cache = pollution.get('final_cache_stats', {})
        report += f"  Initial Cache Hit Rate: {init_cache.get('hit_rate', 0) * 100:.2f}%\n"
        report += f"  Final Cache Hit Rate: {final_cache.get('hit_rate', 0) * 100:.2f}%\n"

        print(report)
        with open('test_report.txt', 'w') as report_file:
            report_file.write(report)

if __name__ == '__main__':
    tester = LRUCacheTester()
    tester.run_all_tests()
