pub mod cli;
pub mod config;

use tinytemplate::TinyTemplate;

static TEMPLATE: &'static str = "
# Hi. I'm {person.full_name}. <sup>(Online I also go by {person.screen_name}{{ for alias in person.aliases }}{{ if @last }} or{{ else }},{{ endif }} {alias}{{ endfor }})</sup>

{person.bio}
";

fn main() {
    let args = cli::get_args();
    let config = config::get_config(args.config);
    println!("{:#?}", config);

    let mut tt = TinyTemplate::new();
    tt.add_template("t", TEMPLATE).unwrap();

    let rendered = tt.render("t", &config).unwrap();
    println!("{}", rendered);
}
