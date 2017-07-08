extern crate gtk;
extern crate rand;
use gtk::traits::*;
use rand::*;

fn main() {
    if gtk::init().is_err() {
        println!("Error loading gtk");
        return;
    }

    let window: gtk::Window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("Random Isaac");
    window.set_default_size(250, 150);
    window.connect_delete_event(|_,_| {
        gtk::main_quit();
        gtk::Inhibit(false)
    });
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);

    let widget_box: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let rand_button: gtk::Button = gtk::Button::new_with_label("New Run!");
    let character: gtk::Label = gtk::Label::new("");
    let mode: gtk::Label = gtk::Label::new("");
    let end: gtk::Label = gtk::Label::new("");
    let br: gtk::Label = gtk::Label::new("");
    let hush: gtk::Label = gtk::Label::new("");
    let del: gtk::Label = gtk::Label::new("");
    let mega: gtk::Label = gtk::Label::new("");

    let char_list: Vec<&str> = vec!["Isaac", "Maggie", "Cain", "Judas", "???", "Eve", "Samson", "Azazel", "Lazarus", "Eden", "The Lost", "Lilith", "The Keeper", "Apollyon"];
    let mode_list: Vec<&str> = vec!["Normal", "Hard", "Greed", "Greedier"];
    let end_list: Vec<&str> = vec!["Sheol", "Cathedral", "The Chest", "The Dark Room"];

    let char_clone = character.clone();
    let char_list_clone = char_list.clone();
    let mode_clone = mode.clone();
    let mode_list_clone = mode_list.clone();
    let end_list_clone = end_list.clone();
    let end_clone = end.clone();
    let br_clone = br.clone();
    let hush_clone = hush.clone();
    let del_clone = del.clone();
    let mega_clone = mega.clone();

    rand_button.connect_clicked(move |_| {
        let mut rng = rand::thread_rng();
        let char_num: i32 = rng.gen_range(0, 14);
        char_clone.set_label(&*format!("Character: {}", char_list_clone[char_num as usize]));

        let mode_num: i32 = rng.gen_range(0, 4);
        mode_clone.set_label(&*format!("Mode: {}", mode_list_clone[mode_num as usize]));

        let end_num: i32 = rng.gen_range(0, 4);
        end_clone.set_label(&*format!("End: {}", end_list_clone[end_num as usize]));
        
        if mode_clone.get_label().unwrap().trim() == &*"Mode: Greed" || mode_clone.get_label().unwrap().trim() == &*"Mode: Greedier" {
            end_clone.set_label(&*format!("{}", "N/A"));
        }

        if rng.gen() {
            br_clone.set_label("Boss Rush: Yes");
        } else {
            br_clone.set_label("Boss Rush: No");
        }

        if rng.gen() {
            hush_clone.set_label("Hush: Yes");
        } else {
            hush_clone.set_label("Hush: No");
        }

        if rng.gen() {
            del_clone.set_label("Delirium: Yes");
            mega_clone.set_label("Mega Satan: No");
        } else {
            del_clone.set_label("Delirium: No");
            mega_clone.set_label("Mega Satan: Yes");
        }
    });

    widget_box.add(&rand_button);
    widget_box.add(&character);
    widget_box.add(&mode);
    widget_box.add(&end);
    widget_box.add(&br);
    widget_box.add(&hush);
    widget_box.add(&del);
    widget_box.add(&mega);

    window.add(&widget_box);

    window.show_all();

    gtk::main();
}
