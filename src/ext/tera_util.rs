use std::fs;
use tera::{Tera, Context};
use nom_sql::parser;

pub trait Render {
    fn render_string(self: &mut Self, template: &str, context: Option<Context>) -> String;
}

pub struct RenderEngine {
    tera: Tera,
    default_context: Context
}
impl RenderEngine {
    fn new(template_path: &str)-> Self {
        return RenderEngine {
            tera: compile_templates!(format!("{}/**/*", template_path).as_str()),
            default_context: Context::default()
        };
    }
}

impl Render for RenderEngine {

    fn render_string(self: &mut Self, template: &str, context: Option<Context>) -> String {
        self.tera.add_raw_template("template", template)
            .expect("render_string(): tera failed to add raw template ");

        let mut merged_context = Context::new();
        merged_context.extend(self.default_context.clone());
        if context.is_some() {
            merged_context.extend(context.unwrap())
        }

        return match self.tera.render("template", &merged_context) {
            Ok(r) => r,
            Err(e) => {
                println!("{}", e);
                template.to_string()
            }
        }
    }
}

pub fn build_tera(template_path: &str) -> Tera {
    return compile_templates!(format!("{}/**/*", template_path).as_str());
}

pub fn render_string(tera: &mut Tera, context: &Context, content_string: &Option<&str>) -> Option<String> {
    if content_string.is_none() {
        return None;
    }

    tera.add_raw_template("template", content_string.as_ref().unwrap())
        .expect("render_string(): tera failed to add raw template ");

    return match tera.render("template", context) {
        Ok(r) => Some(r),
        Err(e) => {
            println!("{}", e);
            Some(content_string.unwrap().to_string())
        }
    }
}

#[test]
fn test_render_engine() {
    let mut render_engine = RenderEngine{ tera: tera::Tera::default(), default_context: Context::default() };
    let mut context = Context::new();
    context.insert("val", "world");

    let result = render_engine.render_string("hello {{val}}", Some(context));
    assert_eq!("hello world", result);

    let sql_result = match parser::parse_query("select * from web") {
        Ok(v) => v,
        Err(e) => panic!("parse sql failed")
    };
    let mut context = Context::new();
    context.insert("sql", &sql_result);
    println!("{:?}", &context);

    let result = render_engine.render_string("hello {{sql.Select.tables[0].name}}", Some(context));
    println!("{:?}", result);
    assert_eq!("hello web", result);

}

#[test]
fn test_render_string() {
    let mut context = Context::new();
    context.insert("name", "wang wei");
    let mut tera = tera::Tera::default();
    let content1 = Some("hello {{name}}");
    let content2 = Some("hello {{not_exists_name}}");

    assert_eq!("", render_string(&mut tera, &context, &None).unwrap_or("".to_string()));
    assert_eq!("hello wang wei", render_string(&mut tera, &context, &content1).unwrap());
    assert_eq!("hello {{not_exists_name}}", render_string(&mut tera, &context, &content2).unwrap());


}