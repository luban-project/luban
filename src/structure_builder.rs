use super::tera::Context;
use super::toml;
use std::fs;
use tera::Tera;
use crate::file_utils::read_to_string;
use crate::ext::tera_util::render_string;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ProjectStructureItem {
    pub item_type: String,
    pub item_path: String,
    pub item_file: Option<String>,
    pub item_tmpl: Option<String>,
}

impl ProjectStructureItem {
    pub fn is_static(&self) -> bool {
        return self.item_type.eq_ignore_ascii_case("static");
    }
    pub fn is_dynamic(&self) -> bool {
        return self.item_type.eq_ignore_ascii_case("dynamic");
    }
    pub fn is_dir(&self) -> bool {
        return self.item_type.eq_ignore_ascii_case("dir");
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectStructure {
    pub project_structure: Vec<ProjectStructureItem>,
}



pub fn parse_structure_item(
    item: &ProjectStructureItem,
    context: &Context,
) -> ProjectStructureItem {
    let mut tera = tera::Tera::default();

    let item_path_result =
        render_string(&mut tera, context, &Some(item.item_path.as_str()));

    let item_file_result = match &item.item_file {
        None => None,
        Some(f) => {
            render_string(&mut tera, context, &Some(f))
        }
    };

    let item_tmpl_result = match &item.item_tmpl {
        None => None,
        Some(t) => {
            render_string(&mut tera, context, &Some(t))
        }
    };

    return ProjectStructureItem {
        item_type: item.item_type.clone(),
        item_path: item_path_result.expect("item path must be set"),
        item_file: item_file_result,
        item_tmpl: item_tmpl_result
    };
}


pub fn parse_project_structure(template_dir: &str) -> ProjectStructure {
//    let structure_template_dir = format!("{}/structure/structure.toml", template_dir);
////    let content = match fs::read_to_string(&structure_template_dir) {
////        Err(why) => panic!("failed to read structure file {}: {}", &structure_template_dir, why),
////        Ok(content) => content,
////    };

    let structure_template_dir = format!("{}/structure/structure.toml", template_dir);
    let structure_content = read_to_string(structure_template_dir.as_str())
        .expect("failed to read structure file");

    return match toml::from_str(structure_content.as_str()) {
        Err(why) => panic!("failed to parse structure file: {}", why),
        Ok(result) => result,
    };
}





