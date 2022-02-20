fn main() {
    if nix::unistd::Uid::effective().is_root() {
        print!("{}", include_str!("woodo.txt"));
    } else {
        println!("It's a weird tree.");
    }
}
