#[derive(Debug, PartialEq, Clone)]
pub enum LayoutMode {
    ThreePane,     // Full width: folders | emails | preview
    TwoPane,       // Medium: folders+emails | preview
    SinglePane,    // Narrow: stack vertically
}

impl LayoutMode {
    pub fn from_width(width: f32) -> Self {
        if width > 1000.0 {
            LayoutMode::ThreePane
        } else if width > 700.0 {
            LayoutMode::TwoPane
        } else {
            LayoutMode::SinglePane
        }
    }
    
    pub fn display_name(&self) -> &'static str {
        match self {
            LayoutMode::ThreePane => "Three-Pane",
            LayoutMode::TwoPane => "Two-Pane",
            LayoutMode::SinglePane => "Single-Pane",
        }
    }
}

pub struct ResponsiveLayout;

impl ResponsiveLayout {
    pub fn calculate_folder_width(available_width: f32) -> f32 {
        (available_width * 0.2).max(150.0).min(250.0)
    }
    
    pub fn calculate_email_width(available_width: f32) -> f32 {
        (available_width * 0.35).max(300.0).min(500.0)
    }
    
    pub fn calculate_left_pane_width(available_width: f32) -> f32 {
        (available_width * 0.45).max(350.0)
    }
}