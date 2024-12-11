# Henkaiki

This project is the backend for a blog system, providing API endpoints for managing articles, caching, and indexing. It is built to efficiently handle article metadata and content using a structured configuration and metadata files.

## Features

- **Article Management**: Retrieve, index, and manage articles using a `metainfo.toml` file for metadata.
- **API Endpoints**: RESTful API to interact with articles, including fetching, caching, and refreshing operations.
- **Caching**: In-memory caching to improve performance with configurable cache size.
- **Markdown Support**: Extensive Markdown parsing with customizable extensions.

### Configuration

- The application requires a `config.toml` file in the working directory. This file specifies the articles directory, cache settings, and Markdown extensions.
- Refer to the [Configuration Guide](configuration.md) for detailed setup instructions.

### API Usage

- **Base URL**: `http://127.0.0.1:8080/api/v1`
- **Available endpoints** include:
  - `GET /article/{id}`: Retrieve a specific article by ID.
  - `GET /article/all`: Retrieve summaries of all articles.
  - `DELETE /article/cache`: Clear the article cache.
  - `PUT /article/{id}/cache`: Refresh cache for a specific article.
  - `PUT /index`: Refresh the article index.

For more details, see the [API Documentation](api.md).

### Article Metadata

- Articles are managed using a `metainfo.toml` file, which includes fields like:
  - `id`
  - `title`
  - `description`
  - `markdown_path`
  - `date`
  - `tags`
- See the [Metainfo Documentation](article.md) for more information.

## Development

- Ensure your development environment is set up with Rust and a suitable IDE like Visual Studio Code.
- Use `cargo test` to run unit tests and ensure code quality.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request for any improvements or bug fixes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact

For questions or support, please contact **2835365572zty@gmail.com**.
