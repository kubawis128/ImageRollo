use std::io::Read;
use std::net::TcpListener;
use std::thread;

use gtk::gdk_pixbuf::{InterpType, Pixbuf};
use gtk::gio::MemoryInputStream;
use gtk::glib::{MainContext, PRIORITY_DEFAULT};
use gtk::pango::{AttrFontDesc, AttrList, FontDescription};
use gtk::prelude::*;
use gtk::*;

use serde::{Deserialize, Serialize};

use base64::{Engine as _, engine::general_purpose};

fn main() {
    let application = gtk::Application::new(Some("com.kubawis128.ImageRoll"), Default::default());
    application.connect_activate(build_ui);
    if let Err(e) = gtk::init(){
        println!("Couldn't init GTK stack");
        println!("Error: {}", e);
        println!("Exitting");
        return;
    };
    gtk::set_debug_flags(1);
    application.run();
}

#[derive(Serialize, Deserialize, Debug)]
struct Rating {
    score: String, // String because of in the future we want to have an option to use "Teacher" as a rating
    image: String,
}

fn build_ui(application: &gtk::Application) {
    // Init window and basic outline
    let glade_src: &str = include_str!("glade.ui");
    let builder: Builder = Builder::from_string(glade_src);
    let window: ApplicationWindow = builder.object("window").expect("Coauldn't get window");
    window.fullscreen();
    window.set_application(Some(application));

    let scrolled_window: ScrolledWindow = builder
        .object("scrolled_window")
        .expect("Couldn't get window");

    let (tx, rx) = MainContext::channel(PRIORITY_DEFAULT);

    thread::spawn(move || {
        let listener = TcpListener::bind("127.0.0.1:2137").unwrap();

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut messages = String::new();
            if let Err(e) = stream.read_to_string(&mut messages){
                println!("Couldn't read tcp packet into string [rating] (ignoring frame)");
                println!("Error: {}", e);
                continue
            };

            let ratings: Vec<Rating> = serde_json::from_str(&messages).unwrap();
            if let Err(e) = tx.send(ratings){
                println!("Couldn't send out a signal in GTK stack [rating] (ignoring frame)");
                println!("Error: {}", e);
                continue
            };
        }
    });

    let (tx2, rx2) = MainContext::channel(PRIORITY_DEFAULT);
    thread::spawn(move || {
        let listener = TcpListener::bind("127.0.0.1:2138").unwrap();

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut current_web_cam_image = String::new();
            if let Err(e) = stream.read_to_string(&mut current_web_cam_image){
                println!("Couldn't read tcp packet into string [live feed] (ignoring frame)");
                println!("Error: {}", e);
                continue
            };

            if let Err(e) = tx2.send(current_web_cam_image){
                println!("Couldn't send out a signal in GTK stack [live feed] (ignoring frame)");
                println!("Error: {}", e);
                continue
            };
        }
    });

    window.show_all();

    // clone some varibles because closures don't want to share >:(
    let builder_clone: Builder = builder.clone();
    let builder_clone2: Builder = builder.clone();
    let window1: ApplicationWindow = window.clone();
    let application_clone: Application = application.clone();
    
    let engine = general_purpose::STANDARD_NO_PAD;
    rx.attach(None, move |images| {
        let grid: Grid = builder_clone.object("gride").expect("Couldn't get grid");
        let children: Vec<Widget> = grid.children();
        for child in children {
            grid.remove(&child);
        }
        for (index, data) in images.iter().enumerate() {
            let i :i32 = (index + 1).try_into().unwrap();
            let image: Image = Image::new();
            let label: Label = Label::new(Some(&data.score));

            let decoded: Vec<u8> = engine.decode(&data.image).unwrap();
            let bytes: glib::Bytes = glib::Bytes::from(&decoded);
            let stream: MemoryInputStream = MemoryInputStream::from_bytes(&bytes);
            let mut pixbuf: Pixbuf = Pixbuf::from_stream(&stream, None::<&gio::Cancellable>).unwrap();
            pixbuf = pixbuf.scale_simple(360, 360, InterpType::Bilinear).unwrap();
            image.set_pixbuf(Some(&pixbuf));

            let attributes: AttrList = AttrList::new();
            attributes.insert(AttrFontDesc::new(&FontDescription::from_string(
                "Minecraft Regular 24",
            )));

            label.set_attributes(Some(&attributes));
            label.set_size_request(360, 24);
            label.set_wrap(true);
            label.set_line_wrap(true);
            label.set_width_chars(1);
            label.set_max_width_chars(5);
            label.set_margin_bottom(12);
            label.set_wrap_mode(gtk::pango::WrapMode::Char);
            label.set_text(&format!("Score: {}", &data.score));

            grid.attach(&label, 360 * i, 0, 360, 24);
            grid.attach(&image, 360 * i, 24, 360, 360);
        }
        window1.show_all();
        Continue(true)
    });

    let engine = general_purpose::STANDARD_NO_PAD;
    rx2.attach(None, move |images| {
        let web_cam: Image = builder_clone2
            .object("currentWebCamImg")
            .expect("Couldn't get currentWebCamImg");
        let decoded = engine.decode(&images).unwrap();
        let bytes = glib::Bytes::from(&decoded);
        let stream = MemoryInputStream::from_bytes(&bytes);
        let mut pixbuf = Pixbuf::from_stream(&stream, None::<&gio::Cancellable>).unwrap();
        pixbuf = pixbuf.scale_simple(480, 480, InterpType::Bilinear).unwrap();
        web_cam.set_pixbuf(Some(&pixbuf));
        Continue(true)
    });

    // Handle keypresses
    scrolled_window
        .clone()
        .connect_key_press_event(move |_, k| {
            let gstring = k.keyval().name().unwrap();
            let val = gstring.as_str();
            match val {
                "q" => application_clone.quit(), // quit
                _ => {}
            }
            Inhibit(false)
        });
}
