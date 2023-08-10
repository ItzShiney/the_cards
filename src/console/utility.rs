use {
    crate::{
        custom_string::CustomString,
        default_formatted::DefaultFormatted,
        game_input::PromptArgs,
    },
    crossterm::{
        event,
        event::{
            Event,
            KeyCode,
            KeyEvent,
            KeyEventKind,
        },
        style::Stylize,
    },
    std::{
        fmt,
        fmt::Display,
        io::{
            stdout,
            Write,
        },
    },
};

impl Display for DefaultFormatted<KeyCode> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            KeyCode::Enter => "⤶".bold().fmt(f),
            KeyCode::Left => "←".bold().yellow().fmt(f),
            KeyCode::Right => "→".bold().yellow().fmt(f),
            KeyCode::Up => "↑".bold().yellow().fmt(f),
            KeyCode::Down => "↓".bold().yellow().fmt(f),
            KeyCode::Char(chr) => chr.bold().fmt(f),
            KeyCode::Esc => "esc".bold().red().fmt(f),
            _ => unimplemented!(),
        }
    }
}

pub fn read_keycode(pred: impl Fn(KeyCode) -> bool) -> KeyCode {
    print!("  └─> ");
    stdout().flush().unwrap();

    loop {
        let Ok(Event::Key(KeyEvent {
            code: keycode,
            kind: KeyEventKind::Press,
            ..
        })) = event::read()
        else {
            continue;
        };

        if pred(keycode) {
            println!("{}", DefaultFormatted(keycode));
            return keycode;
        }
    }
}

pub fn read_chr(pred: impl Fn(char) -> bool) -> char {
    let KeyCode::Char(chr) = read_keycode(|keycode| match keycode {
        KeyCode::Char(chr) => pred(chr),
        _ => false,
    }) else {
        unreachable!()
    };
    chr
}

pub type IsEnabled = bool;

pub fn prompt(
    args: PromptArgs,
    options: impl Iterator<Item = (IsEnabled, CustomString)>,
) -> Option<usize> {
    fn to_idx(chr: char) -> usize {
        chr as usize - 'a' as usize
    }

    let mut chrs = vec![];
    let mut next_chr = 'a';
    let mut output = String::default();

    output += format!("  ┌─< {}\n", args.str).as_str();
    for (chr, (is_enabled, option)) in ('a'..).clone().zip(options) {
        let key = DefaultFormatted(KeyCode::Char(chr));

        if is_enabled {
            output += format!("  │ {} {}\n", key, option).as_str();
            chrs.push(next_chr);
        } else {
            output += format!(
                "  │ {} {}\n",
                key.to_string().black(),
                option.to_string().black()
            )
            .as_str();
        }

        next_chr = (next_chr..).nth(1).unwrap();
    }
    if args.is_cancellable {
        output += format!("  │ {}\n", DefaultFormatted(KeyCode::Esc)).as_str();
    }

    if args.autochoose_single_option {
        match chrs.len() {
            0 => return None,
            1 => return Some(to_idx(chrs[0])),
            _ => {}
        }
    }

    print!("{}", output);

    let KeyCode::Char(picked_chr) = read_keycode(|x| match x {
        KeyCode::Esc => args.is_cancellable,
        KeyCode::Char(x) => chrs.contains(&x),
        _ => false,
    }) else {
        return None;
    };

    Some(to_idx(picked_chr))
}
