use std::{io::{Write, stdout}, thread, time};

pub const FG_RED: &str = "\x1b[0;31m";
pub const FG_GREEN: &str = "\x1b[0;32m";
pub const FG_YELLOW: &str = "\x1b[0;33m";
pub const FG_BLUE: &str = "\x1b[0;34m";
pub const FG_PURPLE: &str = "\x1b[0;35m";
pub const FG_CYAN: &str = "\x1b[0;36m";
pub const FG_WHITE: &str = "\x1b[0;37m";
pub const FG_BLACK: &str = "\x1b[0;38m";

pub const FG_LIGHTRED: &str = "\x1b[1;31m";
pub const FG_LIGHTGREEN: &str = "\x1b[1;32m";
pub const FG_LIGHTYELLOW: &str = "\x1b[1;33m";
pub const FG_LIGHTBLUE: &str = "\x1b[1;34m";
pub const FG_LIGHTPURPLE: &str = "\x1b[1;35m";
pub const FG_LIGHTCYAN: &str = "\x1b[1;36m";
pub const FG_LIGHTWHITE: &str = "\x1b[1;37m";
pub const FG_LIGHTBLACK: &str = "\x1b[1;38m";

pub const FG_DARKRED: &str = "\x1b[2;31m";
pub const FG_DARKGREEN: &str = "\x1b[2;32m";
pub const FG_DARKYELLOW: &str = "\x1b[2;33m";
pub const FG_DARKBLUE: &str = "\x1b[2;34m";
pub const FG_DARKPURPLE: &str = "\x1b[2;35m";
pub const FG_DARKCYAN: &str = "\x1b[2;36m";
pub const FG_DARKWHITE: &str = "\x1b[2;37m";
pub const FG_DARKBLACK: &str = "\x1b[2;38m";

pub const BG_RED: &str = "\x1b[0;41m";
pub const BG_GREEN: &str = "\x1b[0;42m";
pub const BG_YELLOW: &str = "\x1b[0;43m";
pub const BG_BLUE: &str = "\x1b[0;44m";
pub const BG_PURPLE: &str = "\x1b[0;45m";
pub const BG_CYAN: &str = "\x1b[0;46m";
pub const BG_WHITE: &str = "\x1b[0;47m";
pub const BG_BLACK: &str = "\x1b[0;48m";

pub const BG_LIGHTRED: &str = "\x1b[1;41m";
pub const BG_LIGHTGREEN: &str = "\x1b[1;42m";
pub const BG_LIGHTYELLOW: &str = "\x1b[1;43m";
pub const BG_LIGHTBLUE: &str = "\x1b[1;44m";
pub const BG_LIGHTPURPLE: &str = "\x1b[1;45m";
pub const BG_LIGHTCYAN: &str = "\x1b[1;46m";
pub const BG_LIGHTWHITE: &str = "\x1b[1;47m";
pub const BG_LIGHTBLACK: &str = "\x1b[1;48m";


pub const FG_RESET: &str = "\x1b[39m";
pub const BG_RESET: &str = "\x1b[49m";
pub const FBG_RESET: &str = "\x1b[0m";
pub const LN_RESET: &str = "\x1b[200D";

pub const RAINBOW:[&str;18]=[
    FG_LIGHTRED,
    FG_RED,
    FG_DARKRED,
    FG_LIGHTYELLOW,
    FG_YELLOW,
    FG_DARKYELLOW,
    FG_LIGHTGREEN,
    FG_GREEN,
    FG_DARKGREEN,
    FG_LIGHTBLUE,
    FG_BLUE,
    FG_DARKBLUE,
    FG_LIGHTPURPLE,
    FG_PURPLE,
    FG_DARKPURPLE,
    FG_LIGHTCYAN,
    FG_CYAN,
    FG_DARKCYAN,
    // FG_LIGHTWHITE,
    // FG_WHITE,
    // FG_LIGHTBLACK,
    // FG_BLACK,
];

#[warn(unused_must_use)]
pub fn to_rainbow(a:&str,cnt:i32){
    let v:Vec<_>=a.split("").collect();
    let mut r="".to_owned();
    for j in 0..cnt{
        for i in 0..a.len() as i32{
            r.push_str(v[i as usize]);
            r.push_str(RAINBOW[((j+i)%RAINBOW.len() as i32) as usize  ]);
        }
        print!("{}{}", r,"\r".to_string()+LN_RESET);
        r="".to_string();
        stdout().lock().flush().unwrap();
        thread::sleep(time::Duration::from_millis(100));
    }
}
#[warn(unused_must_use)]
pub fn to_karaoke(a:&str,_cnt:i32){
    let v:Vec<_>=a.split("").collect();
    let mut r="".to_owned();
    for j in 0..a.len() as i32{
        r.push_str(FG_LIGHTPURPLE);
        for i in 0..a.len() as i32{
            r.push_str(v[i as usize]);
            if j==i {
                r.push_str(FG_RESET);
            }
        }
        print!("{}{}", r,LN_RESET);
        r="".to_string();
        stdout().lock().flush().unwrap();
        thread::sleep(time::Duration::from_millis(500));
    }
}
// pub fn to_radar(){

// }
// pub fn to_glitch(){

// }