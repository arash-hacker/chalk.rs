extern crate chalk;


pub trait Chalk {
    fn bold(self)->&'static str;
}

impl<'a> Chalk for &'a str {

    fn bold(self)->&'static str{
        "hihihihih"
    }
}

fn main(){
    
    println!("{}","dfa".bold());
}