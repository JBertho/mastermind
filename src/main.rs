use ansi_term::Colour;
use std::io::Write;

#[derive(Debug,PartialEq)]
enum Color {
    White,
    Yellow,
    Purple,
    Cyan,
    Red,
    Green,
    Blue,
    Black,
    None
}

fn main() {

    let mut play = true;
    let mut times = 0;
    
    let mut guess: Vec<Color> = Vec::new();
    guess.push(Color::White);
    guess.push(Color::Blue);
    guess.push(Color::Red);
    guess.push(Color::Yellow);
    guess.push(Color::White);

    while  play == true {

        print!("Please enter some text: ");
        std::io::stdout().flush();
        let mut user_input: String = String::new();
        std::io::stdin().read_line(&mut user_input);
        user_input = String::from(user_input.trim());
        if user_input.len() != 5 {
            print!("You need 5 letters");
            continue;
        }
        times = times + 1;

        let color_user: Vec<Color> = add_user_colors(user_input); 

        let result = check_equality(&guess, &color_user);

        fancy_print_guess(&color_user);
        if result == true {
            println!("YOU WIN! With {} try", times);
            play = false;
        } else {
            println!("You have {} colors well placed !", number_of_well_placed_pawns(&guess, &color_user));
            println!("And have {} colors not well placed !", number_of_not_well_placed_pawns(&guess, &color_user));

        }
    }
}
fn add_user_colors(user_input: String) -> Vec<Color> {
    
    let mut color_user: Vec<Color> = Vec::new(); 
    for c in user_input.chars() {
        match c {
            'W' => color_user.push(Color::White),
            'Y' => color_user.push(Color::Yellow),
            'P' => color_user.push(Color::Purple),
            'C' => color_user.push(Color::Cyan),
            'R' => color_user.push(Color::Red),
            'G' => color_user.push(Color::Green),
            'B' => color_user.push(Color::Black),
            'b' => color_user.push(Color::Blue),
            _ => color_user.push(Color::None)
        }
    }
    return color_user;
}

fn number_of_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32 {
    let mut well_placed = 0;
    for (i,x) in guess.iter().enumerate() {
        if guess[i] == secret[i] {
            well_placed = well_placed + 1;
        }
    }
    return well_placed; 
}

fn number_of_not_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32 {
    let mut not_well_placed = 0;
    let mut well_placed_table = [0;5];

    for (i,x) in guess.iter().enumerate() {
        if guess[i] == secret[i] {
            well_placed_table[i] = 1;
        }
    }
    
    for (si,sx) in secret.iter().enumerate() {
        if well_placed_table[si] == 0 {
            for (gi,gx) in guess.iter().enumerate() {
                if well_placed_table[gi] == 0 && si != gi && sx == gx {
                    not_well_placed = not_well_placed + 1;
                }
            }
        }

    }
    return not_well_placed;
}


fn check_equality(guess: &[Color], user_input: &[Color]) -> bool {
    for (i,x) in guess.iter().enumerate() {
        if guess[i] != user_input[i]{
            return false;
        }
    
    }
    return true;
}

fn fancy_print_guess(guess: &[Color]) {
    for color in guess {
        match color {
            Color::White => print!("{}", Colour::White.paint("W")),
            Color::Yellow => print!("{}", Colour::Yellow.paint("Y")),
            Color::Purple => print!("{}", Colour::Purple.paint("P")),
            Color::Cyan => print!("{}", Colour::Cyan.paint("C")),
            Color::Red => print!("{}", Colour::Red.paint("R")),
            Color::Green => print!("{}", Colour::Green.paint("G")),
            Color::Black => print!("{}", Colour::Black.paint("B")),
            Color::Blue => print!("{}",Colour::Blue.paint("b")),
            Color::None => print!("ERROR"),
        }
    }
    println!("");
}