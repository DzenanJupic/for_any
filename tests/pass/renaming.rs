use for_any::ForAny;

#[derive(ForAny)]
enum RenameNone {
    A,
}

#[derive(ForAny)]
#[for_any(for_any = "new_for_any")]
enum RenameForAny {
    A,
}

#[derive(ForAny)]
#[for_any(for_any_ref = "new_for_any_ref")]
enum RenameForAnyRef {
    A,
}

#[derive(ForAny)]
#[for_any(for_any_mut = "new_for_any_mut")]
enum RenameForAnyMut {
    A,
}

#[derive(ForAny)]
#[for_any(for_any = "new_for_any")]
#[for_any(for_any_ref = "new_for_any_ref")]
enum RenameForAnyForAnyRef {
    A,
}

#[derive(ForAny)]
#[for_any(for_any = "new_for_any")]
#[for_any(for_any_mut = "new_for_any_mut")]
enum RenameForAnyForAnyMut {
    A,
}

#[derive(ForAny)]
#[for_any(for_any_ref = "new_for_any_ref")]
#[for_any(for_any_mut = "new_for_any_mut")]
enum RenameForAnyRefForAnyMut {
    A,
}

#[derive(ForAny)]
#[for_any(for_any = "new_for_any")]
#[for_any(for_any_ref = "new_for_any_ref")]
#[for_any(for_any_mut = "new_for_any_mut")]
enum RenameForAnyForAnyRefForAnyMut {
    A,
}

fn main() {
    RenameNone::A.for_any(|| {});
    RenameNone::A.for_any_ref(|| {});
    RenameNone::A.for_any_mut(|| {});

    RenameForAny::A.new_for_any(|| {});
    RenameForAny::A.for_any_ref(|| {});
    RenameForAny::A.for_any_mut(|| {});

    RenameForAnyRef::A.for_any(|| {});
    RenameForAnyRef::A.new_for_any_ref(|| {});
    RenameForAnyRef::A.for_any_mut(|| {});

    RenameForAnyMut::A.for_any(|| {});
    RenameForAnyMut::A.for_any_ref(|| {});
    RenameForAnyMut::A.new_for_any_mut(|| {});

    RenameForAnyForAnyRef::A.new_for_any(|| {});
    RenameForAnyForAnyRef::A.new_for_any_ref(|| {});
    RenameForAnyForAnyRef::A.for_any_mut(|| {});

    RenameForAnyForAnyMut::A.new_for_any(|| {});
    RenameForAnyForAnyMut::A.for_any_ref(|| {});
    RenameForAnyForAnyMut::A.new_for_any_mut(|| {});

    RenameForAnyRefForAnyMut::A.for_any(|| {});
    RenameForAnyRefForAnyMut::A.new_for_any_ref(|| {});
    RenameForAnyRefForAnyMut::A.new_for_any_mut(|| {});

    RenameForAnyForAnyRefForAnyMut::A.new_for_any(|| {});
    RenameForAnyForAnyRefForAnyMut::A.new_for_any_ref(|| {});
    RenameForAnyForAnyRefForAnyMut::A.new_for_any_mut(|| {});
}
