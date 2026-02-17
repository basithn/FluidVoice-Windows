# AI Provider

## Purpose

Protocol and default implementation for non-streaming chat completion used for **dictation cleanup**: one system prompt + user (transcribed) text → one cleaned text response. OpenAI-compatible HTTP API; local endpoints skip Authorization header.

## Source files

- `FluidVoice-1.5.5/Sources/Fluid/Networking/AIProvider.swift`

## Depends on

- `Foundation`, `URLSession`
- Request/response: model, messages (system + user), temperature, optional reasoning_effort (Groq), stream: false

## Consumed by

- ASRService (or dictation post-processing path): after transcription, if AI processing enabled, calls provider with dictation system prompt and raw text; result becomes final text to type/copy.
- Caller supplies: systemPrompt, userText, model, apiKey, baseURL, stream (false for this path).

## Contract

### Protocol

```text
protocol AIProvider {
  func process(systemPrompt: String, userText: String, model: String, apiKey: String, baseURL: String, stream: Bool) async -> String
}
```

- Returns: full response content string or error message string (e.g. "Error: Invalid URL").

### OpenAICompatibleProvider

- **URL:** If baseURL does not already contain "/chat/completions" (or "/api/chat", "/api/generate"), appends "/chat/completions".
- **Local endpoint:** Host is localhost, 127.x, 10.x, 192.168.x, 172.16–31.x → do not add Authorization header.
- **Body:** model, messages ([system, user]), temperature (0.2 unless reasoning model), optional reasoning_effort ("low" for gpt-oss), stream.
- **Reasoning models:** o1, o3, gpt-5 prefix → temperature omitted. gpt-oss or openai/ prefix → reasoning_effort "low".
- **Response:** Parses JSON for choices[0].message.content; returns that string or error description.

## Invariants

- No streaming in this path; single POST, single response.
- API key may be empty for local endpoints; caller still passes it.
