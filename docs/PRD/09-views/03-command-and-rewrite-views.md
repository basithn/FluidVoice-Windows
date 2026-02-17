# Command Mode View and Rewrite Mode View

## Purpose

**CommandModeView:** Full-screen or sheet UI for command mode: text input (or voice), conversation history with tool calls and thinking, streaming response, pending command confirmation, chat switcher and clear. **RewriteModeView:** UI for rewrite/write: show original text (or "Write mode"), prompt input, streamed rewritten text, actions (Type, Copy). Both are shown in the detail area when user selects Command Mode or Rewrite Mode from sidebar (or via hotkey flow).

## Source files

- `FluidVoice-1.5.5/Sources/Fluid/Views/CommandModeView.swift`
- `FluidVoice-1.5.5/Sources/Fluid/Views/RewriteModeView.swift`

## CommandModeView

### Depends on

- CommandModeService (StateObject or passed); NotchOverlayManager (optional: show command output in notch); ChatHistoryStore; LLMClient/FunctionCallingProvider (via service).
- Theme; SwiftUI.

### Contract

- Input: text field or voice trigger; on submit call commandModeService.processUserInput(text).
- Conversation: list of Message (user, assistant, tool); show thinking, stepType (checking, executing, verifying), toolCall (command, purpose). Streaming: show streamingText and streamingThinkingText.
- Pending command: when pendingCommand != nil, show command and purpose; buttons Confirm / Cancel; call confirmPendingCommand() or rejectPendingCommand().
- Chat: dropdown or list of recent chats (getRecentChats); New Chat (createNewChat); Switch (switchToChat(id)); Clear (clearHistory).
- Optional: "Run in notch" or similar to show output in NotchCommandOutputExpandedView.

### Invariants

- Disable input or New Chat while isProcessing. Show loading state when isProcessing.

## RewriteModeView

### Depends on

- RewriteModeService (StateObject or passed); TextSelectionService (via service); TypingService (via service); Theme; SwiftUI.

### Contract

- Original text: read-only or editable (originalText). If empty, show "Write mode" and prompt user to speak or type request.
- Capture selection: button "Capture Selection" calls captureSelectedText(); show success or "Select text first".
- Prompt: text field for instruction; on submit call processRewriteRequest(prompt).
- Result: show rewrittenText (streaming); optional streamingThinkingText.
- Actions: "Type into app" (typeResult()), "Copy" (ClipboardService or pasteboard).
- Write mode: no original text; prompt is the request; processRewriteRequest uses it as user message for generation.

### Invariants

- Disable submit while isProcessing. Clear or reset state when starting new rewrite (captureSelectedText or startWithoutSelection).
