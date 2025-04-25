# Gospin - GO Boilerplate Initiator

Gospin is a simple command-line tool designed to help developers kickstart a new Go project instantly. With Gospin, you can quickly generate boilerplate code tailored to your project's needs, saving you time and effort. Whether you're building a REST API, a project with database integration, or a WebSocket-based application, Gospin has got you covered.

---

## Features

- Generate boilerplate Go projects instantly.
- Support for optional database integration.
- Easy setup for WebSocket-enabled applications.
- Ability to specify a custom HTTP port.
- GitHub username integration for project initialization.

---

## Installation

To install Gospin, ensure you have [Cargo](https://doc.rust-lang.org/cargo/) installed, and run the following command:

```bash
cargo install gospin
```

---

## Usage

To create a new Go project using Gospin, use the following command:

```bash
gospin [OPTIONS] [PROJECT]
```

### Arguments

- `[PROJECT]`: The name of the Go project to be generated. This is optional; if not provided, Gospin will use a default name.

### Options

- `-u, --username <GHU>`: Specify your GitHub username. This will include your GitHub username in the project's metadata.
- `-p, --port <PORT>`: Set the HTTP port for the project. Defaults to `8080` if not specified.
- `-d, --database`: Include boilerplate code for database integration.
- `-w, --websockets`: Include boilerplate code for WebSocket functionality.
- `-h, --help`: Print the help message and exit.
- `-V, --version`: Print the version of Gospin and exit.

---

### Examples

#### Basic Project Initialization

Generate a basic Go project:
```bash
gospin my-project
```

#### Custom Port

Generate a project with a custom HTTP port:
```bash
gospin -p 3000 my-project
```

#### Include Database Integration

Generate a project with boilerplate code for database integration:
```bash
gospin -d my-project
```

#### Include WebSocket Support

Generate a project with WebSocket support:
```bash
gospin -w my-project
```

#### GitHub Username

Include your GitHub username in the project metadata:
```bash
gospin -u myusername my-project
```

#### Combined Options

Generate a project with database integration, WebSocket support, and a custom port:
```bash
gospin -d -w -p 5000 my-project
```

---

## Contributing

Contributions are welcome! Feel free to submit issues or pull requests to improve Gospin.

---

## License

Gospin is open-source and available under the [BSD 3-Clause License](LICENSE).
