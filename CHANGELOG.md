# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.1](https://github.com/kosolabs/axum-anyhow/compare/v0.4.0...v0.4.1) - 2025-10-24

### Other

- Add function to convert ApiError back to an anyhow::Error ([#43](https://github.com/kosolabs/axum-anyhow/pull/43))

## [0.4.0](https://github.com/kosolabs/axum-anyhow/compare/v0.3.2...v0.4.0) - 2025-10-24

### Other

- Fix unauthorized and forbidden function names ([#42](https://github.com/kosolabs/axum-anyhow/pull/42))
- Support errors that can be coerced to an anyhow ([#41](https://github.com/kosolabs/axum-anyhow/pull/41))
- Call user's callback when ApiErrors get built ([#39](https://github.com/kosolabs/axum-anyhow/pull/39))

## [0.3.2](https://github.com/kosolabs/axum-anyhow/compare/v0.3.1...v0.3.2) - 2025-10-22

### Other

- Include README.md in lib.rs so that examples are tested ([#38](https://github.com/kosolabs/axum-anyhow/pull/38))
- Fix release-plz configuration ([#36](https://github.com/kosolabs/axum-anyhow/pull/36))

## [0.3.1](https://github.com/kosolabs/axum-anyhow/compare/v0.3.0...v0.3.1) - 2025-10-21

### Other

- Move error helper functions into a separate module ([#34](https://github.com/kosolabs/axum-anyhow/pull/34))

## [0.3.0](https://github.com/kosolabs/axum-anyhow/compare/v0.2.1...v0.3.0) - 2025-10-21

### Other

- Replaces direct constructor with builder pattern for ApiError ([#28](https://github.com/kosolabs/axum-anyhow/pull/28))

## [0.2.1](https://github.com/kosolabs/axum-anyhow/compare/v0.2.0...v0.2.1) - 2025-10-20

### Other

- Remove dead comment ([#26](https://github.com/kosolabs/axum-anyhow/pull/26))

## [0.2.0](https://github.com/kosolabs/axum-anyhow/compare/v0.1.1...v0.2.0) - 2025-10-20

### Other

- Convert ok_or_ to context_ to mirror Anyhow ([#18](https://github.com/kosolabs/axum-anyhow/pull/18))

## [0.1.1](https://github.com/kosolabs/axum-anyhow/compare/v0.1.0...v0.1.1) - 2025-10-20

### Other

- Configure Renovate ([#2](https://github.com/kosolabs/axum-anyhow/pull/2))
- Configure Release-plz ([#7](https://github.com/kosolabs/axum-anyhow/pull/7))
- Configure Release-plz ([#6](https://github.com/kosolabs/axum-anyhow/pull/6))
- Configure Release-plz ([#5](https://github.com/kosolabs/axum-anyhow/pull/5))
