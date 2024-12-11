# `metainfo.toml` Documentation

This document provides a detailed description of the structure and purpose of the `metainfo.toml` file used in the Article Management System. The `metainfo.toml` file contains metadata for each article, such as its ID, title, description, and other properties. This metadata is essential for indexing and managing articles in the application.

---

## Purpose of `metainfo.toml`

The `metainfo.toml` file serves as a descriptor for each article. It provides the following information:

1. Identifies the article using a unique ID.
2. Describes the article with a title, description, and tags.
3. Specifies where the article's content (Markdown file) is located.
4. Defines the publication date of the article.

The application reads and parses this file to build an index of articles, load article content, and serve article data via the API.

---

## File Structure

The `metainfo.toml` file follows the [TOML](https://toml.io/en/) format and contains the following key sections:

### `[article]` Section

The `[article]` section contains the metadata for the article. All fields in this section are required, and their types must match the specifications below.

### Fields in `[article]`

| Field            | Type             | Description                                                                                          | Example Value                            |
|-------------------|------------------|------------------------------------------------------------------------------------------------------|------------------------------------------|
| `id`             | Integer          | A unique identifier for the article. This must match the directory name containing the article.      | `1`                                      |
| `title`          | String           | The title of the article.                                                                           | `"Sample Article"`                       |
| `description`    | String           | A brief description of the article.                                                                 | `"This is a sample description."`        |
| `markdown_path`  | String (file path) | The relative path to the Markdown file containing the article's content, within the article directory. | `"content.md"`                           |
| `date`           | Integer (YYYYMMDD) | The publication date of the article, formatted as an integer (e.g., `YYYYMMDD`).                     | `20231201`                               |
| `tags`           | Array of Strings | A list of tags associated with the article.                                                         | `["sample", "example"]`                  |

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
```

### Explanation of Example

- **`id`**: Identifies the article as `1`. This must match the name of the directory containing this `metainfo.toml` file.
- **`title`**: The article's title is `"Sample Article"`.
- **`description`**: A short description of the article.
- **`markdown_path`**: Specifies that the article's content is stored in a file named `content.md` within the same directory.
- **`date`**: The article was published on `2023-12-01` (December 1, 2023).
- **`tags`**: The tags `"sample"`, `"example"`, and `"documentation"` are associated with the article.

---

## How `metainfo.toml` is Used

The application uses the `metainfo.toml` file in the following ways:

1. **Indexing Articles**:
   - During initialization, the application reads all `metainfo.toml` files in the configured `articles` directory to build an index of available articles.
   - The `id` field is used to ensure each article has a unique identifier.

2. **Loading Article Metadata**:
   - The `title`, `description`, `date`, and `tags` fields are used to create summaries of articles (e.g., for the `/api/v1/article/all` endpoint).

3. **Loading Article Content**:
   - The `markdown_path` field specifies the location of the article's Markdown file, which is then converted into HTML for serving via the API.

4. **Validation**:
   - The application checks for the presence of a valid `metainfo.toml` file in each article directory. If the file is missing or invalid, the article is excluded from the index.

---

## Validation Rules

The application enforces the following validation rules for `metainfo.toml`:

1. **Presence of Required Fields**:
   - All fields (`id`, `title`, `description`, `markdown_path`, `date`, `tags`) must be present in the `[article]` section.
   - Missing fields will result in a warning, and the article will not be indexed.

2. **Field Type Validation**:
   - Each field must match its expected type (e.g., `id` must be an integer, `title` must be a string, etc.).
   - Invalid types will result in a warning, and the article will not be indexed.

3. **Directory and ID Match**:
   - The `id` field in the `metainfo.toml` file must match the name of the directory containing the file. If there is a mismatch, the article will not be indexed.

4. **Markdown File Existence**:
   - The file specified by `markdown_path` must exist in the article directory. If the file is missing, the article will not be included in the index.

---

## Common Errors and Resolutions

### 1. Missing `metainfo.toml`
**Error**:
- The application logs a warning: `Metainfo file missing for article ID {id} in path {path}`.

**Resolution**:
- Ensure that each article directory contains a valid `metainfo.toml` file.

---

### 2. Invalid or Missing Fields
**Error**:
- The application logs a warning: `Missing or invalid '{field}' in metainfo.toml`.

**Resolution**:
- Check the `metainfo.toml` file to ensure all required fields are present and have the correct type.

---

### 3. ID Mismatch
**Error**:
- The application logs a warning: `ID mismatch in metainfo for article ID {id}: metainfo ID is {metainfo_id}`.

**Resolution**:
- Ensure that the `id` field in `metainfo.toml` matches the name of the directory containing the file.

---

### 4. Missing Markdown File
**Error**:
- The application logs an error: `Markdown file missing for article ID {id}: {path}`.

**Resolution**:
- Verify that the file specified in the `markdown_path` field exists in the article directory.

---

## Best Practices

1. **Unique IDs**:
   - Ensure that each article directory name and `id` field in `metainfo.toml` are unique across all articles.

2. **Consistent Date Format**:
   - Use the `YYYYMMDD` format for the `date` field to ensure consistency.

3. **Validate Metadata**:
   - Before deploying articles, validate the `metainfo.toml` files to ensure all required fields are present and correctly formatted.

4. **Organized Directory Structure**:
   - Keep article directories well-organized, with one `metainfo.toml` file and one Markdown file per directory.