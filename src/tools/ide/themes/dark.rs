use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Theme {
    name: String,
    colors: Colors,
    syntax: SyntaxColors,
}

#[derive(Serialize, Deserialize)]
pub struct Colors {
    // UI Colors
    background: String,
    foreground: String,
    selection: String,
    cursor: String,
    line_number: String,
    line_number_active: String,
    gutter_background: String,
    
    // Editor UI
    editor_background: String,
    editor_foreground: String,
    editor_line_highlight: String,
    editor_selection: String,
    editor_word_highlight: String,
    editor_indent_guide: String,
    
    // Sidebar
    sidebar_background: String,
    sidebar_foreground: String,
    sidebar_selection: String,
    
    // Status Bar
    statusbar_background: String,
    statusbar_foreground: String,
    statusbar_error: String,
    statusbar_warning: String,
    
    // Tabs
    tab_active_background: String,
    tab_inactive_background: String,
    tab_active_foreground: String,
    tab_inactive_foreground: String,
}

#[derive(Serialize, Deserialize)]
pub struct SyntaxColors {
    // Keywords
    keyword: String,
    control: String,
    operator: String,
    
    // Literals
    string: String,
    number: String,
    boolean: String,
    
    // Identifiers
    type_name: String,
    function: String,
    variable: String,
    constant: String,
    
    // Smart Contract
    contract: String,
    storage: String,
    event: String,
    modifier: String,
    
    // Comments
    comment: String,
    doc_comment: String,
    
    // Special
    error: String,
    warning: String,
    info: String,
    hint: String,
}

pub fn stremax_dark() -> Theme {
    Theme {
        name: "Stremax Dark".to_string(),
        colors: Colors {
            // UI Colors
            background: "#1a1a1a".to_string(),
            foreground: "#d4d4d4".to_string(),
            selection: "#264f78".to_string(),
            cursor: "#00ff99".to_string(),
            line_number: "#858585".to_string(),
            line_number_active: "#c6c6c6".to_string(),
            gutter_background: "#1e1e1e".to_string(),
            
            // Editor UI
            editor_background: "#1e1e1e".to_string(),
            editor_foreground: "#d4d4d4".to_string(),
            editor_line_highlight: "#282828".to_string(),
            editor_selection: "#264f78".to_string(),
            editor_word_highlight: "#343434".to_string(),
            editor_indent_guide: "#404040".to_string(),
            
            // Sidebar
            sidebar_background: "#252526".to_string(),
            sidebar_foreground: "#cccccc".to_string(),
            sidebar_selection: "#37373d".to_string(),
            
            // Status Bar
            statusbar_background: "#007acc".to_string(),
            statusbar_foreground: "#ffffff".to_string(),
            statusbar_error: "#f48771".to_string(),
            statusbar_warning: "#cca700".to_string(),
            
            // Tabs
            tab_active_background: "#1e1e1e".to_string(),
            tab_inactive_background: "#2d2d2d".to_string(),
            tab_active_foreground: "#ffffff".to_string(),
            tab_inactive_foreground: "#969696".to_string(),
        },
        syntax: SyntaxColors {
            // Keywords
            keyword: "#569cd6".to_string(),
            control: "#c586c0".to_string(),
            operator: "#d4d4d4".to_string(),
            
            // Literals
            string: "#ce9178".to_string(),
            number: "#b5cea8".to_string(),
            boolean: "#569cd6".to_string(),
            
            // Identifiers
            type_name: "#4ec9b0".to_string(),
            function: "#dcdcaa".to_string(),
            variable: "#9cdcfe".to_string(),
            constant: "#4fc1ff".to_string(),
            
            // Smart Contract
            contract: "#00ff99".to_string(),
            storage: "#00b8d4".to_string(),
            event: "#ff9d00".to_string(),
            modifier: "#d16969".to_string(),
            
            // Comments
            comment: "#6a9955".to_string(),
            doc_comment: "#608b4e".to_string(),
            
            // Special
            error: "#f48771".to_string(),
            warning: "#cca700".to_string(),
            info: "#75beff".to_string(),
            hint: "#008080".to_string(),
        },
    }
} 