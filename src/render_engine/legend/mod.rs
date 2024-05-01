//! # Legend rendering module
//! 


/// ## Legend position
/// 
/// This enum represents the position of the legend in the chart.
/// 
/// ### Variants
/// - `TopLeft` - The legend is at the top left of the chart
/// - `TopMiddle` - The legend is at the top middle of the chart
/// - `TopRight` - The legend is at the top right of the chart
/// - `BottomLeft` - The legend is at the bottom left of the chart
/// - `BottomMiddle` - The legend is at the bottom middle of the chart
/// - `BottomRight` - The legend is at the bottom right of the chart
/// - `LeftMiddle` - The legend is at the left middle of the chart
/// - `RightMiddle` - The legend is at the right middle of the chart
/// 
pub enum LegendPosition {
    TopLeft,
    TopMiddle,
    TopRight,
    BottomLeft,
    BottomMiddle,
    BottomRight,
    LeftMiddle,
    RightMiddle
}