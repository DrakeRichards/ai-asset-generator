# AI Asset Generator

Generates descriptions and images of defined "assets" using AI. Saves the generated assets to a markdown file for use in Obsidian or other note-taking apps. Also generates a corresponding image. I created this to generate random RPG assets for use in my D&D campaign, but it can be used for any kind of asset generation.

## Installation

### From Source

1. Clone the repository:

   ```bash
   git clone git@github.com:DrakeRichards/ai-asset-generator.git
   ```

1. Install the dependencies:

   ```bash
   cd ai-asset-generator
   cargo build --release
   ```

1. If using the OpenAI API (default), set the `OPENAI_API_KEY` environment variable to your OpenAI API key. Alternatively, you can save this key in a .env file in the same directory as the binary.
1. Check the help message for usage instructions:

   ```bash
   ./target/release/ai-asset-generator --help
   ```

### Binary

Not yet implemented. I'll figure out how to do this later.

1. Download the compiled binary from the [releases page](https://github.com/DrakeRichards/ai-asset-generator/releases).
1. If using the OpenAI API (default), set the `OPENAI_API_KEY` environment variable to your OpenAI API key. Alternatively, you can save this key in a .env file in the same directory as the binary.
1. Check the help message for usage instructions:

   ```bash
   ./ai-asset-generator --help
   ```

## Usage

This tool takes a TOML configuration file as an input and generates a random RPG asset based on the configuration. The generated asset is saved to a markdown file and an image file. Below is an example configuration file:

```toml
output_directory = "."

[random_phrase_generator]
csv_files = ["test/animals.csv", "test/verbs.csv"]

[llm_structured_response]
provider = "OpenAi"
json_schema_file = "test/example.schema.json"
system_prompt = "You are a tester for my JSON schema file. You need to provide a JSON object that matches the schema."
initial_prompt = "Provide a JSON object that matches the schema."

[llm_structured_response.provider_config]
model = "gpt-4o-2024-08-06"

[ai_images.provider]
name = "OpenAi"

[ai_images.provider.config]
url = "http://localhost:7860"

[ai_images.params]
prompt = "A beautiful sunset over the ocean."
output_directory = "."
width = 1024
height = 1024
steps = 15
sampler_name = "UniPC"
cfg_scale = 2

[markdown_template_filler]
template_file_path = "test/example-template.md"
```

Each section is defined as follows:

### Top-Level

- `output_directory`: The directory to save the generated markdown file. Default is the current directory.

### `random_phrase_generator`

Parameters for generating a random phrase to be used in the generation, if no input is provided by the user.

- `csv_files`: An array of CSV files containing phrases to be used in the generation. Each file needs to have a header row of `value,weight` followed by the phrases to be used. The `value` column contains the phrase, and the `weight` column contains the weight of the phrase. The weight is used to determine how likely the phrase is to be selected. The higher the weight, the more likely the phrase is to be selected.

### `llm_structured_response`

Parameters for generating a response to a structured input from an LLM provider like OpenAI.

- `provider`: The provider to use for the generation. Currently, only `OpenAi` is supported.
- `json_schema_file`: The path to the JSON schema file to use for the generation. See the [OpenAI Structered Outputs specification](https://platform.openai.com/docs/guides/structured-outputs) for more information.
   - If you want to generate an image using the `ai_images` section, you need to include an `image_prompt` key in the JSON schema file.
- `system_prompt`: The system prompt to use for the generation.
- `initial_prompt`: The initial prompt to use for the generation. This is the default initial prompt that will be used if no initial prompt is provided by either the user or the `random_phrase_generator` section.

#### `llm_structured_response.provider_config`

- `model`: The model to use for the generation. See the [OpenAI API documentation](https://beta.openai.com/docs/api-reference/completions/create) for more information.

### `ai_images`

Parameters for generating an image using an AI model such as DALL-E or Stable Diffusion.

#### `ai_images.provider`

- `name`: The provider to use for the generation. Currently, only `OpenAi` and `StableDiffusion` are supported.

#### `ai_images.provider.config`

- `url`: The URL of the provider's API. Only needed if Stable Diffusion is the provider.

#### `ai_images.params`

- `prompt`: The prompt to use for the generation. This is the default prompt that will be used if no image prompt is returned from the `llm_structured_response` section.
- `output_directory`: The directory to save the generated image. Default is the current directory.
- `width`: The width of the generated image. Default is `1024`.
- `height`: The height of the generated image. Default is `1024`.
- `steps`: The number of steps to use in the generation. Default is `15`. Only used by Stable Diffusion.
- `sampler_name`: The name of the sampler to use in the generation. Default is `UniPC`. Only used by Stable Diffusion.
- `cfg_scale`: The scale of the configuration. Default is `2`. Only used by Stable Diffusion.

### `markdown_template_filler`

Parameters for filling in a markdown template with the generated content.

- `template_file_path`: The path to the markdown template file to fill in. The template file should contain placeholders that will be replaced with the generated content. Placeholders should be in the format `{{ key_name }}`.
   - If you want to include an image in the markdown file, use the placeholder `{{ image_file_name }}`. Since this tool assumes you will be using wikilinks-style image links, it strips out all but the name and extension of the image file.

## Examples

Test that the example configuration file works:

```bash
./ai-asset-generator --config test/example-config.toml
```

Output:

```markdown
# Dog

This animal is jumping.
```
