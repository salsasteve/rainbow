# Rainbow CLI Application

The **Rainbow CLI** is a versatile command-line tool that provides various functionalities, including searching files, generating fake data, sending Telegram messages, and fetching Kanye West quotes.

## Features

1. **Search**: Search for a query in a file with optional case-insensitivity.
2. **Fake Data Generation**: Generate fake data based on specified columns and write it to a CSV file.
3. **Send Telegram Messages**: Send messages to a specified Telegram chat ID.
4. **Good Morning Kanye**: Fetch a Kanye West quote and send it to a Telegram chat.

---

## Commands and Examples

### 1. **Search**
Search for a query in a file.

#### Usage:
```bash
rainbow search --query <QUERY> --file <FILE_PATH> [--ignore-case]
```

#### Example:
```bash
rainbow search --query "hello" --file "example.txt" --ignore-case
```

#### Environment Variables:
- `QUERY`: The search query.
- `FILE`: The file path to search in.
- `IGNORE_CASE`: Set to `true` to enable case-insensitive search.

---

### 2. **Fake Data Generation**
Generate fake data and save it to a CSV file.

#### Usage:
```bash
rainbow fake-data --columns <COLUMNS> --rows <ROWS> --file-path <FILE_PATH>
```

#### Example:
```bash
rainbow fake-data --columns "FirstName:FirstName,LastName:LastName,Email:Email" --rows 10 --file-path "output.csv"
```

#### Notes:
- Columns should be specified in the format `name:type`, separated by commas.
- Supported types: `FirstName`, `LastName`, `CityName`, `StreetName`, `ZipCode`, `Email`, `Username`, `Password`, `Color`, `Paragraph`, `Number`.

---

### 3. **Send Telegram Messages**
Send a message to a Telegram chat.

#### Usage:
```bash
rainbow send-telegram --message <MESSAGE> --chat-id <CHAT_ID>
```

#### Example:
```bash
rainbow send-telegram --message "Hello, world!" --chat-id "123456789"
```

#### Environment Variables:
- `MESSAGE`: The message to send.
- `CHAT_ID`: The Telegram chat ID.

---

### 4. **Good Morning Kanye**
Fetch a random Kanye West quote and send it to a Telegram chat.

#### Usage:
```bash
rainbow good-morning-kanye --chat-id <CHAT_ID>
```

#### Example:
```bash
rainbow good-morning-kanye --chat-id "123456789"
```

---

## Environment Variables

The following environment variables can be used to configure the application:

- `QUERY`: Default query for the `search` command.
- `FILE`: Default file path for the `search` command.
- `IGNORE_CASE`: Default case-insensitivity setting for the `search` command.
- `MESSAGE`: Default message for the `send-telegram` command.
- `CHAT_ID`: Default chat ID for the `send-telegram` and `good-morning-kanye` commands.

---

## Dependencies

Ensure the following dependencies are installed:

- **Rust**: Required to build and run the application.
- **Telegram Bot Token**: Set up a bot using [BotFather](https://core.telegram.org/bots#botfather) and retrieve the token.

---

## Building and Running

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd rainbow
   ```

2. Build the application:
   ```bash
   cargo build --release
   ```

3. Run the application:
   ```bash
   ./target/release/rainbow <COMMAND>
   ```

---

## Testing

Run the tests using:
```bash
cargo test
```

---

## License

This project is licensed under the MIT License.

--- 

Let me know if you need further refinements!