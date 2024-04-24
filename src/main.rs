use std::time::Instant;

use account_obj::AccountObject;
use gtk::prelude::{ApplicationExt, ApplicationExtManual};
use gtk::{glib, Application};
const APP_ID: &str = "org.gtk_rs.bench";
fn main() -> glib::ExitCode {
    println!("Before startup");
    bench();
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(|_| {
        println!("After startup");
        bench();
    });

    app.run()
}

fn bench() {
    let start = Instant::now();
    let mut list = Vec::new();
    list.push(AccountObject::new_setter(1, "first".into()));
    for i in 0..2000000 {
        let with_children = AccountObject::new_setter(i * 2, format!("number {}", i * 2).into());
        list.push(with_children);
    }
    println!("Setter took {}ms", start.elapsed().as_millis());
    list.clear();

    let start = Instant::now();
    list.push(AccountObject::new_builder(1, "first".into()));
    for i in 0..2000000 {
        let with_children = AccountObject::new_builder(i * 2, format!("number {}", i * 2).into());
        list.push(with_children);
    }
    println!("Builder took {}ms", start.elapsed().as_millis());
}

mod account_obj {
    mod imp {
        use std::cell::RefCell;

        use gtk::glib::{
            self,
            subclass::{object::ObjectImpl, types::ObjectSubclass},
            Properties,
        };
        use gtk::subclass::prelude::DerivedObjectProperties;

        use gtk::prelude::ObjectExt;

        #[derive(Properties, Default)]
        #[properties(wrapper_type = super::AccountObject)]
        pub struct AccountObject {
            #[property(name = "id", get, set, type = u64, member = id)]
            #[property(name = "name", get, set, type = String, member = name)]
            pub data: RefCell<super::AccountData>,
        }

        #[glib::object_subclass]
        impl ObjectSubclass for AccountObject {
            const NAME: &'static str = "AccountObject";
            type Type = super::AccountObject;
        }

        #[glib::derived_properties]
        impl ObjectImpl for AccountObject {}
    }

    use gtk::glib::{self, Object};

    #[derive(Default)]
    pub struct AccountData {
        pub id: u64,
        pub name: String,
    }

    glib::wrapper! {
        pub struct AccountObject(ObjectSubclass<imp::AccountObject>);
    }

    impl AccountObject {
        pub fn new_builder(id: u64, name: String) -> Self {
            Object::builder()
                .property("id", id)
                .property("name", name)
                .build()
        }

        pub fn new_setter(id: u64, name: String) -> Self {
            let obj: AccountObject = Object::builder().build();
            obj.set_id(id);
            obj.set_name(name);
            obj
        }
    }
}
