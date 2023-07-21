trait PrintLOL {
    fn print_lol(&self) {
        println!("LOL!");
    }
}

struct Fgh {
    data: String,
}

struct DDD {
    num: i32,
}

impl PrintLOL for Fgh {}

impl PrintLOL for DDD {}

fn main() {
    let data = Fgh {
        data: "Data ?".to_string(),
    };
    get_object(data);
}

fn get_object(obj: impl PrintLOL) {
    obj.print_lol();
}