use cursive::event;
use cursive::view::Nameable;
use cursive::view::Resizable;
use cursive::views::{Dialog, EditView};
use cursive::Cursive;
use std::cell::RefCell;
use std::process::exit;
use std::sync::Arc;

fn obscure(s: &mut Cursive) {
    s.call_on_name("dialog", |view: &mut Dialog| {
        view.clear_buttons();
        view.add_button("Reveal", reveal);
        view.add_button("Cancel", |s| s.quit());
    });

    s.call_on_name("passwd", |view: &mut EditView| {
        view.set_secret(true);
    });
}

fn reveal(s: &mut Cursive) {
    s.call_on_name("dialog", |view: &mut Dialog| {
        view.clear_buttons();
        view.add_button("Obscure", obscure);
        view.add_button("Cancel", |s| s.quit());
    });

    s.call_on_name("passwd", |view: &mut EditView| {
        view.set_secret(false);
    });
}

fn main() {
    let mut siv = cursive::default();

    let password: Arc<RefCell<Option<String>>> = Arc::new(RefCell::new(None));
    let password_write = password.clone();

    let field = EditView::new()
        .content("")
        .secret()
        .on_submit(move |s, text| {
            password_write.replace(Some(text.to_string()));
            s.quit();
        })
        .with_name("passwd")
        .min_width(30);

    siv.add_layer(Dialog::around(field).title("Cursive").with_name("dialog"));

    obscure(&mut siv);

    siv.add_global_callback(event::Key::Esc, |s| s.quit());

    siv.run();

    match password.replace(None) {
        Some(p) => {
            print!("{}", p);
        }
        None => {
            exit(1);
            // exit 1
        }
    }
}
