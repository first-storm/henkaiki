import requests
import time
import random
import concurrent.futures
import statistics
from collections import defaultdict
import json

# 基础配置
BASE_URL = 'http://127.0.0.1:8080'
ARTICLE_IDS = list(range(1, 1001))
TAGS = ['技术', '编程', '教程', '开发', '软件', '云计算', '人工智能', '数据库', '前端', '后端']
CACHE_SIZE = 100
MAX_WORKERS = 10  # 并发线程数

class LRUCacheTester:
    def __init__(self):
        self.session = requests.Session()
        self.results = defaultdict(dict)
        
    def clear_cache(self):
        url = f'{BASE_URL}/api/v1/admin/cache/clear'
        try:
            response = self.session.post(url)
            return response.status_code == 200
        except requests.RequestException as e:
            print(f"清除缓存失败: {e}")
            return False

    def reset_cache_stats(self):
        url = f'{BASE_URL}/api/v1/admin/cache/stats/reset'
        try:
            response = self.session.post(url)
            return response.status_code == 200
        except requests.RequestException as e:
            print(f"重置缓存统计失败: {e}")
            return False

    def get_cache_stats(self):
        url = f'{BASE_URL}/api/v1/admin/cache/stats'
        try:
            response = self.session.get(url)
            if response.status_code == 200:
                return response.json().get('data', {})
            return None
        except requests.RequestException as e:
            print(f"获取缓存统计失败: {e}")
            return None

    def make_request(self, url):
        """每个请求使用独立的Session，避免线程安全问题"""
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
            print(f"请求失败: {e}")
            return {
                'response_time': end_time - start_time,
                'status_code': None,
                'url': url
            }

    def test_lru_pattern(self):
        """测试LRU缓存在不同访问模式下的性能"""
        print("\n=== LRU缓存模式测试 ===")
        self.clear_cache()  # 开始测试前清空缓存
        test_patterns = {
            'hot_items': self._generate_hot_items_pattern(),
            'sequential': self._generate_sequential_pattern(),
            'random': self._generate_random_pattern()
        }

        for pattern_name, urls in test_patterns.items():
            print(f"\n测试模式: {pattern_name}")
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
        """生成热点数据访问模式：80%的请求访问20%的文章"""
        hot_articles = random.sample(ARTICLE_IDS, int(len(ARTICLE_IDS) * 0.2))
        urls = []
        for _ in range(total_requests):
            if random.random() < 0.8:  # 80%概率访问热点文章
                article_id = random.choice(hot_articles)
            else:
                article_id = random.choice(ARTICLE_IDS)
            urls.append(f'{BASE_URL}/api/v1/articles/{article_id}')
        return urls

    def _generate_sequential_pattern(self, total_requests=1000):
        """生成顺序访问模式：模拟扫描式访问"""
        urls = []
        current_position = 0
        while len(urls) < total_requests:
            article_id = ARTICLE_IDS[current_position % len(ARTICLE_IDS)]
            urls.append(f'{BASE_URL}/api/v1/articles/{article_id}')
            current_position += 1
        return urls

    def _generate_random_pattern(self, total_requests=1000):
        """生成随机访问模式"""
        return [f'{BASE_URL}/api/v1/articles/{random.choice(ARTICLE_IDS)}' 
                for _ in range(total_requests)]

    def test_cache_pollution(self):
        """测试缓存污染情况"""
        print("\n=== 缓存污染测试 ===")
        self.clear_cache()
        self.reset_cache_stats()
        
        # 第一阶段：建立热点数据缓存
        hot_articles = random.sample(ARTICLE_IDS, 50)  # 选择50个热点文章
        results_phase1 = []
        for _ in range(500):  # 反复访问热点文章
            article_id = random.choice(hot_articles)
            url = f'{BASE_URL}/api/v1/articles/{article_id}'
            result = self.make_request(url)
            results_phase1.append(result)
            
        stats_phase1 = self.get_cache_stats()
        
        # 第二阶段：模拟缓存污染（一次性访问大量不同文章）
        pollution_articles = random.sample([x for x in ARTICLE_IDS if x not in hot_articles], 200)
        results_phase2 = []
        for article_id in pollution_articles:
            url = f'{BASE_URL}/api/v1/articles/{article_id}'
            result = self.make_request(url)
            results_phase2.append(result)
            
        # 第三阶段：再次访问热点文章
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

    def test_tag_search(self):
        """并发测试标签搜索性能"""
        print("\n=== 标签搜索性能测试 ===")
        self.clear_cache()  # 开始测试前清空缓存
        self.reset_cache_stats()  # 重置缓存统计
        
        def test_tag(tag):
            results = []
            for _ in range(50):  # 每个标签测试50次
                url = f'{BASE_URL}/api/v1/tags/{tag}/articles'
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

    def test_list_all_articles(self):
        """测试获取所有文章列表的性能"""
        print("\n=== 文章列表性能测试 ===")
        self.clear_cache()  # 开始测试前清空缓存
        self.reset_cache_stats()  # 重置缓存统计
        
        results = []
        for _ in range(50):  # 测试50次
            result = self.make_request(f'{BASE_URL}/api/v1/articles')
            results.append(result)
            
        response_times = [r['response_time'] for r in results if r['status_code'] == 200]
        
        if response_times:
            self.results['list_all_articles'] = {
                'avg_response_time': statistics.mean(response_times),
                'p95_response_time': statistics.quantiles(response_times, n=20)[18],
                'min_response_time': min(response_times),
                'max_response_time': max(response_times),
                'samples': len(response_times)
            }
        else:
            self.results['list_all_articles'] = {
                'avg_response_time': 0,
                'p95_response_time': 0,
                'min_response_time': 0,
                'max_response_time': 0,
                'samples': 0
            }

    def test_in_cache_performance(self):
        """测试缓存容量内的性能表现"""
        print("\n=== 缓存空间内性能测试 ===")
        self.clear_cache()
        self.reset_cache_stats()
        
        # 使用等于缓存大小的文章ID集合
        in_cache_ids = random.sample(ARTICLE_IDS, CACHE_SIZE)
        
        # 第一轮：预热缓存
        print("预热缓存...")
        warmup_results = []
        for article_id in in_cache_ids:
            url = f'{BASE_URL}/api/v1/articles/{article_id}'
            result = self.make_request(url)
            warmup_results.append(result)
        
        # 清空缓存统计并开始第二轮测试
        self.reset_cache_stats()
        print("测试缓存内访问性能...")
        test_urls = [f'{BASE_URL}/api/v1/articles/{random.choice(in_cache_ids)}' for _ in range(1000)]
        
        with concurrent.futures.ThreadPoolExecutor(max_workers=MAX_WORKERS) as executor:
            test_results = list(executor.map(self.make_request, test_urls))
            
        # 计算性能指标
        warmup_times = [r['response_time'] for r in warmup_results if r['status_code'] == 200]
        test_times = [r['response_time'] for r in test_results if r['status_code'] == 200]
        
        cache_stats = self.get_cache_stats()
        hit_rate_pct = cache_stats['hit_rate'] * 100 if cache_stats else 0
        
        self.results['in_cache_performance'] = {
            'warmup_phase': {
                'avg_response_time': statistics.mean(warmup_times) if warmup_times else 0,
                'p95_response_time': statistics.quantiles(warmup_times, n=20)[18] if warmup_times else 0
            },
            'test_phase': {
                'avg_response_time': statistics.mean(test_times) if test_times else 0,
                'p95_response_time': statistics.quantiles(test_times, n=20)[18] if test_times else 0,
                'min_response_time': min(test_times) if test_times else 0,
                'max_response_time': max(test_times) if test_times else 0
            },
            'cache_stats': cache_stats
        }

    def run_all_tests(self):
        test_start_time = time.time()
        
        # 执行所有测试
        self.test_lru_pattern()
        self.test_cache_pollution()
        self.test_tag_search()
        self.test_list_all_articles()
        self.test_in_cache_performance()
        
        test_duration = time.time() - test_start_time
        self.results['test_summary'] = {
            'total_duration': test_duration,
            'timestamp': time.strftime('%Y-%m-%d %H:%M:%S')
        }
        
        self.generate_report()

    def generate_report(self):
        print("\n====== LRU缓存性能测试报告 ======")
        print(f"\n测试执行时间: {self.results['test_summary']['timestamp']}")
        print(f"总测试时长: {self.results['test_summary']['total_duration']:.2f} 秒")
        
        print("\n1. LRU缓存模式测试结果:")
        for pattern in ['hot_items', 'sequential', 'random']:
            data = self.results.get(f'lru_{pattern}', {})
            cache_stats = data.get('cache_stats', {})
            hit_rate_pct = cache_stats.get('hit_rate', 0) * 100
            print(f"\n{pattern}模式:")
            print(f"  - 总请求数: {data.get('total_requests', 0)}")
            print(f"  - 平均响应时间: {data.get('avg_response_time', 0)*1000:.2f} ms")
            print(f"  - 95分位响应时间: {data.get('p95_response_time', 0)*1000:.2f} ms")
            print(f"  - 缓存命中率: {hit_rate_pct:.2f}%")
        
        print("\n2. 缓存污染测试结果:")
        pollution_data = self.results.get('cache_pollution', {})
        print("阶段1 (热点数据):")
        print(f"  - 平均响应时间: {pollution_data.get('phase1', {}).get('avg_response_time', 0)*1000:.2f} ms")
        init_hit_rate = pollution_data.get('initial_cache_stats', {}).get('hit_rate', 0) * 100
        print(f"  - 缓存命中率: {init_hit_rate:.2f}%")
        print("阶段3 (污染后热点数据):")
        print(f"  - 平均响应时间: {pollution_data.get('phase3', {}).get('avg_response_time', 0)*1000:.2f} ms")
        final_hit_rate = pollution_data.get('final_cache_stats', {}).get('hit_rate', 0) * 100
        print(f"  - 缓存命中率: {final_hit_rate:.2f}%")
        
        print("\n3. 标签搜索性能:")
        tag_search_results = self.results.get('tag_search', {})
        for tag, stats in tag_search_results.items():
            print(f"  {tag}:")
            print(f"  - 平均响应时间: {stats.get('avg_response_time', 0)*1000:.2f} ms")
            print(f"  - 95分位响应时间: {stats.get('p95_response_time', 0)*1000:.2f} ms")
        
        print("\n4. 文章列表性能:")
        list_stats = self.results.get('list_all_articles', {})
        print(f"  - 平均响应时间: {list_stats.get('avg_response_time', 0)*1000:.2f} ms")
        print(f"  - 95分位响应时间: {list_stats.get('p95_response_time', 0)*1000:.2f} ms")
        print(f"  - 最短响应时间: {list_stats.get('min_response_time', 0)*1000:.2f} ms")
        print(f"  - 最长响应时间: {list_stats.get('max_response_time', 0)*1000:.2f} ms")
        
        print("\n5. 缓存空间内性能:")
        cache_stats = self.results.get('in_cache_performance', {})
        print("预热阶段:")
        warmup_phase = cache_stats.get('warmup_phase', {})
        print(f"  - 平均响应时间: {warmup_phase.get('avg_response_time', 0)*1000:.2f} ms")
        print(f"  - 95分位响应时间: {warmup_phase.get('p95_response_time', 0)*1000:.2f} ms")
        print("\n测试阶段:")
        test_phase = cache_stats.get('test_phase', {})
        print(f"  - 平均响应时间: {test_phase.get('avg_response_time', 0)*1000:.2f} ms")
        print(f"  - 95分位响应时间: {test_phase.get('p95_response_time', 0)*1000:.2f} ms")
        print(f"  - 最短响应时间: {test_phase.get('min_response_time', 0)*1000:.2f} ms")
        print(f"  - 最长响应时间: {test_phase.get('max_response_time', 0)*1000:.2f} ms")
        hit_rate_pct = cache_stats.get('cache_stats', {}).get('hit_rate', 0) * 100
        print(f"  - 缓存命中率: {hit_rate_pct:.2f}%")
        
        # 保存详细报告到文件
        with open('lru_cache_test_report.json', 'w', encoding='utf-8') as f:
            json.dump(self.results, f, ensure_ascii=False, indent=2)
        print("\n详细测试报告已保存至 lru_cache_test_report.json")

if __name__ == '__main__':
    tester = LRUCacheTester()
    tester.run_all_tests()

