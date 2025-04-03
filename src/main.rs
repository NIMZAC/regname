use iocraft::prelude::*;
use regex::Regex;
use smol;

#[derive(Clone, Copy, PartialEq)]
enum FocusedField {
    Match,
    Rename,
}

#[derive(Default, Props)]
struct AppProps {
    dir: std::path::PathBuf,
    exit_code: i32,
    out_buffer: String,
    err_buffer: String,
}

#[component]
fn App(props: &mut AppProps, mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let (width, height) = hooks.use_terminal_size();
    let mut system = hooks.use_context_mut::<SystemContext>();

    let mut out_buffer = hooks.use_state(|| String::new());
    let mut err_buffer = hooks.use_state(|| String::new());
    let mut should_exit = hooks.use_state::<Option<i32>, _>(|| None);
    let mut scroll_offset = hooks.use_state(|| 0);

    let mut eprintln = |msg: &str| {
        err_buffer.set(format!("{}{}\n", err_buffer.read().as_str(), msg));
    };

    let items = hooks.use_state(|| {
        let entries = match std::fs::read_dir(&props.dir) {
            Ok(entries) => entries.collect::<Vec<_>>(),
            Err(err) => {
                eprintln(&format!("ERROR: Could not read directory: {}", err));
                should_exit.set(Some(1));
                vec![]
            }
        };

        let mut items = vec![];

        for entry in entries {
            if let Err(err) = entry {
                eprintln(&format!("ERROR: Could not read entry: {}", err));
                should_exit.set(Some(1));
                continue;
            }

            let entry = entry.unwrap();

            let metadata = entry.metadata();
            if let Err(err) = metadata {
                eprintln(&format!("ERROR: Could not read entry metadata: {}", err));
                should_exit.set(Some(1));
                continue;
            }
            let metadata = metadata.unwrap();

            let is_file = metadata.is_file();
            let path = entry.path();

            let filename = path.file_name();
            if let Some(filename) = filename {
                let filename = filename.to_string_lossy().to_string();
                items.push((is_file, filename));
            }
        }

        items.sort_by(|a, b| {
            let a = a.1.to_lowercase();
            let b = b.1.to_lowercase();
            a.cmp(&b)
        });

        items
    });

    let mut match_field = hooks.use_state(|| "(.*)".to_string());
    let mut rename_field = hooks.use_state(|| "$1".to_string());
    let mut focused_field = hooks.use_state(|| FocusedField::Match);

    let item_count = items.read().len() as i32;

    let match_re = Regex::new(&match_field.read().to_string())
        .unwrap_or_else(|_| Regex::new("^$").unwrap());

    let item_elts = items
        .read()
        .iter()
        .map(|(is_file, filename)| {
            let label = if *is_file {
                format!("📄 {}", filename)
            } else {
                format!("📁 {}/", filename)
            };

            element! {
                Text(
                    color: if match_re.is_match(filename) {
                        Color::Green
                    } else {
                        Color::Grey
                    },
                    content: label,
                )
            }
        })
        .collect::<Vec<_>>();

    let renamed_elts = items
        .read()
        .iter()
        .map(|(is_file, filename)| {
            let pattern = rename_field.to_string();

            if match_re.is_match(filename) && !pattern.is_empty() {
                let renamed = match_re.replace_all(filename, &pattern);
                let label = if *is_file {
                    format!("📄 {}", renamed)
                } else {
                    format!("📁 {}/", renamed)
                };

                element! {
                    Text(
                        color: Color::Red,
                        content: label,
                    )
                }
            }
            else {
                let label = if *is_file {
                    format!("📄 {}", filename)
                } else {
                    format!("📁 {}/", filename)
                };

                element! {
                    Text(
                        color: Color::Grey,
                        content: label,
                    )
                }
            }
        })
        .collect::<Vec<_>>();

    hooks.use_terminal_events({
        let dir = props.dir.clone();

        move |event| {
            let mut oprintln = |msg: &str| {
                out_buffer.set(format!("{}{}\n", out_buffer.read().as_str(), msg));
            };
            let mut eprintln = |msg: &str| {
                err_buffer.set(format!("{}{}\n", err_buffer.read().as_str(), msg));
            };

            let match_re = Regex::new(&match_field.read().to_string())
                .unwrap_or_else(|_| Regex::new("^$").unwrap());

            match event {
                TerminalEvent::Key(KeyEvent { code, kind, .. })
                    if { kind != KeyEventKind::Release } =>
                {
                    match code {
                        KeyCode::Up => scroll_offset.set((scroll_offset.get() - 1).max(0)),
                        KeyCode::Down => {
                            scroll_offset.set((scroll_offset.get() + 1).min(item_count - 1))
                        }
                        KeyCode::Tab => match focused_field.get() {
                            FocusedField::Match => {
                                focused_field.set(FocusedField::Rename);
                            }
                            FocusedField::Rename => {
                                focused_field.set(FocusedField::Match);
                            }
                        },
                        KeyCode::Enter => {
                            let pattern = rename_field.to_string();

                            for (_, filename) in items.read().iter() {
                                if match_re.is_match(filename) && !pattern.is_empty() {
                                    let renamed = match_re.replace_all(
                                        filename,
                                        &pattern,
                                    ).to_string();

                                    oprintln(&format!("{} -> {}", filename, renamed));

                                    let old_path = dir.join(filename);
                                    let new_path = dir.join(&renamed);

                                    match std::fs::rename(old_path, new_path) {
                                        Ok(_) => {}
                                        Err(err) => {
                                            eprintln(&format!(
                                                "ERROR: Could not rename file: {}",
                                                err,
                                            ));
                                            should_exit.set(Some(1));
                                            break;
                                        }
                                    }
                                }
                            }

                            should_exit.set(Some(0));
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    });

    if should_exit.get().is_some() {
        props.exit_code = should_exit.get().unwrap();
        props.out_buffer = out_buffer.read().to_string();
        props.err_buffer = err_buffer.read().to_string();
        system.exit();
    }

    element! {
        View(
            width,
            height,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Stretch,
        ) {
            View(
                width: Size::Percent(100f32),
                height: 1,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Stretch,
            ) {
                View(
                    flex_shrink: 1f32,
                ) {
                    Text(content: "Directory: ")
                }
                View(
                    flex_grow: 1f32,
                    background_color: Color::Black,
                    padding_left: 1,
                    padding_right: 1,
                ) {
                    TextInput(value: format!("{}", props.dir.display()))
                }
            }
            View(
                width: Size::Percent(100f32),
                height: height - 2,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Stretch,
                justify_content: JustifyContent::Stretch,
            ) {
                View(
                    flex_grow: 1f32,
                    border_style: BorderStyle::Double,
                    border_color: Color::Blue,
                    padding_left: 1,
                    padding_right: 1,
                    overflow: Overflow::Scroll,
                ) {
                    View(
                        width: Size::Percent(100f32),
                        height: item_count,
                    ) {
                        View(
                            width: Size::Percent(100f32),
                            position: Position::Absolute,
                            top: -scroll_offset.get(),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Start,
                        ) {
                            #(item_elts)
                        }
                    }
                }
                View(
                    flex_grow: 1f32,
                    border_style: BorderStyle::Double,
                    border_color: Color::Blue,
                    padding_left: 1,
                    padding_right: 1,
                    overflow: Overflow::Scroll,
                ) {
                    View(
                        width: Size::Percent(100f32),
                        height: item_count,
                    ) {
                        View(
                            width: Size::Percent(100f32),
                            position: Position::Absolute,
                            top: -scroll_offset.get(),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Start,
                        ) {
                            #(renamed_elts)
                        }
                    }
                }
            }
            View(
                width: Size::Percent(100f32),
                height: 1,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Stretch,
                gap: 1,
            ) {
                View(
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Stretch,
                    flex_grow: 1f32,
                    gap: 1,
                    padding_left: 1,
                    padding_right: 1,
                ) {
                    View(
                        flex_shrink: 1f32,
                    ) {
                        Text(content: "Match:")
                    }
                    View(
                        flex_grow: 1f32,
                        background_color: if focused_field.get() == FocusedField::Match {
                            Color::DarkGrey
                        } else {
                            Color::Black
                        },
                        padding_left: 1,
                        padding_right: 1,
                    ) {
                        TextInput(
                            has_focus: focused_field.get() == FocusedField::Match,
                            value: match_field.to_string(),
                            on_change: move |val| match_field.set(val),
                        )
                    }
                }
                View(
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Stretch,
                    flex_grow: 1f32,
                    gap: 1,
                    padding_left: 1,
                    padding_right: 1,
                ) {
                    View(
                        flex_shrink: 1f32,
                    ) {
                        Text(content: "Rename:")
                    }
                    View(
                        flex_grow: 1f32,
                        background_color: if focused_field.get() == FocusedField::Rename {
                            Color::DarkGrey
                        } else {
                            Color::Black
                        },
                        padding_left: 1,
                        padding_right: 1,
                    ) {
                        TextInput(
                            has_focus: focused_field.get() == FocusedField::Rename,
                            value: rename_field.to_string(),
                            on_change: move |val| rename_field.set(val),
                        )
                    }
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let dir = args
        .get(1)
        .map(|s| match std::path::PathBuf::from(s).canonicalize() {
            Ok(path) => path,
            Err(err) => {
                eprintln!("ERROR: {}", err);
                std::process::exit(1);
            },
        })
        .or_else(|| match std::env::current_dir() {
            Ok(path) => Some(path),
            Err(err) => {
                eprintln!("ERROR: {}", err);
                std::process::exit(1);
            },
        })
        .unwrap();

    let mut elt = element! {App(dir: dir)};
    smol::block_on(elt.fullscreen()).unwrap();
    print!("{}", elt.props.out_buffer);
    eprint!("{}", elt.props.err_buffer);
    std::process::exit(elt.props.exit_code);
}
