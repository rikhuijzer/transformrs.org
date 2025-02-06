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

The examples below are based on the tests in the [repository](https://github.com/rikhuijzer/transformrs/tree/main/tests).
Many tests run repeatedly against the actual APIs to ensure that the library works as expected.

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