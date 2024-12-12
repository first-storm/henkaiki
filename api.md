# API Documentation

This document provides detailed information about the API endpoints, input and output formats, and how to use these APIs to interact with the article management system.

## Overview

- **Base URL**: `http://127.0.0.1:8080`
- **Available Endpoints**:
  - `/health`
  - `/api/v1/articles`
  - `/api/v1/articles/{id}`
  - `/api/v1/tags/{tag}/articles`
  - `/api/v1/admin/articles/{id}/refresh`
  - `/api/v1/admin/cache/clear`
  - `/api/v1/admin/index/refresh`
  - `/api/v1/admin/cache/stats`
  - `/api/v1/admin/cache/stats/reset`

---

## Endpoints

### 1. Health Check

Check the health status of the server.

- **Endpoint**

  ```
  GET /health
  ```

- **Responses**

  - **200 OK**: The server is running.
    - **Body**: JSON object containing the health status.

- **Response Format**

  ```json
  {
    "success": true,
    "data": "Server is running",
    "message": null
  }
  ```

- **Example Request**

  ```
  GET /health
  ```

- **Example Response**

  ```json
  {
    "success": true,
    "data": "Server is running",
    "message": null
  }
  ```

---

### 2. Get All Articles

Retrieve a list of all articles without their content.

- **Endpoint**

  ```
  GET /api/v1/articles
  ```

- **Responses**

  - **200 OK**: A list of article summaries is returned.
    - **Body**: [ApiResponse](#apiresponse-object) containing an array of [ArticleSummary](#article-summary-object) objects.
  - **500 Internal Server Error**: Failed to retrieve articles.
    - **Body**: [ApiResponse](#apiresponse-object) with an error message.

- **Example Request**

  ```
  GET /api/v1/articles
  ```

- **Example Response**

  ```json
  {
    "success": true,
    "data": [
      {
        "id": 1,
        "title": "Sample Article",
        "description": "A sample article summary.",
        "date": 20231015,
        "tags": ["sample", "demo"],
        "keywords": ["example", "sample article"]
      },
      {
        "id": 2,
        "title": "Another Article",
        "description": "Summary of another article.",
        "date": 20231016,
        "tags": ["other"],
        "keywords": ["demo", "testing"]
      }
    ],
    "message": null
  }
  ```

---

### 3. Get Article by ID

Retrieve a specific article by its ID.

- **Endpoint**

  ```
  GET /api/v1/articles/{id}
  ```

- **Path Parameters**

  - `{id}`: The integer ID of the article to retrieve.

- **Responses**

  - **200 OK**: The article was found and returned.
    - **Body**: [ApiResponse](#apiresponse-object) containing an [Article](#article-object).
  - **404 Not Found**: The article with the specified ID does not exist.
    - **Body**: [ApiResponse](#apiresponse-object) with an error message.

- **Example Request**

  ```
  GET /api/v1/articles/1
  ```

- **Example Response**

  ```json
  {
    "success": true,
    "data": {
      "id": 1,
      "title": "Sample Article",
      "description": "A sample article for demonstration purposes.",
      "content": "<p>This is the content of the sample article.</p>",
      "date": 20231015,
      "tags": ["sample", "demo"],
      "keywords": ["example", "sample article"]
    },
    "message": null
  }
  ```

---

### 4. Get Articles by Tag

Retrieve a list of articles associated with a specific tag.

- **Endpoint**

  ```
  GET /api/v1/tags/{tag}/articles
  ```

- **Path Parameters**

  - `{tag}`: The tag to filter articles by.

- **Responses**

  - **200 OK**: A list of articles with the specified tag is returned.
    - **Body**: [ApiResponse](#apiresponse-object) containing an array of [ArticleSummary](#article-summary-object) objects.
  - **500 Internal Server Error**: Failed to retrieve articles by tag.
    - **Body**: [ApiResponse](#apiresponse-object) with an error message.

- **Example Request**

  ```
  GET /api/v1/tags/sample/articles
  ```

- **Example Response**

  ```json
  {
    "success": true,
    "data": [
      {
        "id": 1,
        "title": "Sample Article",
        "description": "A sample article summary.",
        "date": 20231015,
        "tags": ["sample", "demo"],
        "keywords": ["example", "sample article"]
      }
    ],
    "message": null
  }
  ```

---

### 5. Refresh Article Cache by ID (Admin)

Refresh the cache for a specific article by its ID. This reloads the article from the filesystem and updates the cache.

- **Endpoint**

  ```
  POST /api/v1/admin/articles/{id}/refresh
  ```

- **Path Parameters**

  - `{id}`: The integer ID of the article to refresh.

- **Responses**

  - **200 OK**: The article cache was successfully refreshed.
    - **Body**: [ApiResponse](#apiresponse-object) with a success message.
  - **500 Internal Server Error**: Failed to refresh the article cache.
    - **Body**: [ApiResponse](#apiresponse-object) with an error message.

- **Example Request**

  ```
  POST /api/v1/admin/articles/1/refresh
  ```

- **Example Response**

  ```json
  {
    "success": true,
    "data": null,
    "message": "Article cache refreshed"
  }
  ```

---

### 6. Clear Article Cache (Admin)

Clear the server's article cache. This forces the server to reload articles from the filesystem on the next request.

- **Endpoint**

  ```
  POST /api/v1/admin/cache/clear
  ```

- **Responses**

  - **200 OK**: The cache was successfully cleared.
    - **Body**: [ApiResponse](#apiresponse-object) with a success message.

- **Example Request**

  ```
  POST /api/v1/admin/cache/clear
  ```

- **Example Response**

  ```json
  {
    "success": true,
    "data": null,
    "message": "Cache cleared"
  }
  ```

---

### 7. Refresh Article Index (Admin)

Refresh the server's article index. This reloads the index of articles from the filesystem.

- **Endpoint**

  ```
  POST /api/v1/admin/index/refresh
  ```

- **Responses**

  - **200 OK**: The article index was successfully refreshed.
    - **Body**: [ApiResponse](#apiresponse-object) with a success message.
  - **500 Internal Server Error**: Failed to refresh the article index.
    - **Body**: [ApiResponse](#apiresponse-object) with an error message.

- **Example Request**

  ```
  POST /api/v1/admin/index/refresh
  ```

- **Example Response**

  ```json
  {
    "success": true,
    "data": null,
    "message": "Index refreshed"
  }
  ```

---

### 8. Get Cache Statistics (Admin)

Retrieve statistics about cache usage, including cache hits, misses, and hit rate.

- **Endpoint**

  ```
  GET /api/v1/admin/cache/stats
  ```

- **Responses**

  - **200 OK**: Cache statistics are returned.
    - **Body**: [ApiResponse](#apiresponse-object) containing a [CacheStats](#cachestats-object) object.
  - **500 Internal Server Error**: Failed to retrieve cache statistics.
    - **Body**: [ApiResponse](#apiresponse-object) with an error message.

- **Example Request**

  ```
  GET /api/v1/admin/cache/stats
  ```

- **Example Response**

  ```json
  {
    "success": true,
    "data": {
      "cache_hit": 150,
      "cache_miss": 50,
      "hit_rate": 75.0
    },
    "message": null
  }
  ```

---

### 9. Reset Cache Statistics (Admin)

Reset the cache statistics counters for cache hits and misses.

- **Endpoint**

  ```
  POST /api/v1/admin/cache/stats/reset
  ```

- **Responses**

  - **200 OK**: Cache statistics have been reset.
    - **Body**: [ApiResponse](#apiresponse-object) with a success message.
  - **500 Internal Server Error**: Failed to reset cache statistics.
    - **Body**: [ApiResponse](#apiresponse-object) with an error message.

- **Example Request**

  ```
  POST /api/v1/admin/cache/stats/reset
  ```

- **Example Response**

  ```json
  {
    "success": true,
    "data": null,
    "message": "Cache statistics have been reset"
  }
  ```

---

## Data Models

### ApiResponse Object

Generic response object for all API responses.

```json
{
  "success": true,
  "data": {...},
  "message": "Optional message"
}
```

- **Fields**
  - `success` (boolean): Indicates whether the request was successful.
  - `data` (varies): The data payload. Can be an object, array, or null.
  - `message` (string|null): Optional message providing additional information.

### Article Object

Represents a full article with content.

```json
{
  "id": 1,
  "title": "Article Title",
  "description": "Short description of the article.",
  "content": "<p>HTML content of the article.</p>",
  "date": 20231015,
  "tags": ["tag1", "tag2"],
  "keywords": ["keyword1", "keyword2"]
}
```

- **Fields**
  - `id` (integer): Unique identifier of the article.
  - `title` (string): Title of the article.
  - `description` (string): Brief description of the article.
  - `content` (string): The HTML content of the article.
  - `date` (integer): Publication date represented as an integer (e.g., YYYYMMDD).
  - `tags` (array of strings): List of tags associated with the article.
  - `keywords` (array of strings): List of keywords for the article.

### Article Summary Object

Represents a summary of an article without the full content.

```json
{
  "id": 1,
  "title": "Article Title",
  "description": "Short description of the article.",
  "date": 20231015,
  "tags": ["tag1", "tag2"],
  "keywords": ["keyword1", "keyword2"]
}
```

- **Fields**
  - `id` (integer): Unique identifier of the article.
  - `title` (string): Title of the article.
  - `description` (string): Brief description of the article.
  - `date` (integer): Publication date represented as an integer (e.g., YYYYMMDD).
  - `tags` (array of strings): List of tags associated with the article.
  - `keywords` (array of strings): List of keywords for the article.

### CacheStats Object

Represents statistics about cache usage.

```json
{
  "cache_hit": 150,
  "cache_miss": 50,
  "hit_rate": 75.0
}
```

- **Fields**
  - `cache_hit` (integer): Number of cache hits.
  - `cache_miss` (integer): Number of cache misses.
  - `hit_rate` (float): Percentage of cache hits out of total cache requests.

---

## Notes

- **New Fields Added**: Articles and summaries now include a `keywords` field.
- **Sample Article**: If the configuration includes the sample article, ID `0` is reserved for it.
- **Cache Statistics Endpoints**: New administrative endpoints have been added to monitor and manage cache performance.