

use rstest::*;
use super::*;
use super::super::Container;


struct Dummy (Dummy2);
struct Dummy2 (i32);

impl Injectable for Dummy {
    type Deps = Dummy2;

    fn inject(deps: Self::Deps) -> Self {
        Self(deps)
    }
}

impl Injectable for Dummy2 {
    type Deps = ();

    fn inject(_: Self::Deps) -> Self {
        Self(10)
    }
}


#[rstest]
fn it_resolves_injectable_service_through_container() {
    // 💥 Using the REAL container (not mocking anything)
    let container = Container::new();

    let svc = container.resolve::<Dummy>();

    // 💾 Compile-time type checks
    let _: Dummy = svc;
    let _: Dummy2 = svc.0;

    // Runtime type info (extra debug, optional)
    assert_eq!(std::any::type_name::<Dummy>(), std::any::type_name_of_val(&svc));
    assert_eq!(std::any::type_name::<Dummy2>(), std::any::type_name_of_val(&svc.0));

    // Asserts: value is correct
    assert_eq!(svc.0.0, 10, "Dummy2 inner value should be 10");
}


#[rstest]
fn it_should_have_typesafe_inject_params() {
    let dummy2: Dummy2 = Dummy2(10);
    let dummy: Dummy = Dummy::inject(dummy2);

    assert_eq!(dummy.0.0, 10, "Dummy2 inner value should be 10");
}



injectable!(() => NoDepNoField {});
injectable!(() => NoDepWithField { a: i32 = 5});

injectable!((d: Dummy2) => OneDepNoField {});

injectable!((d: Dummy2) => OneDepWithField { a: i32 = 5} );

injectable!((a: Dummy2, b: Dummy2) => MultiDepWithField { x: i32 = 5});


#[rstest]
fn it_should_create_service_with_macro() {


    // 0 dependency – no field
    let _ = NoDepNoField::inject(());

    // 0 dependency – with field
    let s1 = NoDepWithField::inject(());
    assert_eq!(s1.a, 5);

    // 1 dependency – no field
    let s2 = OneDepNoField::inject(Dummy2(10));
    assert_eq!(s2.d.0, 10);

    // 1 dependency – with field
    let s3 = OneDepWithField::inject(Dummy2(20));
    assert_eq!(s3.a, 5);
    assert_eq!(s3.d.0, 20);

    // >1 dependency – with field
    let s4 = MultiDepWithField::inject((Dummy2(7), Dummy2(8)));
    assert_eq!(s4.x, 5);
    assert_eq!(s4.a.0, 7);
    assert_eq!(s4.b.0, 8);
}