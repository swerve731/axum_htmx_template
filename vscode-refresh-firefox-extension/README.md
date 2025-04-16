# vscode-refresh-firefox-extension

This is a Visual Studio Code extension that automatically refreshes the Firefox browser whenever a file is saved in the editor. It is particularly useful for web developers who want to see their changes reflected immediately in the browser without having to manually refresh.

## Features

- Automatically executes a shell script to refresh Firefox on file save.
- Simple and easy to use.

## Installation

1. Clone the repository or download the ZIP file.
2. Open the project in Visual Studio Code.
3. Open the terminal and run the following command to install the dependencies:

   ```
   npm install
   ```

4. Compile the TypeScript files:

   ```
   npm run compile
   ```

5. Press `F5` to open a new VS Code window with the extension loaded.

## Usage

Once the extension is installed and running, it will automatically execute the `refresh_firefox.sh` script every time you save a file. Make sure that the script is executable and located in the correct path.

## Contributing

Feel free to submit issues or pull requests if you have suggestions or improvements for the extension.

## License

This project is licensed under the MIT License. See the LICENSE file for details.