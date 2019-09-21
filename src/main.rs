use std::io;
use std::io::BufRead;

extern crate rand;
use rand::Rng;

mod 
    #[derive(Copy,Clone)]

    pub enum StaticObject {
        None,
        Net,
        PlayerRepresent,
        Coin,
    }
}

fn main() {
    #[derive(Copy,Clone)]
    enum StaticObject {
        None,
        Net,
        PlayerRepresent,
        Coin,
    }

    const GMPSIZE:usize = 5;

    let mut playerpos: usize = 0;

    let mut playercoins: u32 = 0;

    let mut game_map: [StaticObject; GMPSIZE] = [StaticObject::None; GMPSIZE];
    
    let reader = io::BufReader::new(io::stdin());

    fn chngpos(pp: usize, dec: bool) -> usize {
    if dec && pp != 0 {
        (pp - 1)
    } else if pp < 4 && !dec {
        (pp + 1)
    } else {
        pp
    }
    }

    for line in reader.lines() {

        /* Get and Process Input */

        match line.unwrap().as_ref() {
            "r" => playerpos = chngpos(playerpos, false),
            "l" => playerpos = chngpos(playerpos, true),
            "x" => game_map[playerpos] = StaticObject::Net,
            _ => (),
        }

        /* The loop stuff that doesn't depend on stuff */
        if rand::thread_rng().gen_range(1, 5) == 1 {
        let random = rand::thread_rng().gen_range(1, 5);
        if std::mem::discriminant(&game_map[random]) == std::mem::discriminant(&StaticObject::Net) {
            game_map[random] = StaticObject::Coin;
        }
        }

        if std::mem::discriminant(&game_map[playerpos]) == std::mem::discriminant(&StaticObject::Coin) {
            game_map[playerpos] = StaticObject::None;
            playercoins = playercoins + 1
        }

        /* Rendering */

        let mut print_map = game_map;
        print_map[playerpos] = StaticObject::PlayerRepresent;

        for character in print_map.into_iter() {
            use StaticObject;
            print!("{}",
                match character {
                    StaticObject::None => "-",
                    StaticObject::Net => "#",
                    StaticObject::Coin => "$",
                    StaticObject::PlayerRepresent => "@",
                }
            );
        }
        print!{" {}", playercoins}
        println!();

    }

}
