# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.7.3](https://github.com/kosolabs/axum-anyhow/compare/v0.7.2...v0.7.3) - 2025-11-02

### Other

- *(deps)* update dependency rust to v1.91.0 ([#67](https://github.com/kosolabs/axum-anyhow/pull/67))
- *(deps)* lock file maintenance ([#68](https://github.com/kosolabs/axum-anyhow/pull/68))
- Update renovate.json ([#65](https://github.com/kosolabs/axum-anyhow/pull/65))

## [0.7.2](https://github.com/kosolabs/axum-anyhow/compare/v0.7.1...v0.7.2) - 2025-11-01

### Other

- *(config)* migrate renovate config ([#64](https://github.com/kosolabs/axum-anyhow/pull/64))
- Use a mutable builder to make code nicer ([#62](https://github.com/kosolabs/axum-anyhow/pull/62))

## [0.7.1](https://github.com/kosolabs/axum-anyhow/compare/v0.7.0...v0.7.1) - 2025-11-01

### Other

- Update README version with comment to use latest ([#60](https://github.com/kosolabs/axum-anyhow/pull/60))

## [0.7.0](https://github.com/kosolabs/axum-anyhow/compare/v0.6.2...v0.7.0) - 2025-11-01

### Other

- Use environment variable instead of feature flag to expose error details ([#58](https://github.com/kosolabs/axum-anyhow/pull/58))

## [0.6.2](https://github.com/kosolabs/axum-anyhow/compare/v0.6.1...v0.6.2) - 2025-10-29

### Other

- Add feature to expose error details directly in ApiError ([#56](https://github.com/kosolabs/axum-anyhow/pull/56))

## [0.6.1](https://github.com/kosolabs/axum-anyhow/compare/v0.6.0...v0.6.1) - 2025-10-29

### Other

- Update README with new helper functions available in 0.6 ([#55](https://github.com/kosolabs/axum-anyhow/pull/55))
- Configure Renovate for Rust and to run once a week ([#53](https://github.com/kosolabs/axum-anyhow/pull/53))

## [0.6.0](https://github.com/kosolabs/axum-anyhow/compare/v0.5.0...v0.6.0) - 2025-10-28

### Other

- Bump version in README ([#52](https://github.com/kosolabs/axum-anyhow/pull/52))
- Seal the traits so that adding methods isn't considered API breaking ([#50](https://github.com/kosolabs/axum-anyhow/pull/50))

## [0.5.0](https://github.com/kosolabs/axum-anyhow/compare/v0.4.2...v0.5.0) - 2025-10-28

### Other

- Add additional helper functions and move unit tests to doc tests ([#49](https://github.com/kosolabs/axum-anyhow/pull/49))
- Update README dependencies list in Installation instructions ([#47](https://github.com/kosolabs/axum-anyhow/pull/47))

## [0.4.2](https://github.com/kosolabs/axum-anyhow/compare/v0.4.1...v0.4.2) - 2025-10-24

### Other

- Add a warning message about the API still being in flux ([#45](https://github.com/kosolabs/axum-anyhow/pull/45))

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
