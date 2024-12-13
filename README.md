# Henkaiki

This project is the backend for a blog system, providing API endpoints for managing articles, caching, and indexing. It is built to efficiently handle article metadata and content using a structured configuration and metadata files.

## Features

- **Article Management**: Retrieve, index, and manage articles using a `metainfo.toml` file for metadata.
- **Keyword Support**: Articles now support keywords for enhanced metadata and indexing.
- **API Endpoints**: RESTful API to interact with articles, including fetching, caching, refreshing operations, and filtering by tags or keywords.
- **Caching**: In-memory caching with an LRU (Least Recently Used) strategy to improve performance. The cache size is configurable.
- **Markdown Support**: Comprehensive Markdown parsing with customizable extensions and caching for generated HTML content.
- **Sample Article**: Includes an optional sample article for testing and demonstration purposes.

### Configuration

- The application requires a `config.toml` file in the working directory. This file specifies the articles directory, cache settings, and Markdown extensions.
- New configuration options include enabling or disabling the sample article.
- Refer to the [Configuration Guide](docs/configuration.md) for detailed setup instructions.

### API Usage

- **Base URL**: `http://127.0.0.1:8080/api/v1`
- **Available endpoints** include:
  - `/health`: Check the service health status
  - `/api/v1/articles`: Get a list of all available articles without content
  - `/api/v1/articles/{id}`: Retrieve a specific article by its ID
  - `/api/v1/tags/{tag}/articles`: Get all articles associated with a specific tag
  - `/api/v1/admin/articles/{id}/refresh`: Force refresh the cache for a specific article
  - `/api/v1/admin/cache/clear`: Clear the entire article cache
  - `/api/v1/admin/index/refresh`: Rebuild the article index from the filesystem

For more details, see the [API Documentation](docs/api.md).

### Article Metadata

- Articles are managed using a `metainfo.toml` file, which includes fields like:
  - `id`: Unique identifier for the article.
  - `title`: Title of the article.
  - `description`: Brief summary of the article.
  - `markdown_path`: Path to the Markdown file for the article.
  - `date`: Date of publication in YYYYMMDD format.
  - `tags`: List of tags associated with the article.
  - `keywords`: List of keywords for enhanced search and indexing. (NEW)
- The `keywords` field provides an additional layer of metadata for more granular article searches.
- See the [Metainfo Documentation](docs/article.md) for more information.

## Development

- Ensure your development environment is set up with Rust and a suitable IDE like Visual Studio Code.
- Key dependencies:
  - `dashmap` for concurrent indexing.
  - `lru` for in-memory caching.
  - `lazy_static` for initializing global resources like the sample article.

### Performance Enhancements

- LRU caching minimizes filesystem access by caching frequently accessed articles.
- Efficient indexing structures (`DashMap`) enable fast lookups by ID, tags, and keywords.
- Markdown-to-HTML conversion is performed on demand, with support for configurable Markdown extensions.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request for any improvements or bug fixes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact

For questions or support, please contact **2835365572zty@gmail.com**.
