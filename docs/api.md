# API Documentation

This document provides detailed information about the API endpoints, input and output formats, and how to use these APIs to interact with the article management system.

## Overview

- **Base URL**: `http://127.0.0.1:8080`
- **Available Endpoints**:
  - `/health`
  - `/api/v1/articles`
  - `/api/v1/articles/pages`
  - `/api/v1/articles/{id}`
  - `/api/v1/articles/index/refresh`
  - `/api/v1/articles/cache`
  - `/api/v1/articles/{id}/refresh`
  - `/api/v1/articles/tags/{tag}`
  - `/api/v1/articles/tags/{tag}/pages`
  - `/api/v1/articles/cache/stats`
  - `/api/v1/articles/cache/stats/reset`

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

- **Example Response**
  ```json
  {
    "success": true,
    "data": "Server is running",
    "message": null
  }
  ```

---

### 2. Get Articles

Retrieve a list of articles with optional pagination.

- **Endpoint**
  ```
  GET /api/v1/articles
  ```

- **Query Parameters**
  - `limit` (optional): Maximum number of articles per page
  - `page` (optional): Page number (0-based index)

- **Responses**
  - **200 OK**: A list of article summaries is returned
  - **400 Bad Request**: Invalid pagination parameters
  - **500 Internal Server Error**: Failed to retrieve articles

- **Example Requests**
  ```
  GET /api/v1/articles
  GET /api/v1/articles?limit=10&page=0
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

### 3. Get Total Pages

Get the total number of pages for articles.

- **Endpoint**
  ```
  GET /api/v1/articles/pages
  ```

- **Query Parameters**
  - `limit` (optional): Maximum number of articles per page (default: 10)

- **Responses**
  - **200 OK**: Returns the total number of pages

- **Example Request**
  ```
  GET /api/v1/articles/pages?limit=10
  ```

- **Example Response**
  ```json
  {
    "success": true,
    "data": 5,
    "message": null
  }
  ```

---

### 4. Get Article by ID

Retrieve a specific article by its ID.

- **Endpoint**
  ```
  GET /api/v1/articles/{id}
  ```

- **Path Parameters**
  - `{id}`: The integer ID of the article to retrieve

- **Responses**
  - **200 OK**: The article was found and returned
  - **404 Not Found**: Article not found

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

### 5. Refresh Index

Refresh the articles index by reloading from the filesystem.

- **Endpoint**
  ```
  POST /api/v1/articles/index/refresh
  ```

- **Responses**
  - **200 OK**: Index refreshed successfully
  - **500 Internal Server Error**: Failed to refresh index

- **Example Response**
  ```json
  {
    "success": true,
    "data": null,
    "message": "Index refreshed"
  }
  ```

---

### 6. Clear Cache

Clear the articles cache.

- **Endpoint**
  ```
  DELETE /api/v1/articles/cache
  ```

- **Responses**
  - **200 OK**: Cache cleared successfully

- **Example Response**
  ```json
  {
    "success": true,
    "data": null,
    "message": "Cache cleared"
  }
  ```

---

### 7. Refresh Article

Refresh a specific article in the cache.

- **Endpoint**
  ```
  POST /api/v1/articles/{id}/refresh
  ```

- **Path Parameters**
  - `{id}`: The integer ID of the article to refresh

- **Responses**
  - **200 OK**: Article refreshed successfully
  - **500 Internal Server Error**: Failed to refresh article

- **Example Response**
  ```json
  {
    "success": true,
    "data": null,
    "message": "Article refreshed"
  }
  ```

---

### 8. Get Articles by Tag

Retrieve articles filtered by a specific tag with optional pagination.

- **Endpoint**
  ```
  GET /api/v1/articles/tags/{tag}
  ```

- **Path Parameters**
  - `{tag}`: The tag to filter articles by

- **Query Parameters**
  - `limit` (optional): Maximum number of articles per page
  - `page` (optional): Page number (0-based index)

- **Responses**
  - **200 OK**: List of articles with the specified tag
  - **400 Bad Request**: Invalid pagination parameters
  - **500 Internal Server Error**: Failed to retrieve articles

- **Example Requests**
  ```
  GET /api/v1/articles/tags/sample
  GET /api/v1/articles/tags/sample?limit=10&page=0
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

### 9. Get Tag Pages

Get the total number of pages for articles with a specific tag.

- **Endpoint**
  ```
  GET /api/v1/articles/tags/{tag}/pages
  ```

- **Path Parameters**
  - `{tag}`: The tag to count pages for

- **Query Parameters**
  - `limit` (optional): Maximum number of articles per page (default: 10)

- **Responses**
  - **200 OK**: Returns the total number of pages for the tag

- **Example Request**
  ```
  GET /api/v1/articles/tags/sample/pages?limit=10
  ```

- **Example Response**
  ```json
  {
    "success": true,
    "data": 3,
    "message": null
  }
  ```

---

### 10. Get Cache Statistics

Retrieve statistics about cache usage.

- **Endpoint**
  ```
  GET /api/v1/articles/cache/stats
  ```

- **Responses**
  - **200 OK**: Cache statistics retrieved successfully

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

### 11. Reset Cache Statistics

Reset the cache statistics counters.

- **Endpoint**
  ```
  POST /api/v1/articles/cache/stats/reset
  ```

- **Responses**
  - **200 OK**: Cache statistics reset successfully

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
  - `success` (boolean): Indicates whether the request was successful
  - `data` (varies): The data payload. Can be an object, array, or null
  - `message` (string|null): Optional message providing additional information

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
  - `id` (integer): Unique identifier of the article
  - `title` (string): Title of the article
  - `description` (string): Brief description of the article
  - `content` (string): The HTML content of the article
  - `date` (integer): Publication date represented as an integer (YYYYMMDD)
  - `tags` (array of strings): List of tags associated with the article
  - `keywords` (array of strings): List of keywords for the article

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
  - `id` (integer): Unique identifier of the article
  - `title` (string): Title of the article
  - `description` (string): Brief description of the article
  - `date` (integer): Publication date represented as an integer (YYYYMMDD)
  - `tags` (array of strings): List of tags associated with the article
  - `keywords` (array of strings): List of keywords for the article

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
  - `cache_hit` (integer): Number of cache hits
  - `cache_miss` (integer): Number of cache misses
  - `hit_rate` (float): Percentage of cache hits out of total cache requests

---

## Notes

- **Pagination**: Many endpoints support pagination through optional `limit` and `page` query parameters
- **Sample Article**: If the configuration includes the sample article, ID `0` is reserved for it
- **Default Page Size**: When using pagination, the default page size is 10 items per page
- **Page Numbers**: Page numbers are 0-based indices
- **Cache Management**: Cache-related endpoints are now consolidated under the `/api/v1/articles/cache` path