use {
    convert_case::{
        Case,
        Casing,
    },
    itertools::Itertools,
    std::{
        fs,
        io,
        path::Path,
    },
};

struct SnakePascal {
    snake: String,
    pascal: String,
}

#[derive(Clone, Copy)]
struct Argument {
    name: &'static str,
    type_: &'static str,
    mut_: bool,
}

fn format_mut(mut_: bool) -> &'static str {
    if mut_ {
        "mut "
    } else {
        ""
    }
}

fn format_args_definition(args: &[Argument]) -> String {
    args.iter()
        .copied()
        .map(|Argument { name, type_, mut_ }| format!("{}{name}: {type_}", format_mut(mut_)))
        .join(", ")
}

fn format_args_call(args: &[Argument]) -> String {
    args.iter().copied().map(|arg| arg.name).join(", ")
}

type MatchBranchGenerator = fn(name: &'static str, &[Argument], case_name: &SnakePascal) -> String;

#[derive(Clone)]
enum MethodBody {
    All,
    FieldCopy,
    FieldMemoize,
    Method {
        args: Vec<Argument>,
        match_branch_generator: MatchBranchGenerator,
    },
}

#[derive(Clone)]
struct Method {
    name: &'static str,
    ret: &'static str,
    body: MethodBody,
}

impl Method {
    pub fn append(&self, res: &mut String, cases: &[SnakePascal]) {
        let Self {
            name,
            ret,
            ref body,
        } = *self;

        match *body {
            MethodBody::All => {
                *res += format!("pub fn {name}() -> Vec<Self> {{\n").as_str();
                *res += "vec![\n";
                for SnakePascal { snake: _, pascal } in cases.iter() {
                    *res += format!("Self::{pascal},\n").as_str();
                }
                *res += "]\n";
                *res += "}\n";
            }

            MethodBody::FieldCopy => {
                *res += format!("pub fn {name}(self) -> {ret} {{\n").as_str();
                *res += "match self {\n";
                for SnakePascal { snake, pascal } in cases.iter() {
                    *res += format!("Self::{pascal} => {snake}::{name}(),\n").as_str();
                }
                *res += "}\n";
                *res += "}\n";
            }

            MethodBody::FieldMemoize => {
                *res += "#[allow(non_upper_case_globals, clippy::explicit_auto_deref)]\n";
                *res += format!("pub fn {name}(self) -> &'static {ret} {{\n").as_str();
                *res += "lazy_static! {\n";
                for SnakePascal { snake, pascal } in cases.iter() {
                    *res += format!("static ref {pascal}: {ret} = {snake}::{name}();\n").as_str();
                }
                *res += "};\n";
                *res += "match self {\n";
                for SnakePascal { snake: _, pascal } in cases {
                    *res += format!("Self::{pascal} => &*{pascal},\n").as_str();
                }
                *res += "}\n";
                *res += "}\n";
            }

            MethodBody::Method {
                ref args,
                match_branch_generator,
            } => {
                let args_definition = format_args_definition(args);

                *res += format!("pub fn {name}(self, {args_definition}) -> {ret} {{\n").as_str();
                *res += "match self {\n";
                for case in cases {
                    *res += format!("Self::{} => ", case.pascal).as_str();
                    *res += match_branch_generator(name, &args, case).as_str();
                }
                *res += "}\n";
                *res += "}\n";
            }
        }
    }
}

fn file_names(dir: &Path) -> io::Result<Vec<SnakePascal>> {
    let mut res = Vec::<SnakePascal>::default();

    for file in fs::read_dir(dir)? {
        let file = file?;
        if !file.file_type()?.is_file() {
            continue;
        }

        let snake = String::from(
            file.file_name()
                .to_str()
                .unwrap()
                .strip_suffix(".rs")
                .unwrap(),
        );
        let pascal = snake.to_case(Case::Pascal);

        res.push(SnakePascal { snake, pascal });
    }

    Ok(res)
}

fn make_enum_file(
    input_dir: &Path,
    enum_name: &str,
    methods: impl IntoIterator<Item = Method>,
) -> io::Result<String> {
    let mut res = String::default();

    {
        res += "use lazy_static::lazy_static;\n";
        res += "use crate::card_uses::*;\n";
    }

    let cases = file_names(Path::new("src/").join(input_dir).as_path())?;

    {
        let input_dir = input_dir.to_str().unwrap();
        for SnakePascal { snake, pascal: _ } in cases.iter() {
            let path = format!("{input_dir}{snake}.rs");

            cargo_emit::rerun_if_changed!(format!("src/{path}"));
            res += format!("#[path = \"{path}\"] mod {snake};\n").as_str();
        }
    }
    {
        res += "#[derive(Clone, Copy, PartialEq, Eq)]\n";
        res += format!("pub enum {enum_name} {{\n").as_str();
        for SnakePascal { snake: _, pascal } in cases.iter() {
            res += format!("    {pascal},\n").as_str();
        }
        res += "}\n";
    }
    {
        res += format!("impl {enum_name} {{\n").as_str();
        {
            for method in methods {
                method.append(&mut res, &cases);
            }
        }
        res += "}\n";
    }

    Ok(res)
}

fn generate_transparent_match_branch(
    name: &'static str,
    args: &[Argument],
    SnakePascal { snake, .. }: &SnakePascal,
) -> String {
    let mut res = format!("{snake}::{name}(");

    for Argument { name, .. } in args.iter().copied() {
        res += format!("{name},").as_str();
    }

    res += "),\n";
    res
}

fn generate_act_handle_event_match_branch(
    name: &'static str,
    args: &[Argument],
    SnakePascal { snake, .. }: &SnakePascal,
) -> String {
    let signed_event = args.last().unwrap().name;
    let args_without_signed_event = format_args_call(&args[..args.len() - 1]);
    let args = format_args_call(args);
    format!("{{\nif let &mut Event::Use {{ act_id: _act_id, ref mut use_way }} = &mut {signed_event}.value {{\nif _act_id == id {{\nmatch use_way {{\nUseWay::OnCharacter(chr_id) => *chr_id = {snake}::use_on_chr({args_without_signed_event}, *chr_id)?,\nUseWay::OnField => {snake}::use_on_field({args_without_signed_event})?,\n}}\nreturn Ok(signed_event);\n}}\n}}\n{snake}::{name}({args})\n}}")
}

fn main() -> io::Result<()> {
    cargo_emit::rerun_if_changed!("build.rs");

    fn handle_event_chr() -> Method {
        Method {
            name: "handle_event",
            ret: "EventResult",
            body: MethodBody::Method {
                args: vec![
                    Argument {
                        name: "game",
                        type_: "&mut Game",
                        mut_: false,
                    },
                    Argument {
                        name: "id",
                        type_: "CharacterID",
                        mut_: false,
                    },
                    Argument {
                        name: "signed_event",
                        type_: "SignedEvent",
                        mut_: false,
                    },
                ],
                match_branch_generator: generate_transparent_match_branch,
            },
        }
    }

    fn handle_event_act() -> Method {
        Method {
            name: "handle_event",
            ret: "EventResult",
            body: MethodBody::Method {
                args: vec![
                    Argument {
                        name: "game",
                        type_: "&mut Game",
                        mut_: false,
                    },
                    Argument {
                        name: "id",
                        type_: "ActiveID",
                        mut_: false,
                    },
                    Argument {
                        name: "signed_event",
                        type_: "SignedEvent",
                        mut_: true,
                    },
                ],
                match_branch_generator: generate_act_handle_event_match_branch,
            },
        }
    }

    fn handle_check(id_argument_type: &'static str) -> Method {
        Method {
            name: "handle_check",
            ret: "CheckResult",
            body: MethodBody::Method {
                args: vec![
                    Argument {
                        name: "game",
                        type_: "&Game",
                        mut_: false,
                    },
                    Argument {
                        name: "id",
                        type_: id_argument_type,
                        mut_: false,
                    },
                    Argument {
                        name: "signed_check",
                        type_: "SignedCheck",
                        mut_: false,
                    },
                ],
                match_branch_generator: generate_transparent_match_branch,
            },
        }
    }

    let common_methods = vec![
        Method {
            name: "all",
            ret: "Vec<Self>",
            body: MethodBody::All,
        },
        Method {
            name: "name",
            ret: "CustomString",
            body: MethodBody::FieldMemoize,
        },
        Method {
            name: "groups",
            ret: "Groups",
            body: MethodBody::FieldMemoize,
        },
        Method {
            name: "description",
            ret: "CustomString",
            body: MethodBody::FieldMemoize,
        },
    ];

    fs::write(
        Path::new("src/chrs.rs"),
        make_enum_file(
            Path::new("chrs/"),
            "CharacterType",
            common_methods.iter().cloned().chain(
                vec![
                    Method {
                        name: "stats",
                        ret: "Stats",
                        body: MethodBody::FieldCopy,
                    },
                    handle_event_chr(),
                    handle_check("CharacterID"),
                ]
                .into_iter(),
            ),
        )?,
    )?;

    fs::write(
        Path::new("src/acts.rs"),
        make_enum_file(
            Path::new("acts/"),
            "ActiveType",
            common_methods
                .iter()
                .cloned()
                .chain(vec![handle_event_act(), handle_check("ActiveID")].into_iter()),
        )?,
    )?;

    Ok(())
}
