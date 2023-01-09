//TODO: use more weak references, so that memory leaks are avoided, possibly have only strong references be in program wide collection

#[macro_export]
macro_rules! bidirectional_one_to_many {
    (
        $one_struct:ident, 
        $one_struct_self_ref_field:ident,
        $one_struct_many_field:ident, 
        $add_many_to_one_func:ident,
        $remove_many_from_one_func:ident,
        $many_struct:ident, 
        $many_struct_self_ref_field:ident,
        $many_struct_one_field:ident,
        $set_one_for_many_func:ident
    ) => {
        impl $one_struct {
            pub fn $add_many_to_one_func(&mut self, many_ref: Rc<RefCell<$many_struct>>) {
                self.$one_struct_many_field.push(many_ref.clone());
                let mut many = many_ref.borrow_mut();
                if let Some(o) = self.$one_struct_self_ref_field.clone().upgrade() {
                    many.$many_struct_one_field = Some(o);
                } else {
                    panic!("Getting RC self ref from weak failed - very weird");
                }
            }

            pub fn $remove_many_from_one_func(&mut self, many_ref: Rc<RefCell<$many_struct>>) {
                self.$one_struct_many_field.retain(
                    |x| !(Rc::ptr_eq(x, &many_ref))
                );
                let mut many = many_ref.borrow_mut();
                many.$many_struct_one_field = None;
            }
        }

        impl $many_struct {
            pub fn $set_one_for_many_func(&mut self, one_ref: Option<Rc<RefCell<$one_struct>>>) {
                if let Some(o) = self.$many_struct_one_field.as_mut() {
                    let mut one = o.borrow_mut();
                    if let Some(m) = self.$many_struct_self_ref_field.clone().upgrade() {
                        one.$one_struct_many_field.retain(
                            |x| !(Rc::ptr_eq(x, &m))
                        );
                    } else {
                        panic!("Getting RC self ref from weak failed - very weird");
                    }
                }

                if let Some(o) = one_ref {
                    let mut one = o.borrow_mut();
                    if let Some(m) = self.$many_struct_self_ref_field.clone().upgrade() {
                        one.$one_struct_many_field.push(m);
                    } else {
                        panic!("Getting RC self ref from weak failed - very weird");
                    }
                    self.$many_struct_one_field = Some(o.clone());
                } else {
                    self.$many_struct_one_field = None;
                }
            }
        }
    };
}