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


fn temp(value: u8) {
    let termsize::Size { rows: _, cols } = termsize::get().unwrap();

    for color in FIRE_COLOR_PALETTE[4..8].iter() {
        println!("{}", (value as char).to_string().repeat(cols as usize).color(*color));
    }

    println!("");
}                    

pub fn main() {
    // let termsize::Size { rows: _, cols } = termsize::get().unwrap();

    // for symb in []
        // for color in FIRE_COLOR_PALETTE[4..8].iter() {
            // println!("{}", "\x40".repeat(cols as usize).color(*color));
        // }    
    
    for code in 33..=47 {
        temp(code);
    }

    for code in 58..=64 {
        temp(code);
    }

    for code in 91..=96 {
        temp(code);
    }

    for code in 123..=126 {
        temp(code);
    }
}
