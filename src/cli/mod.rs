use clap::App;

pub mod options;

pub fn cli() {
    let yaml = load_yaml!("cli.yml");
    let x = App::from_yaml(yaml).get_matches();
    options::from_args(&x)
}

