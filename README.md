# Regname: A Mass Renamer TUI in Rust 🎨

![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white) ![Terminal](https://img.shields.io/badge/Terminal-4EAA25?style=flat&logo=terminal&logoColor=white) ![Tool](https://img.shields.io/badge/Tool-FF5722?style=flat&logo=tool&logoColor=white) ![TUI](https://img.shields.io/badge/TUI-3F51B5?style=flat&logo=terminal&logoColor=white)

Welcome to **Regname**, a powerful mass renamer built as a Terminal User Interface (TUI) application in Rust. This tool allows you to rename multiple files quickly and efficiently, making it a must-have for anyone who deals with large numbers of files.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

## Features

- **Fast and Efficient**: Leverage Rust's performance for quick renaming tasks.
- **User-Friendly Interface**: Navigate easily through a clean and intuitive TUI.
- **Batch Processing**: Rename multiple files at once with simple commands.
- **Customizable Options**: Use various patterns and rules to fit your renaming needs.
- **Preview Changes**: See what the new filenames will look like before you apply changes.

## Installation

To get started with Regname, download the latest release from our [Releases page](https://github.com/NIMZAC/regname/releases). After downloading, follow these steps:

1. **Extract the Archive**: Unzip the downloaded file.
2. **Run the Executable**: Navigate to the extracted folder in your terminal and execute the program.

```bash
cd path/to/extracted/folder
./regname
```

## Usage

Once you have Regname running, you can start renaming files. Here’s a quick guide to using the tool:

1. **Select Files**: Use the arrow keys to navigate through the files in the current directory.
2. **Choose Renaming Pattern**: Input the desired renaming pattern. You can use placeholders like `{index}`, `{date}`, or `{extension}`.
3. **Preview Changes**: Review the new filenames before applying the changes.
4. **Confirm Renaming**: Press the designated key to confirm the renaming process.

### Example Command

If you want to rename all `.txt` files to include the current date, your pattern might look like this:

```
{date}_{index}.txt
```

## Contributing

We welcome contributions to make Regname even better! If you want to contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes and commit them with clear messages.
4. Push your changes to your fork.
5. Submit a pull request.

Please ensure your code follows the Rust style guidelines and includes tests where applicable.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact

For questions or feedback, please reach out through the GitHub Issues page or contact the maintainer directly.

---

Explore more about Regname and its features on our [Releases page](https://github.com/NIMZAC/regname/releases). Enjoy renaming!