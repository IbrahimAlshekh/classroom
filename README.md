# Classroom Schedule Generator

A simple Rust CLI tool for generating school schedules. This project helps schools organize their teachers, classes, 
and schedules in a structured way.

## Purpose

This application is designed to help schools manage and generate schedules. It provides a command-line interface 
to create, manage, and organize:

- Teachers and their subjects
- Classes and descriptions
- Schedules linking teachers to classes

The goal is to (learn rust ;) make school schedule management easier and more automated.

## Current Status

The project is actively being developed. I've got the core data models working nicely – teachers, classes, and schedules 
are all in place with proper serialization support. Right now I'm working on the CLI interface and data import/export
features. The foundation is solid, and I'm building out the practical features that'll make this tool actually 
useful to be used.

## Features

### Currently Implemented
- Core data models (Teacher, Class, Schedule, School)
- Type-safe ID management for all entities
- JSON serialization/deserialization support
- Comprehensive unit tests
- Idiomatic Rust code following best practices

### In Progress
- CLI interface with command-line arguments
- CSV/Excel data import functionality
- Schedule generation algorithm

### Planned
- Excel export for generated schedules
- Conflict detection (teacher double-booking, room conflicts)
- Time slot management
- Schedule optimization algorithms
- Web UI using Dioxus framework

## Data Format

The application works with three main entities:

**Teachers**: Each teacher has an ID, name, and subject they teach.

**Classes**: Classes have an ID, name, and description.

**Schedules**: Schedules link teachers to classes with time slots (coming soon).

I'm planning to support CSV and Excel formats for bulk importing data. The exact format will be documented once the
import feature is complete.

## Usage Examples

*Note: CLI interface is still being built. These examples show the planned usage.*

```bash
# Import teachers from CSV
classroom import --file data.csv
```

## Future: Web UI with Dioxus

We're planning to add a web-based UI using the Dioxus framework. This will make it easier to:
- Visualize schedules in a calendar view
- Drag-and-drop schedule editing
- Real-time conflict detection
- Export and share schedules

The CLI will be added just during the initial development phase to allow testing and experimentation,
but the Dioxus UI will provide a more visual way to work with schedules.

## TODO

- [ ] Implement CLI interface with command-line arguments
- [ ] Add schedule generation algorithm
- [ ] Support for importing data from CSV/JSON files
- [ ] Export schedules to Excel format (using rust_xlsxwriter)
- [ ] Add conflict detection (teacher double-booking, room conflicts)
- [ ] Implement time slot management
- [ ] Add validation for schedule constraints
- [ ] Create example usage documentation
- [ ] Add integration tests
- [ ] Support multiple schools in one run
- [ ] Add configuration file support
- [ ] Implement schedule optimization (minimize gaps, balance workload)
- [ ] Design and implement Dioxus web UI

## Building

```bash
cargo build
```

## Running Tests

```bash
cargo test
```

# License
MIT License © 2026 