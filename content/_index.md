+++
title = "Home"
+++

<div class="hero">
    <div style="margin-left: auto; margin-right: auto; text-align: center; max-width: 55ch;">
        <p style="margin-top: 5vh; line-height: 150%;">
            transformrs is an <span class="emphasize">interface</span> to multiple <span class="emphasize">AI APIs</span> providers.
        </p>
    </div>
</div>

## Examples

First, set your API key either in an `.env` file or as an environment variable.
For example, for DeepInfra, set `DEEPINFRA_KEY` in `.env`:

```env
DEEPINFRA_KEY=<KEY>
```

and add the library to your `Cargo.toml`:

```toml
[dependencies]
futures-util = "0.3" # Only required for `stream_chat_completion`.
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
transformrs = "0.2.1"
```

Then, you can use the API as follows.

### Chat Completion

_Tested with:_ DeepInfra, OpenAI

```rust
use transformrs::openai;
use transformrs::Message;
use transformrs::Provider;

#[tokio::main]
async fn main() {
    let messages = vec![
        Message {
            role: "system".to_string(),
            content: "You are a helpful assistant.".to_string(),
        },
        Message {
            role: "user".to_string(),
            content: "This is a test. Please respond with 'hello world'.".to_string(),
        },
    ];
    let keys = transformrs::load_keys(".env");
    let key = keys.for_provider(&Provider::DeepInfra).unwrap();
    let model = "meta-llama/Llama-3.3-70B-Instruct";
    // Using the OpenAI-compatible API for chat completions.
    let resp = openai::chat_completion(&key, model, &messages)
        .await
        .unwrap();
    println!("{}", resp.choices[0].message.content);
}
```

```raw
hello world
```

### Streaming Chat Completion

_Tested with:_ DeepInfra, OpenAI

```rust
use futures_util::stream::StreamExt;
use transformrs::openai;
use transformrs::Message;
use transformrs::Provider;

#[tokio::main]
async fn main() {
    let messages = vec![
        Message {
            role: "system".to_string(),
            content: "You are a helpful assistant.".to_string(),
        },
        Message {
            role: "user".to_string(),
            content: "This is a test. Please respond with 'hello world'.".to_string(),
        },
    ];
    let keys = transformrs::load_keys(".env");
    let key = keys.for_provider(&Provider::DeepInfra).unwrap();
    let model = "meta-llama/Llama-3.3-70B-Instruct";
    // Using the OpenAI-compatible API for streaming chat completions.
    let mut stream = openai::stream_chat_completion(&key, model, &messages)
        .await
        .unwrap();
    while let Some(resp) = stream.next().await {
        let resp = resp.unwrap();
        println!("{}", resp.choices[0].delta.content.clone().unwrap_or_default());
    }
}
```

```raw
hello
world
```


### Text to Speech

_Tested with:_ DeepInfra

```rust
use std::fs::File;
use std::io::Write;
use transformrs::Provider;

#[tokio::main]
async fn main() {
    let keys = transformrs::load_keys(".env");
    let key = keys.for_provider(&Provider::DeepInfra).unwrap();
    let mut config = transformrs::text_to_speech::TTSConfig::default();
    config.preset_voice = Some("am_echo".to_string());
    let msg = "Hello, world! This is a test of the TTS API.";
    let model = "hexgrad/Kokoro-82M".to_string();
    let resp = transformrs::text_to_speech::tts(&key, config, &model, msg)
        .await
        .unwrap();
    let bytes = resp.base64_decode().unwrap();
    let mut file = File::create("test.mp3").unwrap();
    file.write_all(&bytes).unwrap();
}
```

### Text to Image

_Tested with:_ Hyperbolic

```rust
use std::fs::File;
use std::io::Write;
use transformrs::Provider;

#[tokio::main]
async fn main() {
    let keys = transformrs::load_keys(".env");
    let key = keys.for_provider(&Provider::Hyperbolic).expect("no key");
    let mut config = transformrs::text_to_image::TTIConfig::default();
    config.model = "FLUX.1-dev".to_string();
    let prompt = "A beautiful sunset over a calm ocean.";
    let resp = transformrs::text_to_image::text_to_image(&key, config, prompt)
        .await
        .unwrap();

    let image = &resp.images[0];
    let bytes = image.base64_decode().unwrap();
    let mut file = File::create("sunset.jpg").unwrap();
    file.write_all(&bytes).unwrap();
}
```
