use minijinja::Environment;
use serde::{Deserialize, Serialize};

fn main() {
    let config = read_config("../config.yaml");
    let mut env = Environment::new();
    env.set_trim_blocks(true);
    let templates = vec![
        Template::new("action", "../templates/action.yml.j2", "../action.yml"),
        Template::new("readme", "../templates/README.md.j2", "../README.md"),
    ];

    read_templates(&templates, &mut env);
    write_rendered_templates(&templates, &env, &config);
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    action_name: String,
    description: String,
    author: String,
    inputs: Vec<Input>,
    outputs: Vec<Output>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Input {
    name: String,
    description: String,
    required: bool,
    default: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Output {
    name: String,
    description: String,
    value: String,
}

#[derive(Debug)]
struct Template {
    name: String,
    content: String,
    rendered_path: String,
}

impl Template {
    fn new(name: &str, src_path: &str, rendered_path: &str) -> Self {
        let content = std::fs::read_to_string(src_path)
            .unwrap_or_else(|_| panic!("Failed to read template file: {}", src_path));

        Template {
            name: name.to_string(),
            content,
            rendered_path: rendered_path.to_string(),
        }
    }
}

fn read_config(path: &str) -> Config {
    let config_str = std::fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Failed to read config file: {}", path));
    serde_saphyr::from_str(&config_str).expect("Failed to parse config JSON")
}

fn read_templates<'a>(templates: &'a [Template], env: &mut Environment<'a>) {
    for t in templates {
        println!("Reading template: {}", t.name);
        env.add_template(&t.name, &t.content)
            .expect("Failed to add template to environment");
    }
}

fn write_rendered_templates(templates: &[Template], env: &Environment, config: &Config) {
    for t in templates {
        let tmpl = env
            .get_template(&t.name)
            .expect("Failed to get template from environment");

        println!("Rendering template: {}", t.name);
        let rendered = tmpl.render(config).expect("Failed to render template");

        println!("Writing rendered template to: {}", t.rendered_path);
        std::fs::write(&t.rendered_path, &rendered).unwrap_or_else(|_| {
            panic!(
                "Failed to write rendered template to file: {}",
                t.rendered_path
            )
        });
    }
}
