trait PrintLOL {
    fn print_lol(&self) {
        println!("LOL!");
    }
}

trait PrintHello {
    fn print_hello(&self) {
        println!("Hello");
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

impl PrintHello for Fgh {}

impl PrintHello for DDD {}

fn main() {
    let data = Fgh {
        data: "Data ?".to_string(),
    };
    //get_objects(data);

    let data_return = return_object();
    data_return.print_hello();
}

fn get_object(obj: impl PrintLOL) {
    obj.print_lol();
}

fn get_objects<T>(obj: T)
    where T: PrintLOL + PrintHello {//ukazat ogranicheniya tipaja s pomoshi where
    obj.print_lol();
    obj.print_hello();
}

fn return_object()->impl PrintHello{
    Fgh{
        data: "Hello from Fgh".to_string(),
    }
}