# Git scraping tool for MultiversX example contracts.

## Overview

The aim of this tool is to scrape the source code of example contracts from the MultiversX GitHub repository and save it to a file that serves as training data for AI models.

## Features

- Fetches contract source code from `src` directories
- Retrieves contract documentation (README.md)
- Captures contract configuration (Cargo.toml)
- Includes interactor code when available
- Formats output in a consistent, readable structure

## Setup

### GitHub Authentication

1. Generate a new GitHub Token [here](https://github.com/settings/tokens)
   - Select "Public Repositories (read-only)" access
   - Set expiration to a value less than 365 days (mvx limit)
   
### Config file

2. Create a `config.toml` file in the git-scraper directory:

```toml title=config.toml
github_token = "your_github_token"
```

## Usage

Run the tool from the mx-sdk-rs root directory using:
```bash
cargo run --bin git-scraper
```

The paths are not yet relative, so running from the root is required.

The tool will:
1. Fetch all example contracts
2. Process their contents
3. Save the data to `git-scraper/contracts_dump.txt`

After the file is created, it can be imported into known AI agents for training. The agents should then be able to generate new contracts based on the examples and along with the interactor code.

