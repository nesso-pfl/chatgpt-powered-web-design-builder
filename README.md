# [WIP]chatgpt-powered-web-design-builder
ChatGPT outputs your desired web design in HTML + CSS (+ JavaScript) format.

## Prerequisites
The following is required to run this program:

- <a href="https://platform.openai.com/account/api-keys" target="_blank">API key of ChatGPT</a>
  - You also need to have a ChatGPT account.
- <a href="https://github.com/rust-lang/cargo" target="_blank">cargo</a>

## Usage
First, create a `.env` file containing the ChatGPT API key in the root directory of the project:
```
CHATGPT_API_KEY=your_api_key
```

Then, run the following command to generate HTML in the `<project_directory>/output/default` directory.
``` sh
cargo run '<request to ChatGPT>'
```

Open the HTML file in your preferred web browser to see the implemented design.
