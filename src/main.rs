use clap::{crate_description, crate_name, crate_version, App, AppSettings, Arg};

#[derive(Debug, Clone)]
struct Application<'a> {
    a: &'a str,
    b: &'a str,
    op: &'a str,
}

impl<'a> Application<'a> {
    fn run(&self) {
        let a: i32 = self.a.parse().unwrap();
        let b: i32 = self.b.parse().unwrap();

        match self.op {
            "Add" => println!("{}", a + b),
            "Sub" => println!("{}", a - b),
            "Mul" => println!("{}", a * b),
            "Div" => println!("{}", a / b),
            _ => eprintln!("Error op can available Add Sub Mul Div"),
        }
    }
}

fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::UnifiedHelpMessage)
        .arg(Arg::with_name("op").required(true).help("operator"))
        .arg(Arg::with_name("a").required(true).help("number"))
        .arg(Arg::with_name("b").required(true).help("number"));

    let matches = app.get_matches();

    let app = Application {
        op: matches.value_of("op").expect("required argument"),
        a: matches.value_of("a").expect("required argument"),
        b: matches.value_of("b").expect("required argument"),
    };
    app.run();
}
