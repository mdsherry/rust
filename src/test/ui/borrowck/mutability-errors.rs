// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// All the possible mutability error cases.

#![allow(unused)]

type MakeRef = fn() -> &'static (i32,);
type MakePtr = fn() -> *const (i32,);

fn named_ref(x: &(i32,)) {
    *x = (1,); //~ ERROR
    x.0 = 1; //~ ERROR
    &mut *x; //~ ERROR
    &mut x.0; //~ ERROR
}

fn unnamed_ref(f: MakeRef) {
    *f() = (1,); //~ ERROR
    f().0 = 1; //~ ERROR
    &mut *f(); //~ ERROR
    &mut f().0; //~ ERROR
}

unsafe fn named_ptr(x: *const (i32,)) {
    *x = (1,); //~ ERROR
    (*x).0 = 1; //~ ERROR
    &mut *x; //~ ERROR
    &mut (*x).0; //~ ERROR
}

unsafe fn unnamed_ptr(f: MakePtr) {
    *f() = (1,); //~ ERROR
    (*f()).0 = 1; //~ ERROR
    &mut *f(); //~ ERROR
    &mut (*f()).0; //~ ERROR
}

fn fn_ref<F: Fn()>(f: F) -> F { f }

fn ref_closure(mut x: (i32,)) {
    fn_ref(|| {
        x = (1,); //~ ERROR
        x.0 = 1; //~ ERROR
        &mut x; //~ ERROR
        &mut x.0; //~ ERROR
    });
    fn_ref(move || {
        x = (1,); //~ ERROR
        x.0 = 1; //~ ERROR
        &mut x; //~ ERROR
        &mut x.0; //~ ERROR
    });
}

fn imm_local(x: (i32,)) {
    &mut x; //~ ERROR
    &mut x.0; //~ ERROR
}

fn imm_capture(x: (i32,)) {
    || { //~ ERROR
        x = (1,);
        x.0 = 1;
        &mut x;
        &mut x.0;
    };
    move || {
        x = (1,); //~ ERROR
        x.0 = 1; //~ ERROR
        &mut x; //~ ERROR
        &mut x.0; //~ ERROR
    };
}

static X: (i32,) = (0,);

fn imm_static() {
    X = (1,); //~ ERROR
    X.0 = 1; //~ ERROR
    &mut X; //~ ERROR
    &mut X.0; //~ ERROR
}

fn main() {}
