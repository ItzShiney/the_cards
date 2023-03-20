use convert_case::Case;
use convert_case::Casing;
use std::fs::{self};
use std::io::{self};
use std::path::Path;

struct SnakePascal {
    snake: String,
    pascal: String,
}

struct Method {
    name: String,
    ret: String,
}

impl From<(&str, &str)> for Method {
    fn from((name, ret): (&str, &str)) -> Self {
        Self { name: name.into(), ret: ret.into() }
    }
}

fn add_sep(res: &mut String) {
    *res += "\n";
}

fn add_all_method(res: &mut String, cases: &[SnakePascal]) {
    *res += "    pub fn all() -> Vec<Self> {\n";
    *res += "        vec![\n";
    for SnakePascal { snake: _, pascal } in cases.iter() {
        *res += format!("            Self::{pascal},\n").as_str();
    }
    *res += "        ]\n";
    *res += "    }\n";
}

fn add_lazy_method(res: &mut String, cases: &[SnakePascal], Method { name, ret }: Method) {
    *res += "    #[allow(non_upper_case_globals, clippy::explicit_auto_deref)]\n";
    *res += format!("    pub fn {name}(self) -> &'static {ret} {{\n").as_str();
    *res += "        lazy_static! {\n";
    for SnakePascal { snake, pascal } in cases.iter() {
        *res += format!("            static ref {pascal}: {ret} = {snake}::{name}();\n").as_str();
    }
    *res += "        };\n";
    *res += "\n";
    *res += "        match self {\n";
    for SnakePascal { snake: _, pascal } in cases {
        *res += format!("            Self::{pascal} => &*{pascal},\n").as_str();
    }
    *res += "        }\n";
    *res += "    }\n";
}

fn add_copy_method(res: &mut String, cases: &[SnakePascal], Method { name, ret }: Method) {
    *res += format!("    pub fn {name}(self) -> {ret} {{\n").as_str();
    *res += "        match self {\n";
    for SnakePascal { snake, pascal } in cases.iter() {
        *res += format!("            Self::{pascal} => {snake}::{name}(),\n").as_str();
    }
    *res += "        }\n";
    *res += "    }\n";
}

fn file_names(dir: &Path) -> io::Result<Vec<SnakePascal>> {
    let mut res = Vec::<SnakePascal>::default();

    for file in fs::read_dir(dir)? {
        let file = file?;
        if !file.file_type()?.is_file() {
            continue;
        }

        let snake = String::from(file.file_name().to_str().unwrap().strip_suffix(".rs").unwrap());
        let pascal = snake.to_case(Case::Pascal);

        res.push(SnakePascal { snake, pascal });
    }

    Ok(res)
}

fn make_enum_file(
    input_dir: &Path,
    enum_name: &str,
    copy_methods: impl IntoIterator<Item = Method>,
    lazy_methods: impl IntoIterator<Item = Method>,
) -> io::Result<String> {
    let mut res = String::default();

    {
        res += "use lazy_static::lazy_static;\n";
        res += "use crate::card_uses::*;\n";
        add_sep(&mut res);
    }

    let cases = file_names(Path::new("src/").join(input_dir).as_path())?;

    {
        let input_dir = input_dir.to_str().unwrap();
        for SnakePascal { snake, pascal: _ } in cases.iter() {
            res += format!("#[path = \"{input_dir}{snake}.rs\"] mod {snake};\n").as_str();
        }
    }
    add_sep(&mut res);
    {
        res += "#[derive(Clone, Copy, PartialEq, Eq)]\n";
        res += format!("pub enum {enum_name} {{\n").as_str();
        for SnakePascal { snake: _, pascal } in cases.iter() {
            res += format!("    {pascal},\n").as_str();
        }
        res += "}\n";
    }
    add_sep(&mut res);
    {
        res += format!("impl {enum_name} {{\n").as_str();
        {
            add_all_method(&mut res, &cases);
            add_sep(&mut res);

            for method in lazy_methods {
                add_lazy_method(&mut res, &cases, method);
                add_sep(&mut res);
            }

            for method in copy_methods {
                add_copy_method(&mut res, &cases, method);
                add_sep(&mut res);
            }
        }
        res += "}\n";
    }

    Ok(res)
}

fn main() -> io::Result<()> {
    fs::write(
        Path::new("src/chrs.rs"),
        make_enum_file(
            Path::new("chrs/"),
            "CharacterType",
            [Method { name: "stats".into(), ret: "Stats".into() }],
            [
                Method::from(("name", "CustomString")),
                Method::from(("groups", "Groups")),
                Method::from(("description", "CustomString")),
                Method::from(("abilities", "GameCallbacks")),
            ],
        )?,
    )?;

    fs::write(
        Path::new("src/acts.rs"),
        make_enum_file(
            Path::new("acts/"),
            "ActiveType",
            [],
            [
                Method::from(("name", "CustomString")),
                Method::from(("groups", "Groups")),
                Method::from(("description", "CustomString")),
                Method::from(("abilities", "GameCallbacks")),
            ],
        )?,
    )?;

    Ok(())
}
