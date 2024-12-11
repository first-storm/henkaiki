# API Documentation

This document provides detailed information about the API endpoints, input and output formats, and how to use these APIs to interact with the article management system. The APIs allow clients to retrieve articles, manage the cache, and refresh the article index.

## Overview

- **Base URL**: `http://127.0.0.1:8080/api/v1`
- **Available Endpoints**:
  - `/article/{id}`
  - `/article/all`
  - `/article/cache`
  - `/article/{id}/cache`
  - `/index`

All responses are in **JSON format** unless stated otherwise.

---

## Endpoints

### 1. Get Article by ID

Retrieve a specific article by its ID.

- **Endpoint**

  ```
  GET /api/v1/article/{id}
  ```

- **Path Parameters**

  - `{id}`: The integer ID of the article to retrieve.

- **Responses**

  - **200 OK**: The article was found and returned in the response body.
    - **Body**: [Article](#article-object) object.
  - **404 Not Found**: The article with the specified ID does not exist.
    - **Body**: Error message.

- **Example Request**

  ```
  GET /api/v1/article/1
  ```

- **Example Response**

  ```json
  {
    "id": 1,
    "title": "Sample Article",
    "description": "A sample article for demonstration purposes.",
    "content": "<p>This is the content of the sample article.</p>",
    "date": 20231015,
    "tags": ["sample", "demo"]
  }
  ```

---

### 2. Get All Articles Without Content

Retrieve a list of all articles without their full content (summaries).

- **Endpoint**

  ```
  GET /api/v1/article/all
  ```

- **Responses**

  - **200 OK**: A list of article summaries is returned.
    - **Body**: Array of [ArticleSummary](#article-summary-object) objects.

- **Example Request**

  ```
  GET /api/v1/article/all
  ```

- **Example Response**

  ```json
  [
    {
      "id": 1,
      "title": "Sample Article",
      "description": "A sample article for demonstration purposes.",
      "date": 20231015,
      "tags": ["sample", "demo"]
    },
    {
      "id": 2,
      "title": "Another Article",
      "description": "Description of another article.",
      "date": 20231016,
      "tags": ["other"]
    }
  ]
  ```

---

### 3. Clear Article Cache

Clear the server's article cache. This forces the server to reload articles from the filesystem on the next request.

- **Endpoint**

  ```
  DELETE /api/v1/article/cache
  ```

- **Responses**

  - **200 OK**: The cache was successfully cleared.
    - **Body**: Confirmation message (plain text).

- **Example Request**

  ```
  DELETE /api/v1/article/cache
  ```

- **Example Response**

  ```
  Cache cleared
  ```

---

### 4. Refresh Article Cache by ID

Refresh the cache for a specific article by its ID. This reloads the article from the filesystem and updates the cache.

- **Endpoint**

  ```
  PUT /api/v1/article/{id}/cache
  ```

- **Path Parameters**

  - `{id}`: The integer ID of the article to refresh.

- **Responses**

  - **200 OK**: The article cache was successfully refreshed.
    - **Body**: Confirmation message (plain text).
  - **500 Internal Server Error**: Failed to refresh the article cache.
    - **Body**: Error message.

- **Example Request**

  ```
  PUT /api/v1/article/1/cache
  ```

- **Example Response**

  ```
  Article cache refreshed
  ```

---

### 5. Refresh Article Index

Refresh the server's article index. This reloads the index of articles from the filesystem.

- **Endpoint**

  ```
  PUT /api/v1/index
  ```

- **Responses**

  - **200 OK**: The article index was successfully refreshed.
    - **Body**: Confirmation message (plain text).
  - **500 Internal Server Error**: Failed to refresh the article index.
    - **Body**: Error message.

- **Example Request**

  ```
  PUT /api/v1/index
  ```

- **Example Response**

  ```
  Index refreshed
  ```

---

## Data Models

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

### Retrieving an Article by ID

To get a specific article, send a `GET` request to `/api/v1/article/{id}`.

**Example with `curl`:**

```sh
curl -X GET http://127.0.0.1:8080/api/v1/article/1
```

### Retrieving All Article Summaries

To get summaries of all articles, send a `GET` request to `/api/v1/article/all`.

**Example with `curl`:**

```sh
curl -X GET http://127.0.0.1:8080/api/v1/article/all
```

### Clearing the Article Cache

To clear the server's article cache, send a `DELETE` request to `/api/v1/article/cache`.

**Example with `curl`:**

```sh
curl -X DELETE http://127.0.0.1:8080/api/v1/article/cache
```

### Refreshing the Cache for a Specific Article

To refresh the cache for a specific article, send a `PUT` request to `/api/v1/article/{id}/cache`.

**Example with `curl`:**

```sh
curl -X PUT http://127.0.0.1:8080/api/v1/article/1/cache
```

### Refreshing the Article Index

To refresh the article index, send a `PUT` request to `/api/v1/index`.

**Example with `curl`:**

```sh
curl -X PUT http://127.0.0.1:8080/api/v1/index
```

---

## Input and Output Formats

### Input Formats

- **GET Requests**: No request body required.
- **DELETE Requests**: No request body required.
- **PUT Requests**: No request body required.

### Output Formats

- **Content-Type**: `application/json` for JSON responses.
- **Content-Type**: `text/plain` for plain text confirmation messages.

### Error Responses

In case of errors, the API will return an appropriate HTTP status code along with an error message.

- **404 Not Found**: The requested resource does not exist.
- **500 Internal Server Error**: An unexpected error occurred on the server.

**Example Error Response:**

```json
{
  "error": "Article not found"
}
```

---

## Examples

### Example 1: Retrieve Article with ID 1

**Request:**

```sh
curl -X GET http://127.0.0.1:8080/api/v1/article/1
```

**Response:**

```json
{
  "id": 1,
  "title": "Sample Article",
  "description": "A sample article for demonstration purposes.",
  "content": "<p>This is the content of the sample article.</p>",
  "date": 20231015,
  "tags": ["sample", "demo"]
}
```

### Example 2: Retrieve All Article Summaries

**Request:**

```sh
curl -X GET http://127.0.0.1:8080/api/v1/article/all
```

**Response:**

```json
[
  {
    "id": 1,
    "title": "Sample Article",
    "description": "A sample article for demonstration purposes.",
    "date": 20231015,
    "tags": ["sample", "demo"]
  },
  {
    "id": 2,
    "title": "Another Article",
    "description": "Description of another article.",
    "date": 20231016,
    "tags": ["other"]
  }
]
```

### Example 3: Clear Article Cache

**Request:**

```sh
curl -X DELETE http://127.0.0.1:8080/api/v1/article/cache
```

**Response:**

```
Cache cleared
```

### Example 4: Refresh Cache for Article with ID 1

**Request:**

```sh
curl -X PUT http://127.0.0.1:8080/api/v1/article/1/cache
```

**Response:**

```
Article cache refreshed
```

### Example 5: Refresh Article Index

**Request:**

```sh
curl -X PUT http://127.0.0.1:8080/api/v1/index
```

**Response:**

```
Index refreshed
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