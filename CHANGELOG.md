# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.10.1] - 2018-12-25

### Added

- Added Sync & Send ability to Pin
- Added initial implementation of an ADC interface (#13) - @HarkonenBade
- Added virtual-feature "device-selected" to simplify feature gating

### Changed

- Added overflow guards to delay

## [v0.10.0] - 2018-12-23

### Added

- Reworked GPIOs and added fully erased pins
- Timer support
- Support for STM32F070
- Additional peripheral support for STM32F030
- Watchdog support

### Changed

- Removed superfluous use statements
- Re-added Send ability for U(S)ART Rx/Tx
- Made crate to compile without features
- Eliminated a lot of unused warnings

### Fixed

- Fixed some comments
- Changed some prelude aliases to reflect crate name

### Removed

- Examples requiring additional driver crates

## [v0.9.0] - 2018-12-17

### Added

- Toggleable implementation for GPIOs
- Initial support for STM32F030
- LICENSE file

### Changed

- Updated stm32f0 dependency to v0.5.0.
- Interrupt handler to new #[interrupt] attribute

[Unreleased]: https://github.com/stm32-rs/stm32f0xx-hal/compare/v0.10.1...HEAD
[v0.10.1]: https://github.com/stm32-rs/stm32f0xx-hal/compare/v0.10.0...v0.10.1
[v0.10.0]: https://github.com/stm32-rs/stm32f0xx-hal/compare/v0.9.0...v0.10.0
[v0.9.0]: https://github.com/stm32-rs/stm32f0xx-hal/compare/v0.8.0...v0.9.0
