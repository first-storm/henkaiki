# `metainfo.toml` Documentation

This document describes the structure and purpose of the `metainfo.toml` file used in the Article Management System. This file provides metadata for each article, enabling efficient indexing, retrieval, and content management within the application.

---

## Purpose of `metainfo.toml`

The `metainfo.toml` file serves as a metadata descriptor for each article, facilitating the following:

1. Identification of the article with a unique ID.
2. Description of the article using fields like `title`, `description`, `tags`, and `keywords`.
3. Specification of the location of the article's Markdown content file.
4. Provision of publication date and categorization details.

The application processes these files to index articles, load content dynamically, and deliver metadata and summaries via the API.

---

## File Structure

The `metainfo.toml` file adheres to the [TOML](https://toml.io/en/) standard and includes the following key sections:

### `[article]` Section

The `[article]` section contains all metadata for the article. All fields are mandatory, and their types must conform to the specifications below.

---

### Fields in `[article]`

| Field            | Type               | Description                                                                                          | Example Value                            |
|-------------------|--------------------|------------------------------------------------------------------------------------------------------|------------------------------------------|
| `id`             | Integer            | A unique identifier for the article. This must match the name of the directory containing the article. | `1`                                      |
| `title`          | String             | The title of the article.                                                                           | `"Sample Article"`                       |
| `description`    | String             | A brief description of the article.                                                                 | `"This is a sample description."`        |
| `markdown_path`  | String (file path) | The relative path to the Markdown file containing the article's content, within the article directory. | `"content.md"`                           |
| `date`           | Integer (YYYYMMDD) | The publication date of the article, formatted as an integer.                                        | `20231201`                               |
| `tags`           | Array of Strings   | A list of tags associated with the article.                                                         | `["sample", "example"]`                  |
| `keywords`       | Array of Strings   | A list of keywords related to the article, used for additional categorization or search optimization. | `["example", "documentation"]`           |

---

## Example `metainfo.toml`

Below is an example of a valid `metainfo.toml` file:

```toml
[article]
id = 1
title = "Sample Article"
description = "This article demonstrates the structure of metainfo.toml."
markdown_path = "content.md"
date = 20231201
tags = ["sample", "example", "documentation"]
keywords = ["tutorial", "metadata", "example"]
```

### Explanation of Example

- **`id`**: Identifies the article as `1`. This must match the directory name containing this `metainfo.toml` file.
- **`title`**: Specifies the title as `"Sample Article"`.
- **`description`**: Provides a brief description of the article's purpose.
- **`markdown_path`**: Indicates that the content is stored in a Markdown file named `content.md` within the directory.
- **`date`**: Specifies the publication date as `2023-12-01` (YYYYMMDD format).
- **`tags`**: Associates the article with tags `"sample"`, `"example"`, and `"documentation"`.
- **`keywords`**: Adds additional keywords `"tutorial"`, `"metadata"`, and `"example"` for enhanced searchability.

---

## Application Usage of `metainfo.toml`

The application utilizes the `metainfo.toml` file for the following purposes:

### 1. **Indexing Articles**
   - During initialization, the application scans the `articles` directory for `metainfo.toml` files to build an index.
   - The `id` field ensures uniqueness for each article.

### 2. **Retrieving Article Metadata**
   - Fields like `title`, `description`, `date`, `tags`, and `keywords` are used to generate summaries or lists of articles (e.g., for API endpoints).

### 3. **Loading Article Content**
   - The `markdown_path` specifies the location of the article's Markdown file, which is then read and converted to HTML.

### 4. **Validation**
   - The application validates each `metainfo.toml` file. Articles with invalid or missing metadata are excluded from the index.

---

## Validation Rules

The application enforces strict validation rules for the `metainfo.toml` file:

### 1. **Required Fields**
   - All fields (`id`, `title`, `description`, `markdown_path`, `date`, `tags`, and `keywords`) are mandatory.
   - Missing fields result in warnings, and the article is excluded from the index.

### 2. **Type Validation**
   - Each field must adhere to its specified type:
     - `id`: Integer
     - `title`, `description`, `markdown_path`: Strings
     - `date`: Integer in `YYYYMMDD` format
     - `tags`, `keywords`: Arrays of strings
   - Type mismatches result in warnings, and the article is excluded.

### 3. **ID and Directory Match**
   - The `id` field must match the directory name. Mismatched IDs result in warnings and exclusion from the index.

### 4. **Markdown File Check**
   - The file specified in `markdown_path` must exist. Missing files result in warnings and exclusion from the index.

---

## Common Errors and Resolutions

### Error 1: Missing `metainfo.toml`
- **Log Message**: `Metainfo file missing for article ID {id} in path {path}`
- **Cause**: The directory lacks a `metainfo.toml` file.
- **Solution**: Add a valid `metainfo.toml` file to the article directory.

---

### Error 2: Missing or Invalid Fields
- **Log Message**: `Missing or invalid '{field}' in metainfo.toml`
- **Cause**: Required fields are missing or have incorrect types.
- **Solution**: Ensure all required fields are present and correctly formatted.

---

### Error 3: ID Mismatch
- **Log Message**: `ID mismatch in metainfo for article ID {id}: metainfo ID is {metainfo_id}`
- **Cause**: The `id` field does not match the directory name.
- **Solution**: Update the `id` field in the `metainfo.toml` file to match the directory name.

---

### Error 4: Missing Markdown File
- **Log Message**: `Markdown file missing for article ID {id}: {path}`
- **Cause**: The Markdown file specified in `markdown_path` is missing.
- **Solution**: Ensure the file exists in the specified location.

---

## Best Practices

1. **Unique Identifiers**
   - Use unique directory names and `id` values to prevent indexing conflicts.

2. **Consistent Date Format**
   - Always use the `YYYYMMDD` format for the `date` field.

3. **Comprehensive Keywords**
   - Include relevant and descriptive keywords to enhance searchability.

4. **Validation Tools**
   - Use automated validation scripts or tools to verify the integrity of `metainfo.toml` files before deployment.

5. **Structured Directories**
   - Maintain a clean and organized directory structure with one `metainfo.toml` and one content file per article.