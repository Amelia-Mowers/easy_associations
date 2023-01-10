#[macro_export]
macro_rules! bidirectional_one_to_many {
    (
        $one_struct:ident, 
        $one_struct_many_field:ident, 
        $add_many_to_one_func:ident,
        $remove_many_from_one_func:ident,
        $many_struct:ident, 
        $many_struct_one_field:ident,
        $set_one_for_many_func:ident
    ) => {
        impl $one_struct {
            pub fn $add_many_to_one_func(&mut self, many_ref: Rc<RefCell<$many_struct>>) {
                self.$one_struct_many_field.push(many_ref.clone());
                let mut many = many_ref.borrow_mut();
                many.$many_struct_one_field = Some(self.clone_self_ref());
            }

            pub fn $remove_many_from_one_func(&mut self, many_ref: Rc<RefCell<$many_struct>>) {
                self.$one_struct_many_field.retain(|x| x != &many_ref);
                let mut many = many_ref.borrow_mut();
                many.$many_struct_one_field = None;
            }
        }

        impl $many_struct {
            pub fn $set_one_for_many_func(&mut self, one_ref: Option<Rc<RefCell<$one_struct>>>) {
                let s = &self.clone_self_ref();
                if let Some(o) = &self.$many_struct_one_field {
                    o.borrow_mut().$one_struct_many_field.retain(|x| !Rc::ptr_eq(x, s));
                }

                if let Some(o) = one_ref {
                    let mut one = o.borrow_mut();
                    one.$one_struct_many_field.push(self.clone_self_ref());
                    self.$many_struct_one_field = Some(o.clone());
                } else {
                    self.$many_struct_one_field = None;
                }
            }
        }
    };
}