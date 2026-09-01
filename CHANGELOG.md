# Changelog

All notable changes to this project will be documented in this file.  
This format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)  
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] 
- Fixed an issue where the command group and name was not accessible from the `AddressedCommand`

## [0.4.0] - 2026-08-27
- Debug Feature
    crate feature adds command debug data to the `AddressedCommand` struct
    to allow consuming applications to inspect the command data that was parsed.
    This byte array does **not** include the commands header data.

## [0.3.0] - 2026-06-27
- Fixed typo in the types "Apature" -> "Aperture"
- Fixed bug in with Aperture FStop: missing sqrt(2^AV) calculation.

## [0.2.4] - 2025-11-12
- Clippy cleanup

## [0.2.3] - 2025-11-12
- Fixed struct and module privacy issues

## [0.2.2] - 2025-10-20

### Fixed
- Fixed issues with error typing for rppal adapter

## [0.2.1] - 2025-10-18

### Enhancements
- Errors: Invalid Command Data Error now contains the command data.

### Fixed
- End of packet padding bug; The last command in the packets I sampled off the shader board was missing some padding.
- End of packet command data bug.

### Added
- DisplayLUT Command
- NDFilterStop Command; This command feature toggled with the "ignore-nd-filter" feature.
Because there is an inconsitency in the way that the protocol was implemented.
I plan on reaching out to Black Magic Design about this to figure out the best course of action.

## [0.2.0] - 2025-10-08

### Added
- Eldritch Shield crate.
- Some tagging to the crates to help them be found in crates.io

### Removed
- write_read method on Shield crate.

## [0.1.0] - 2025-09-03

### Added
- Initial release of `eldritchwire` library crate.
- Basic SDI packet parsing and camera control command support.
