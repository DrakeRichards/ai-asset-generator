# RPG Asset Generator

Generates random RPG assets (such as characters, items, locations, etc.) using AI. Saves the generated assets to a markdown file for use in Obsidian or other note-taking apps. Also generates a corresponding image using the OpenAI API.

Note: This project is still in the early stages of development. The resources used to generate the assets are set at compile time, so you can't currently change much. I plan to add more customization options in the future.

## Features

### Currently implemented

- [x] Character generation
- [x] Building generation

### Planned

- [ ] Item generation
- [ ] Location generation
- [ ] Quest generation
- [ ] Monster generation
- [ ] Additional image generation providers
- [ ] Additional text generation providers
- [ ] Custom resource selection
- [ ] Custom markdown templates

## Installation / Usage

### From Source

1. Clone the repository:

   ```bash
   git clone git@github.com:DrakeRichards/rpg-asset-generator.git
   ```

1. Install the dependencies:

   ```bash
   cd rpg-asset-generator
   cargo build --release
   ```

1. If using the OpenAI API (default), set the `OPENAI_API_KEY` environment variable to your OpenAI API key. Alternatively, you can save this key in a .env file in the same directory as the binary.
1. Check the help message for usage instructions:

   ```bash
   ./target/release/rpg-asset-generator --help
   ```

### Binary

Not yet implemented. I'll figure out how to do this later.

1. Download the compiled binary from the [releases page](https://github.com/DrakeRichards/rpg-asset-generator/releases).
1. If using the OpenAI API (default), set the `OPENAI_API_KEY` environment variable to your OpenAI API key. Alternatively, you can save this key in a .env file in the same directory as the binary.
1. Check the help message for usage instructions:

   ```bash
   ./rpg-asset-generator --help
   ```

## Examples

Generate a random character:

```bash
./rpg-asset-generator character
```

Output:

```markdown
---
tags:
  - character
aliases:
  - "Merrilee"
location: 
obsidianEditingMode: source
obsidianUIMode: preview
---

# Merrilee Stumbleduck

*(gender:: female) (race:: halfling) (class:: Puppeteer)*

![[example.png|+character]]

## Description

Merrilee Stumbleduck is a cheerful and imaginative halfling known for her captivating puppet shows. Having grown up amidst the bustling streets of Waterdeep, she finds joy in bringing stories to life with her marionettes. Her optimistic outlook and whimsical charm endear her to both children and adults alike.

### Looks

Petite even for a halfling, Merrilee stands at just over three feet tall with a runner's build. Her curly chestnut hair is often tied back in a colorful scarf, and her bright hazel eyes twinkle with mischief and creativity.

## Hooks

### Goals

Merrilee aims to turn her puppet shows into grand theatrical performances that can tour the Sword Coast.

### Frustration

Merrilee's prized marionette has been stolen, and she's looking for brave souls to help retrieve it.

```
