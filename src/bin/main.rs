use ptit::ptit::*;

fn main() {
    let dwn = download("https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/132.png");
    let ld = load(dwn.unwrap().unwrap());
    let cr = crop(ld.unwrap());
    let re = resize(cr);
    scan(re);
}