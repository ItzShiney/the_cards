use crate::default_formatted::DefaultFormatted;
use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyEventKind;
use crossterm::style::Stylize;
use std::fmt;
use std::fmt::Display;
use std::io::stdout;
use std::io::Write;

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
        let Ok(Event::Key(KeyEvent { code: keycode, kind: KeyEventKind::Press, .. })) = event::read() else { continue; };

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
    }) else { unreachable!() };
    chr
}

pub fn prompt<D: Display, R>(
    is_cancellable: bool,
    displays: impl ExactSizeIterator<Item = D>,
    mut results: impl Iterator<Item = R>,
) -> Option<R> {
    let first_chr = 'a';
    let last_chr = char::from_u32(first_chr as u32 + (displays.len() - 1) as u32).unwrap();
    let chrs = first_chr..=last_chr;

    println!("  ┌─────");
    for (chr, option) in chrs.clone().zip(displays) {
        println!("  │ \x1b[1m{}\x1b[0m {}", chr, option);
    }
    if is_cancellable {
        println!("  │ {}", DefaultFormatted(KeyCode::Esc));
    }

    let KeyCode::Char(picked_chr) = read_keycode(|x| match x {
        KeyCode::Esc => is_cancellable,
        KeyCode::Char(x) => chrs.contains(&x),
        _ => false,
    }) else { return None };

    let picked_idx = picked_chr as usize - first_chr as usize;

    Some(results.nth(picked_idx).unwrap())
}

pub fn prompt_idxs<D: Display>(
    is_cancellable: bool,
    displays: impl ExactSizeIterator<Item = D>,
) -> Option<usize> {
    prompt(is_cancellable, displays, 0..)
}
