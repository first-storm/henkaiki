#!/usr/bin/env python3

import requests


BASE_URL = "http://127.0.0.1:8080"

def main():
    """
    A simple script to test the article management API endpoints.
    """

    # 1. Health Check
    print("1) Health Check")
    try:
        resp = requests.get(f"{BASE_URL}/health")
        print("GET /health:", resp.status_code, resp.json())
    except Exception as e:
        print("Exception occurred:", e)

    # 2. Get Articles (with optional pagination)
    print("\n2) Get Articles")
    try:
        # Example without pagination
        resp = requests.get(f"{BASE_URL}/api/v1/articles")
        print("GET /api/v1/articles:", resp.status_code, resp.json())

        # Example with pagination
        params = {'limit': 2, 'page': 0}
        resp = requests.get(f"{BASE_URL}/api/v1/articles", params=params)
        print("GET /api/v1/articles?limit=2&page=0:", resp.status_code, resp.json())
    except Exception as e:
        print("Exception occurred:", e)

    # 3. Get Total Pages
    print("\n3) Get Total Pages")
    try:
        params = {'limit': 2}
        resp = requests.get(f"{BASE_URL}/api/v1/articles/pages", params=params)
        print("GET /api/v1/articles/pages?limit=2:", resp.status_code, resp.json())
    except Exception as e:
        print("Exception occurred:", e)

    # 4. Get Article by ID
    print("\n4) Get Article by ID")
    try:
        article_id = 0
        resp = requests.get(f"{BASE_URL}/api/v1/articles/{article_id}")
        print(f"GET /api/v1/articles/{article_id}:", resp.status_code, resp.json())
    except Exception as e:
        print("Exception occurred:", e)

    # 5. Refresh Index
    print("\n5) Refresh Index")
    try:
        resp = requests.post(f"{BASE_URL}/api/v1/articles/index/refresh")
        print("POST /api/v1/articles/index/refresh:", resp.status_code, resp.json())
    except Exception as e:
        print("Exception occurred:", e)

    # 6. Clear Cache
    print("\n6) Clear Cache")
    try:
        resp = requests.delete(f"{BASE_URL}/api/v1/articles/cache")
        print("DELETE /api/v1/articles/cache:", resp.status_code, resp.json())
    except Exception as e:
        print("Exception occurred:", e)

    # 7. Refresh Article
    print("\n7) Refresh Article")
    try:
        article_id = 1
        resp = requests.post(f"{BASE_URL}/api/v1/articles/{article_id}/refresh")
        print(f"POST /api/v1/articles/{article_id}/refresh:", resp.status_code, resp.json())
    except Exception as e:
        print("Exception occurred:", e)

    # 8. Get Articles by Tag
    print("\n8) Get Articles by Tag")
    try:
        tag_name = "sample"
        resp = requests.get(f"{BASE_URL}/api/v1/articles/tags/{tag_name}")
        print(f"GET /api/v1/articles/tags/{tag_name}:", resp.status_code, resp.json())
    except Exception as e:
        print("Exception occurred:", e)

    # 9. Get Tag Pages
    print("\n9) Get Tag Pages")
    try:
        tag_name = "sample"
        params = {'limit': 2}
        resp = requests.get(f"{BASE_URL}/api/v1/articles/tags/{tag_name}/pages", params=params)
        print(f"GET /api/v1/articles/tags/{tag_name}/pages?limit=2:", resp.status_code, resp.json())
    except Exception as e:
        print("Exception occurred:", e)

    # 10. Get Cache Statistics
    print("\n10) Get Cache Statistics")
    try:
        resp = requests.get(f"{BASE_URL}/api/v1/articles/cache/stats")
        print("GET /api/v1/articles/cache/stats:", resp.status_code, resp.json())
    except Exception as e:
        print("Exception occurred:", e)

    # 11. Reset Cache Statistics
    print("\n11) Reset Cache Statistics")
    try:
        resp = requests.post(f"{BASE_URL}/api/v1/articles/cache/stats/reset")
        print("POST /api/v1/articles/cache/stats/reset:", resp.status_code, resp.json())
    except Exception as e:
        print("Exception occurred:", e)

    # 12. Search Articles
    print("\n12) Search Articles")
    try:
        params = {'query': 'sample', 'limit': 2, 'page': 0}
        resp = requests.get(f"{BASE_URL}/api/v1/articles/search", params=params)
        print("GET /api/v1/articles/search?query=sample&limit=2&page=0:", resp.status_code, resp.json())
    except Exception as e:
        print("Exception occurred:", e)

    # 13. Get Search Pages
    print("\n13) Get Search Pages")
    try:
        params = {'query': 'sample', 'limit': 2}
        resp = requests.get(f"{BASE_URL}/api/v1/articles/search/pages", params=params)
        print("GET /api/v1/articles/search/pages?query=sample&limit=2:", resp.status_code, resp.json())
    except Exception as e:
        print("Exception occurred:", e)


if __name__ == "__main__":
    main()