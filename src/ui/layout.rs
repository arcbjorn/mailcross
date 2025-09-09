#[derive(Debug, PartialEq, Clone)]
pub enum LayoutMode {
    ThreePane,     // Fullscreen: folders | emails | preview (1200+)
    TwoPane,       // Half horizontal: folders+emails | preview (800+) 
    CompactPane,   // Half vertical: stack with priority (500+)
    MobilePane,    // Quarter/small: minimal single column (0+)
}

impl LayoutMode {
    pub fn from_width(width: f32) -> Self {
        if width >= 1200.0 {
            LayoutMode::ThreePane      // Fullscreen comfort
        } else if width >= 800.0 {
            LayoutMode::TwoPane        // Half horizontal
        } else if width >= 500.0 {
            LayoutMode::CompactPane    // Half vertical
        } else {
            LayoutMode::MobilePane     // Quarter/mobile
        }
    }
    
    #[allow(dead_code)] // May be used for debugging or future features
    pub fn display_name(&self) -> &'static str {
        match self {
            LayoutMode::ThreePane => "Fullscreen",
            LayoutMode::TwoPane => "Half Horizontal", 
            LayoutMode::CompactPane => "Half Vertical",
            LayoutMode::MobilePane => "Mobile",
        }
    }
}

pub struct ResponsiveLayout;

impl ResponsiveLayout {
    // Minimal spacing and padding constants
    pub const PANEL_SPACING: f32 = 4.0;
    pub const INNER_PADDING: f32 = 8.0;
    pub const SEPARATOR_WIDTH: f32 = 1.0;
    
    // Three-pane layout (fullscreen 1200+)
    pub fn calculate_folder_width_fullscreen(available_width: f32) -> f32 {
        (available_width * 0.18).clamp(180.0, 240.0)  // Slightly wider for comfort
    }
    
    pub fn calculate_email_width_fullscreen(available_width: f32) -> f32 {
        (available_width * 0.32).clamp(320.0, 450.0)  // Optimal email list width
    }
    
    // Two-pane layout (half horizontal 800+)
    pub fn calculate_left_pane_width_half(available_width: f32) -> f32 {
        (available_width * 0.42).clamp(300.0, 400.0)  // Balanced split
    }
    
    // Compact layout helpers (half vertical 500+)
    pub fn calculate_folder_height_compact() -> f32 {
        120.0  // Horizontal folder strip
    }
    
    pub fn calculate_email_height_compact(available_height: f32) -> f32 {
        ((available_height - Self::calculate_folder_height_compact()) * 0.45).max(200.0)
    }
    
    // Mobile layout (quarter <500)
    pub fn calculate_mobile_item_height() -> f32 {
        32.0  // Compact list items
    }
}