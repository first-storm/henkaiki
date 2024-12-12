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
    - **Body**: [ApiResponse](#apiresponse-object) object containing an array of [ArticleSummary](#article-summary-object) objects.
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
        "tags": ["sample", "demo"]
      },
      {
        "id": 2,
        "title": "Another Article",
        "description": "Summary of another article.",
        "date": 20231016,
        "tags": ["other"]
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
      "tags": ["sample", "demo"]
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
        "tags": ["sample", "demo"]
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
  "tags": ["tag1", "tag2"]
}
```

- **Fields**
  - `id` (integer): Unique identifier of the article.
  - `title` (string): Title of the article.
  - `description` (string): Brief description of the article.
  - `content` (string): The HTML content of the article.
  - `date` (integer): Publication date represented as an integer (e.g., YYYYMMDD).
  - `tags` (array of strings): List of tags associated with the article.

### Article Summary Object

Represents a summary of an article without the full content.

```json
{
  "id": 1,
  "title": "Article Title",
  "description": "Short description of the article.",
  "date": 20231015,
  "tags": ["tag1", "tag2"]
}
```

- **Fields**
  - `id` (integer): Unique identifier of the article.
  - `title` (string): Title of the article.
  - `description` (string): Brief description of the article.
  - `date` (integer): Publication date represented as an integer (e.g., YYYYMMDD).
  - `tags` (array of strings): List of tags associated with the article.

---

## How to Use the APIs

### Prerequisites

- Ensure the server is running on `http://127.0.0.1:8080`.
- Use any HTTP client (e.g., curl, Postman, browser) to interact with the API.

### Health Check

To check the health status of the server, send a `GET` request to `/health`.

**Example with `curl`:**

```sh
curl -X GET http://127.0.0.1:8080/health
```

### Retrieving All Articles

To get a list of all articles without content, send a `GET` request to `/api/v1/articles`.

**Example with `curl`:**

```sh
curl -X GET http://127.0.0.1:8080/api/v1/articles
```

### Retrieving an Article by ID

To get a specific article, send a `GET` request to `/api/v1/articles/{id}`.

**Example with `curl`:**

```sh
curl -X GET http://127.0.0.1:8080/api/v1/articles/1
```

### Retrieving Articles by Tag

To get a list of articles associated with a specific tag, send a `GET` request to `/api/v1/tags/{tag}/articles`.

**Example with `curl`:**

```sh
curl -X GET http://127.0.0.1:8080/api/v1/tags/sample/articles
```

### Refreshing the Cache for a Specific Article (Admin)

To refresh the cache for a specific article, send a `POST` request to `/api/v1/admin/articles/{id}/refresh`.

**Example with `curl`:**

```sh
curl -X POST http://127.0.0.1:8080/api/v1/admin/articles/1/refresh
```

### Clearing the Article Cache (Admin)

To clear the server's article cache, send a `POST` request to `/api/v1/admin/cache/clear`.

**Example with `curl`:**

```sh
curl -X POST http://127.0.0.1:8080/api/v1/admin/cache/clear
```

### Refreshing the Article Index (Admin)

To refresh the article index, send a `POST` request to `/api/v1/admin/index/refresh`.

**Example with `curl`:**

```sh
curl -X POST http://127.0.0.1:8080/api/v1/admin/index/refresh
```

---

## Input and Output Formats

### Input Formats

- **GET Requests**: No request body required.
- **POST Requests**: No request body required.

### Output Formats

- **Content-Type**: `application/json` for all responses.

### Error Responses

In case of errors, the API will return an appropriate HTTP status code along with an [ApiResponse](#apiresponse-object) containing the error message.

- **404 Not Found**: The requested resource does not exist.
- **500 Internal Server Error**: An unexpected error occurred on the server.

**Example Error Response:**

```json
{
  "success": false,
  "data": null,
  "message": "Article not found"
}
```

---

## Examples

### Example 1: Health Check

**Request:**

```sh
curl -X GET http://127.0.0.1:8080/health
```

**Response:**

```json
{
  "success": true,
  "data": "Server is running",
  "message": null
}
```

### Example 2: Retrieve All Articles

**Request:**

```sh
curl -X GET http://127.0.0.1:8080/api/v1/articles
```

**Response:**

```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "title": "Sample Article",
      "description": "A sample article summary.",
      "date": 20231015,
      "tags": ["sample", "demo"]
    },
    {
      "id": 2,
      "title": "Another Article",
      "description": "Summary of another article.",
      "date": 20231016,
      "tags": ["other"]
    }
  ],
  "message": null
}
```

### Example 3: Retrieve Article with ID 1

**Request:**

```sh
curl -X GET http://127.0.0.1:8080/api/v1/articles/1
```

**Response:**

```json
{
  "success": true,
  "data": {
    "id": 1,
    "title": "Sample Article",
    "description": "A sample article for demonstration purposes.",
    "content": "<p>This is the content of the sample article.</p>",
    "date": 20231015,
    "tags": ["sample", "demo"]
  },
  "message": null
}
```

### Example 4: Retrieve Articles by Tag

**Request:**

```sh
curl -X GET http://127.0.0.1:8080/api/v1/tags/sample/articles
```

**Response:**

```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "title": "Sample Article",
      "description": "A sample article summary.",
      "date": 20231015,
      "tags": ["sample", "demo"]
    }
  ],
  "message": null
}
```

### Example 5: Refresh Cache for Article with ID 1 (Admin)

**Request:**

```sh
curl -X POST http://127.0.0.1:8080/api/v1/admin/articles/1/refresh
```

**Response:**

```json
{
  "success": true,
  "data": null,
  "message": "Article cache refreshed"
}
```

### Example 6: Clear Article Cache (Admin)

**Request:**

```sh
curl -X POST http://127.0.0.1:8080/api/v1/admin/cache/clear
```

**Response:**

```json
{
  "success": true,
  "data": null,
  "message": "Cache cleared"
}
```

### Example 7: Refresh Article Index (Admin)

**Request:**

```sh
curl -X POST http://127.0.0.1:8080/api/v1/admin/index/refresh
```

**Response:**

```json
{
  "success": true,
  "data": null,
  "message": "Index refreshed"
}
```

---

## Additional Information

### Article Storage

Articles are stored in the filesystem in a designated directory specified in the application's configuration. Each article has a unique ID that corresponds to a directory containing:

- `metainfo.toml`: Metadata about the article.
- Markdown files: The content of the article in markdown format.

### Sample Article

If the configuration enables the sample article, an article with ID `0` (zero) is available. This article provides an example of how articles are structured and served.

### Caching

The application uses an in-memory Least Recently Used (LRU) cache to store articles. The cache size is configurable. Clearing or refreshing the cache can help in development or if the underlying files have changed.

### Configuration

Key configuration parameters:

- **Articles Directory**: The directory where articles are stored.
- **Max Cached Articles**: The maximum number of articles to keep in the cache.
- **Sample Article**: A boolean flag indicating whether to include the sample article.

---

## Changelog

- **Updated**: Changed HTTP methods for admin endpoints to `POST` to align with the API implementation.
- **Updated**: All responses are now JSON objects conforming to the [ApiResponse](#apiresponse-object) structure.
- **Addition**: Included example responses using the updated response format.
- **Removal**: Removed plain text responses; all responses are now in JSON format.

---

## Notes

- Ensure that any client interacting with the API correctly handles JSON responses and interprets the `success`, `data`, and `message` fields appropriately.
- The admin endpoints are intended for administrative use and may require authentication if implemented in the future.