# IDE Integration Guide

Stremax provides rich IDE support through the Language Server Protocol (LSP), offering a modern development experience across multiple editors.

## Supported IDEs

### Visual Studio Code
- Official extension: `stremax.stremax-lang`
- Full language server support
- Integrated debugging
- Smart contract visualization
- Gas profiling

### IntelliJ IDEA
- Plugin ID: `io.stremax.intellij`
- Complete language support
- Custom tool windows
- Blockchain explorer integration

### Other Editors
- Vim/Neovim (via coc.nvim or native LSP)
- Emacs (via lsp-mode)
- Sublime Text (via LSP)

## Features

### 1. Smart Completions

```rust
contract Token {
    // Type '.' to see all available methods
    self.bal<cursor> // Suggests: balance, balance_of, etc.
    
    // Smart context-aware suggestions
    #[inv<cursor> // Suggests: invariant, invalid_state_handler
}
```

### 2. Real-Time Diagnostics

```rust
contract Token {
    // Inline error detection
    fn transfer(amount: u64) -> Result<()> {
        // Error: Insufficient balance check missing
        self.balances[sender] -= amount;
        // Warning: Event emission recommended
        Ok(())
    }
}
```

### 3. Code Navigation

- Go to Definition (`F12`)
- Find All References (`Shift+F12`)
- Symbol Search (`Ctrl+T`)
- Outline View
- Call Hierarchy

### 4. Smart Contract Visualization

```rust
// Use the "View Contract Graph" command to see:
#[contract]
struct Token {
    // State variables shown as nodes
    // Function calls as edges
    // Access patterns highlighted
}
```

## VS Code Setup

### 1. Installation

```bash
# Install the extension
code --install-extension stremax.stremax-lang

# Install language server
strm install language-server
```

### 2. Configuration

```json
{
    "stremax.languageServer.path": "/usr/local/bin/strm-ls",
    "stremax.format.enable": true,
    "stremax.diagnostics.gasWarnings": true,
    "stremax.completion.snippets": true
}
```

### 3. Custom Commands

- `Stremax: New Project` (`Ctrl+Shift+N`)
- `Stremax: Build Contract` (`Ctrl+Shift+B`)
- `Stremax: Deploy Contract` (`Ctrl+Shift+D`)
- `Stremax: Show Gas Usage` (`Ctrl+Shift+G`)

## IntelliJ IDEA Setup

### 1. Plugin Installation

```
Settings → Plugins → Marketplace → Search "Stremax" → Install
```

### 2. Tool Windows

- Contract Explorer (`View → Tool Windows → Stremax Contracts`)
- Gas Profiler (`View → Tool Windows → Gas Profile`)
- Network Monitor (`View → Tool Windows → Network`)

### 3. Custom Actions

```kotlin
// Add to Tools menu
action("Deploy Contract") {
    shortcut("Ctrl+Shift+D")
    perform { deployCurrentContract() }
}
```

## Language Server Features

### 1. Hover Information

```rust
contract Token {
    // Hover over types for documentation
    balance: Map<Address, Amount>, // Shows type details
    
    // Hover over functions for signatures
    fn transfer(...) // Shows full signature
}
```

### 2. Code Actions

```rust
// Quick fixes available via lightbulb
fn unsafe_transfer() {
    // Error: Missing balance check
    // Quick fix: Add balance check
    self.balances[sender] -= amount;
}
```

### 3. Semantic Highlighting

```rust
#[contract]        // Attribute macro
struct Token {     // Contract definition
    total: u64,    // Storage variable
    owner: Address // Blockchain type
}
```

## Debugging Integration

### 1. Breakpoints

```rust
contract Token {
    fn transfer(...) {
        // Regular breakpoint
        let sender = msg::sender();
        
        // Conditional breakpoint
        if amount > 1000 { // Break here if true
            log::warn!("Large transfer");
        }
    }
}
```

### 2. Debug Views

- Variables (local and contract state)
- Call Stack
- Blockchain State
- Gas Usage
- Event Log

### 3. REPL Integration

```rust
// Debug console commands
> contract.balance_of(address)
> contract.total_supply()
> help contract // Show available methods
```

## Advanced Features

### 1. Custom Views

```typescript
// Extension contribution point
"contributes": {
    "views": {
        "stremax-explorer": [
            {
                "id": "contractHierarchy",
                "name": "Contract Hierarchy"
            }
        ]
    }
}
```

### 2. Custom Commands

```typescript
// Register custom command
commands.registerCommand('stremax.deployContract', async () => {
    // Command implementation
});
```

### 3. Custom Providers

```typescript
// Custom completion provider
class StremaxCompletionProvider implements CompletionProvider {
    provideCompletions(document: TextDocument, position: Position) {
        // Provide smart completions
    }
}
```

## Best Practices

1. **Editor Configuration**
   - Use consistent formatting
   - Enable real-time linting
   - Configure auto-save

2. **Workspace Organization**
   - Use workspace folders
   - Set up proper exclude patterns
   - Configure search paths

3. **Performance**
   - Enable caching
   - Use workspace symbols
   - Configure file watching

## Common Issues and Solutions

1. **Language Server**
   ```bash
   # Restart language server
   strm language-server --restart
   
   # Clear cache
   strm language-server --clear-cache
   ```

2. **Extension Issues**
   ```bash
   # Reset extension state
   code --disable-extension stremax.stremax-lang
   code --enable-extension stremax.stremax-lang
   ```
