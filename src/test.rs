#[cfg(test)]
mod tests {

    struct InnerString {
        foo: String,
    }

    struct InnerStringRef<'a> {
        foo: &'a String,
        bar: &'a String,
    }

    struct InnerStringMutRef<'a> {
        foo: &'a mut String,
        bar: &'a mut String,
    }

    struct InnerStringMutRef2<'a> {
        foo: &'a mut String,
        bar: &'a mut String,
    }
    impl InnerStringMutRef2<'_> {
        pub fn nop(&self) {}
    }

    #[test]
    fn always_pass() {
        assert!(true);
    }

    #[test]
    fn contains_testing() {

        struct Bar {
            name: String,
        }

        struct Foo {
            name: String,
            bar: Bar,
        }

        impl Foo {
            // '&' comes after '.' according to rust operator precedence
            fn get_name(&self) -> &String { &self.name }
            fn set_name(&mut self, name: String) -> () { self.name = name }
            fn get_bar(&self) -> &Bar { &self.bar }
            fn set_bar(&mut self, bar: Bar) -> () { self.bar = bar }
        }

        let bar1 = Bar{name: String::from("")};
        let foo1 = Foo {
            name: String::from(""),
            bar: bar1,
        };

        // so far so good

        let foo2 = Foo {
            name: String::from(""),
            // cannot use bar1
            // bar: bar1,
            bar: Bar{name: String::from("")},
        };
        
    }

    #[test]
    fn rust_ownership() {
        // going through chapter 4

        // all data stored on the stack must have a known, fixed size
        // data with an unknown size or a size that might change must be stored on the heap

        // In rust, the String type is mutable and the data is stored on the heap

        // this works because "let y = x" makes a copy of 5
        let mut x = 5;
        let y = x;
        assert!(x == 5);
        assert!(y == 5);
        let x = 6;
        assert!(x == 6);
        assert!(y == 5);

        // no copy; value moved
        // sort of like a shallow copy except previous value no longer valid
        let s1 = String::from("");
        let s2 = s1;
        assert!(s2.is_empty());
        // does not work -- "value borrowed here after move"
        // assert!(s1.is_empty());

        // Rust will never automatically make "deep" copies of your data
        // so any "automatic" copying can be assumed to be inexpensive

        // Cloning to get a deep copy
        let s1 = String::from("");
        let s2 = s1.clone();
        assert!(s1.is_empty());
        assert!(s2.is_empty());

        // the reason this next example works is because integers have a known
        // size at compile time, so they are stored entirely on the stack
        let s1 = 5;
        let s2 = s1;
        assert!(s1 == 5);
        assert!(s2 == 5);

        {
            #[derive(Debug)]
            struct Foo;

            impl std::fmt::Display for Foo {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "")
                }
            }

            let x = Foo;
            let y = x;
            assert!(!format!("y={}", y).is_empty());
            // Error "value borrowed here after move"
            // assert!(!format!("x={}", x).is_empty());
        }

        {
            // 'x' can be used below because Foo has the Copy trait
            // note: Clone is a "supertrait" of Copy, so must be included
            // ==> possible to implement Clone only (e.g. String)

            // Copy vs Clone
            // Copy implicit, Clone explicit
            // Behavior of Copy is _not_ overloadable (it is always a simple bit-wise copy)
            // The compiler will tell you whether you are allowed to implement Copy

            // note:  Generally, if your type _can_ implement Copy, it should
            //        But if you think there might be a time when you would add
            //        non-Copy elements to your type, it might be prudent to not
            //        implement Copy

            // note: Any type which has implemented the Drop trait may not implement Copy,
            //       because the whole point is that Copy is stack-only

            // Generally: Any scalar type is Copy
            //            Anything which requires [heap] allocation is not Copy
            //            Anything which is a resource is not Copy
            
            #[derive(Debug, Copy, Clone)]
            struct Foo;

            impl std::fmt::Display for Foo {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "")
                }
            }

            let x = Foo;
            let y = x;
            assert!(!format!("y={}", y).is_empty());
            // No Error!
            assert!(!format!("x={}", x).is_empty());
        }

        {
            // functions behave similar to variable assignments

            let x = String::from("");

            fn do_stuff(s: String) {}

            do_stuff(x);
            // Error: value borrowed after move
            // assert!(x.is_empty());

            let x = String::from("");

            fn do_stuff_ref(s: &String) {}

            do_stuff_ref(&x);
            assert!(x.is_empty()); // works because it was a reference only

            #[derive(Debug, Copy, Clone)]
            struct Foo;
            impl std::fmt::Display for Foo {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "")
                }
            }

            let x = Foo;

            fn do_stuff_x(x: Foo) {}

            do_stuff_x(x);
            assert!(format!("{}", x).is_empty()); // works because x is Copy

            // in summary, do_stuff() took ownership and do_stuff_x() made a copy
            // do_stuff_ref() was a reference only

            // functions can both take ownership and give back:
            fn take_and_give_back(s: String) -> String {
                s
            }

            let s1 = String::from("");
            assert!(s1.is_empty());

            let s2 = take_and_give_back(s1);

            // now s1 is out of scope but s2 is
            assert!(s2.is_empty());

        }

        
    }

    #[test]
    fn nested1() {
        let s1 = String::from("");
        let _unused1 = InnerString{foo: s1};

        // cannot do this - value used after move
        // let _unused2 = InnerString{foo: s1};

        let s2a = String::from("");
        let s2b = String::from("");
        // this works
        let _unused2 = InnerStringRef{foo: &s2a, bar: &s2b};
        // also works to put out 2 references
        let _unused2 = InnerStringRef{foo: &s2a, bar: &s2a};
        // doesn't work (cannot use str instead of &String
        // let _unused2 = super::InnerStringRef{foo: &s2a, bar: ""};

        let mut s3a = String::from("");
        let mut s3b = String::from("");
        // works
        let _unused3 = InnerStringMutRef{foo: &mut s3a, bar: &mut s3b};
        // does not work
        // let _unused3 = super::InnerStringMutRef{foo: &mut s3a, bar: &mut s3a};
        // works (because shadowing)
        let _unused3 = InnerStringMutRef{foo: &mut s3a, bar: &mut s3b};
        // not sure why this works (compiler optimization perhaps?)
        let _unused3b = InnerStringMutRef{foo: &mut s3a, bar: &mut s3b};

        let mut s4a = String::from("");
        let mut s4b = String::from("");
        let _unused4 = InnerStringMutRef2{foo: &mut s4a, bar: &mut s4b};
        let unused4b = InnerStringMutRef2{foo: &mut s4a, bar: &mut s4b};
        // works
        unused4b.nop();
        // does not work
        //_unused4.nop();

        let mut s5a = String::from("");
        let mut s5b = String::from("");
        let foo5 = InnerStringMutRef2{foo: &mut s5a, bar: &mut s5b};
        {
            let foo5b = InnerStringMutRef2{foo: &mut s5a, bar: &mut s5b};
            // works
            foo5b.nop();
        }
        // does not work
        // foo5.nop();

        // this works
        let mut s6a = String::from("");
        let mut s6b = String::from("");
        {
            let foo5 = InnerStringMutRef2{foo: &mut s6a, bar: &mut s6b};
            foo5.nop();
        }
        {
            let foo5b = InnerStringMutRef2{foo: &mut s6a, bar: &mut s6b};
            foo5b.nop();
        }
    }
}
