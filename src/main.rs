trait PrintLOL {
    fn print_lol(&self) {
        println!("LOL!");
    }
}

struct Fgh {
    data: String,
}

impl PrintLOL for Fgh {}

fn main() {
    let data = Fgh {
        data: "Data ?".to_string(),
    };
    data.print_lol();
}
