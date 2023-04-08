# output-design-by-chatgpt
ChatGPT outputs your desired web design in HTML + CSS (+ JavaScript) format.

## Prerequisites
The following is required to run this program:

- [API key of ChatGPT](https://platform.openai.com/account/api-keys)
  - You also need to have a ChatGPT account.
- [cargo](https://github.com/rust-lang/cargo)

## Usage
First, create a `.env` file containing the ChatGPT API key in the root directory of the project:
```
CHATGPT_API_KEY=your_api_key
```

Then, run the following command to generate HTML in the <project_directory>/output/default directory.
``` sh
cargo run '<request to ChatGPT>'
```

Open the HTML file in your preferred web browser to see the implemented design.
