# Configuration Guide for Henkaiki

This document serves as a comprehensive guide to configuring the Article Management System. It outlines the configuration file structure, available parameters, their default values, and how they influence the behavior of the system.

---

## Table of Contents

1. [Introduction](#introduction)
2. [Configuration File Location](#configuration-file-location)
3. [Configuration Parameters](#configuration-parameters)
   - [Main Configuration](#main-configuration)
     - [`articles_dir`](#articles_dir)
     - [`max_cached_articles`](#max_cached_articles)
     - [`sample_article`](#sample_article)
     - [`address`](#address)
     - [`port`](#port)
     - [`record_cache_stats`](#record_cache_stats)
   - [Markdown Extensions](#markdown-extensions)
     - [`strikethrough`](#strikethrough)
     - [`table`](#table)
     - [`autolink`](#autolink)
     - [`tasklist`](#tasklist)
     - [`footnotes`](#footnotes)
     - [`description_lists`](#description_lists)
     - [`multiline_block_quotes`](#multiline_block_quotes)
     - [`math_dollars`](#math_dollars)
     - [`math_code`](#math_code)
     - [`wikilinks_title_after_pipe`](#wikilinks_title_after_pipe)
     - [`wikilinks_title_before_pipe`](#wikilinks_title_before_pipe)
     - [`spoiler`](#spoiler)
     - [`greentext`](#greentext)
     - [`front_matter_delimiter`](#front_matter_delimiter)
     - [`header_ids`](#header_ids)
     - [`shortcodes`](#shortcodes)
     - [`subscript`](#subscript)
     - [`superscript`](#superscript)
     - [`tagfilter`](#tagfilter)
     - [`underline`](#underline)
4. [Default Values](#default-values)
5. [How Configuration is Loaded](#how-configuration-is-loaded)
6. [Examples](#examples)
   - [Sample `config.toml` File](#sample-configtoml-file)
7. [Conclusion](#conclusion)
8. [Additional Information](#additional-information)

---

## Introduction

The Article Management System uses a configuration file to control various aspects of its behavior, including where articles are stored, how many articles are cached in memory, and which Markdown extensions are enabled.

Understanding and correctly setting these configuration parameters is crucial for tailoring the system to your specific needs.

---

## Configuration File Location

The configuration file is named `config.toml` and should be located in the current working directory of the application. By default, the application searches for `config.toml` in the directory from which it is run.

**Note**: If the configuration file is not found, the application will panic and exit. Ensure that `config.toml` is present and correctly formatted before running the application.

---

## Configuration Parameters

The configuration file uses the [TOML](https://toml.io/en/) format and consists of two main sections:

- `[mainconfig]`: General settings for the application.
- `[extensions]`: Settings for Markdown parsing extensions.

### Main Configuration

The `[mainconfig]` section contains general settings that affect the application's operation.

#### `articles_dir`

- **Description**: Specifies the directory where article files are stored.
- **Type**: String (path)
- **Default**: `./articles` (relative to the current directory)
- **Example**:

  ```toml
  articles_dir = "my_articles"
  ```

#### `max_cached_articles`

- **Description**: Sets the maximum number of articles to keep in the in-memory cache.
- **Type**: Integer
- **Default**: `100`
- **Example**:

  ```toml
  max_cached_articles = 200
  ```

#### `sample_article`

- **Description**: Enables or disables the inclusion of the sample article with ID `0`.
- **Type**: Boolean
- **Default**: `false`
- **Example**:

  ```toml
  sample_article = true
  ```

#### `address`

- **Description**: Specifies the IP address to bind the application to.
- **Type**: String
- **Default**: `127.0.0.1`
- **Example**:

  ```toml
  address = "0.0.0.0"
  ```

#### `port`

- **Description**: Specifies the port number to bind the application to.
- **Type**: Integer
- **Default**: `8080`
- **Example**:

  ```toml
  port = 9090
  ```

#### `record_cache_stats`

- **Description**: Enables or disables the recording of cache statistics for monitoring and debugging purposes.
- **Type**: Boolean
- **Default**: `false`
- **Example**:

  ```toml
  record_cache_stats = true
  ```

### Markdown Extensions

The `[extensions]` section configures which Markdown extensions are enabled during the parsing and rendering of articles.

Each extension is a boolean value (`true` or `false`). The default for all extensions is `true`, meaning they are enabled by default.

#### `strikethrough`

- **Description**: Enables the parsing of strikethrough syntax (`~~text~~`).
- **Type**: Boolean
- **Default**: `true`
- **Example**:

  ```toml
  strikethrough = true
  ```

#### `table`

- **Description**: Enables the parsing of Markdown tables.
- **Type**: Boolean
- **Default**: `true`
- **Example**:

  ```toml
  table = true
  ```

#### `autolink`

- **Description**: Automatically creates links from URLs.
- **Type**: Boolean
- **Default**: `true`
- **Example**:

  ```toml
  autolink = true
  ```

#### `tasklist`

- **Description**: Enables task list items (`- [ ]` and `- [x]`).
- **Type**: Boolean
- **Default**: `true`
- **Example**:

  ```toml
  tasklist = true
  ```

#### `footnotes`

- **Description**: Enables footnote syntax.
- **Type**: Boolean
- **Default**: `true`
- **Example**:

  ```toml
  footnotes = true
  ```

#### `description_lists`

- **Description**: Enables description list syntax.
- **Type**: Boolean
- **Default**: `true`
- **Example**:

  ```toml
  description_lists = true
  ```

#### `multiline_block_quotes`

- **Description**: Enables multiline block quotes.
- **Type**: Boolean
- **Default**: `true`
- **Example**:

  ```toml
  multiline_block_quotes = true
  ```

#### `math_dollars`

- **Description**: Enables parsing of inline math using `$...$` and display math using `$$...$$`.
- **Type**: Boolean
- **Default**: `true`
- **Example**:

  ```toml
  math_dollars = true
  ```

#### `math_code`

- **Description**: Enables parsing of math in code blocks.
- **Type**: Boolean
- **Default**: `true`
- **Example**:

  ```toml
  math_code = true
  ```

#### `wikilinks_title_after_pipe`

- **Description**: Allows the title to come after the pipe in wiki links `[[link|title]]`.
- **Type**: Boolean
- **Default**: `true`
- **Example**:

  ```toml
  wikilinks_title_after_pipe = true
  ```

#### `wikilinks_title_before_pipe`

- **Description**: Allows the title to come before the pipe in wiki links `[[title|link]]`.
- **Type**: Boolean
- **Default**: `true`
- **Example**:

  ```toml
  wikilinks_title_before_pipe = true
  ```

#### `spoiler`

- **Description**: Enables the parsing of spoiler syntax (`||spoiler text||`).
- **Type**: Boolean
- **Default**: `true`
- **Example**:

  ```toml
  spoiler = true
  ```

#### `greentext`

- **Description**: Enables greentext syntax (`>greentext`).
- **Type**: Boolean
- **Default**: `true`
- **Example**:

  ```toml
  greentext = true
  ```

#### `front_matter_delimiter`

- **Description**: Specifies a delimiter for front matter in articles.
- **Type**: String
- **Default**: None
- **Example**:

  ```toml
  front_matter_delimiter = "---"
  ```

#### `header_ids`

- **Description**: Allows specifying custom header IDs.
- **Type**: String
- **Default**: None
- **Example**:

  ```toml
  header_ids = "custom-id"
  ```

#### `shortcodes`

- **Description**: Enables the use of shortcodes in articles.
- **Type**: Boolean
- **Default**: `true`
- **Example**:

  ```toml
  shortcodes = true
  ```

#### `subscript`

- **Description**: Enables subscript syntax.
- **Type**: Boolean
- **Default**: `true`
- **Example**:

  ```toml
  subscript = true
  ```

#### `superscript`

- **Description**: Enables superscript syntax.
- **Type**: Boolean
- **Default**: `true`
- **Example**:

  ```toml
  superscript = true
  ```

#### `tagfilter`

- **Description**: Enables filtering of HTML tags.
- **Type**: Boolean
- **Default**: `true`
- **Example**:

  ```toml
  tagfilter = true
  ```

#### `underline`

- **Description**: Enables underline syntax.
- **Type**: Boolean
- **Default**: `true`
- **Example**:

  ```toml
  underline = true
  ```

---

## Default Values

If a parameter is not specified in the `config.toml` file, the application uses the default value.

- **Main Configuration**:

  - `articles_dir`: Current directory concatenated with `articles` (i.e., `articles`)
  - `max_cached_articles`: `100`
  - `sample_article`: `false`
  - `address`: `127.0.0.1`
  - `port`: `8080`

- **Markdown Extensions**:

  All Markdown extensions are enabled by default (`true`).

---

## How Configuration is Loaded

- The configuration is loaded once at the application's startup using the singleton pattern via `lazy_static!`.

- The application tries to read `config.toml` from the current working directory.

- If the configuration file is missing or contains errors, the application will terminate with a panic message.

- The configuration is parsed into a `Config` struct, which is then shared across the application using `Arc` (Atomic Reference Counting) to allow safe concurrent access.

- The loaded configuration settings influence:

  - How articles are read from the filesystem.
  - How many articles are cached in memory using an LRU (Least Recently Used) cache.
  - Which Markdown parsing features are available when rendering article content.

---

## Examples

### Sample `config.toml` File

Below is an example of a `config.toml` file that demonstrates how to configure the application.

```toml
[mainconfig]
articles_dir = "articles"
max_cached_articles = 50
sample_article = true
address = "127.0.0.1"
port = 8080
record_cache_stats = true

[extensions]
strikethrough = true
table = true
autolink = true
tasklist = true
footnotes = true
description_lists = true
multiline_block_quotes = true
math_dollars = true
math_code = true
wikilinks_title_after_pipe = true
wikilinks_title_before_pipe = true
spoiler = true
greentext = true
front_matter_delimiter = "---"
header_ids = "custom-id"
shortcodes = true
subscript = true
superscript = true
tagfilter = true
underline = true
```

**Explanation**:

- **Main Configuration**:

  - **articles_dir**: The articles are stored in the `./articles` directory relative to where the application is run.
  - **max_cached_articles**: The application will cache up to `50` articles in memory.
  - **sample_article**: The sample article with ID `0` is included and can be accessed via the API.
  - **address**: The application will bind to the IP address `0.0.0.0`.
  - **port**: The application will bind to port `8080`.

- **Markdown Extensions**:

  All Markdown extensions are enabled. If you wish to disable an extension, set its value to `false`.

---

## Conclusion

By customizing the `config.toml` file, you can tailor the Article Management System to meet your specific requirements. Adjusting the main configuration allows you to control where articles are stored and cached, while enabling or disabling Markdown extensions lets you define how articles are parsed and rendered.

Ensuring the configuration file is correctly set up is essential for the smooth operation of the application.

---

## Additional Information

### Important Notes

- **Path Validity**: Ensure that the path specified in `articles_dir` exists and is accessible by the application. If the directory does not exist, the application may fail to load articles.

- **Caching Behavior**:

  - The `max_cached_articles` parameter controls the size of the in-memory cache. A larger cache may improve performance by reducing filesystem reads at the expense of higher memory usage.
  - The application uses an LRU cache, meaning it evicts the least recently used articles when the cache limit is reached.

- **Sample Article**:

  - When `sample_article` is set to `true`, an article with ID `0` is available via the API. This is useful for testing and demonstration purposes.
  - The sample article content is hardcoded within the application.

### Changing Configuration at Runtime

- The configuration is loaded once at startup and is not intended to be changed at runtime.
- To apply new configuration settings, modify the `config.toml` file and restart the application.

### Logging Configuration

- The application uses `env_logger` for logging.
- Logging behavior can be controlled via environment variables (e.g., `RUST_LOG`).

  **Example**:

  ```sh
  RUST_LOG=info ./henkaiki
  ```

---

**Note**: Ensure that your `config.toml` file is properly formatted according to TOML specifications to prevent parsing errors.

If you encounter issues or have questions about specific configuration options, refer to the application's source code or contact the maintainers for assistance.