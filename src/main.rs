use colored::Colorize;

const FIRE_COLOR_PALETTE : [&str; 11] = [
                                            "#000000", // Black
                                            "#400000", // Red-Black
                                            "#700000", // Darkest Red
                                            "#910000", // Dark Red
                                            "#D00000", // Red
                                            "#FFA000", // Dark Orange
                                            "#FFAA00", // Orange
                                            "#FFCC00", // Light Orange
                                            "#FFFF00", // Yellow
                                            "#FFFFA0", // Light Yellow
                                            "#FFFFF0", // Off-White
                                        ];

pub fn main() {
    let termsize::Size { rows: _, cols } = termsize::get().unwrap();

    for color in FIRE_COLOR_PALETTE.iter() {
        println!("{}", "@".repeat(cols as usize).color(*color));
    }    
}
