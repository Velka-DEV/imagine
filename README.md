# Imagine: A Media CDN in Rust

[![Build Status](https://img.shields.io/badge/build-passing-green.svg)](LINK_TO_YOUR_CI)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Imagine is an ambitious media CDN designed to offer robust storage capabilities for images, videos, and audios. Not just a mere storage solution, it also provides format encoding (like AVIF, WebP, etc.) to ensure your media is served in the most optimized format to your audience.

> ⚠️ **Disclaimer**: Imagine is currently in the early stages of development and is **not** recommended for production use.

## Features

- **File Storage**: Store your media safely and retrieve them with ease.
- ⚠️ WIP ⚠️ **Media Encoding**: Automatic format encoding to ensure the best and most optimized format for your media.
- ⚠️ TBA ⚠️ **Authentication & authorization**: User based authentication with API keys.
- ⚠️ TBA ⚠️ **File encryption**: Store your files encrypted for enhanced security and privacy.
- ⚠️ TBA ⚠️ **Dashboard**: Manage everything from a web dashboard.
- **Built with Rust**: Leveraging the power and safety of Rust for efficient and concurrent media handling.

## Endpoints

- Files: 
  - [GET] List: `/file` (May change in neer future to `/files`)
  - [POST] Upload: `/file`
  - [GET] Serve file: `/file/{id}`

## Tech Stack

- **Web Framework**: [Salvo](https://github.com/salvo-rs/salvo)
- **ORM**: [SeaORM](https://github.com/SeaQL/sea-orm)
- **Database Backend**: [sqlx-sqlite](https://github.com/launchbadge/sqlx)

## Getting Started

1. Copy `.env.example` to `.env`
2. Provide `HOST`, `PORT` and the `DATABASE_URL` env variables
3. Run and enjoy

## Contribution

💡 I'm thrilled you're interested in contributing! This being my first Rust project, there might be areas of improvement and I would appreciate any feedback or help.

1. Fork the repository.
2. Clone your fork (`git clone https://github.com/YOUR_USERNAME/Imagine.git`).
3. Create a new feature branch (`git checkout -b feature/my-awesome-feature`).
4. Commit your changes.
5. Push to your branch (`git push origin feat/my-awesome-feature`).
6. Open a pull request.

All PRs are welcome! Whether you're looking to fix bugs, add features, or improve the documentation, I'd love to see your contributions.

## A Little Note

This is my maiden journey into Rust. As such, the code may not be perfect and might not adhere to all best practices. I'm eager to learn and improve, so if you notice anything, please feel free to point it out or contribute.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.