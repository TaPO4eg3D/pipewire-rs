# Guidelines for writing bindings

## Struct bindings
Some raw C objects in pipewire require manual memory management, usually by calling a constructor
to create it on the heap, and a destructor to free it and its internal allocations. \
Additionally, they may require another pipewire object to be kept alive for its own lifetime.

For example, to use a `pw_context` in C, you'd call `pw_context_new()` to obtain a `pw_context*` pointer to an owned context. You need to provide a `pw_loop` to the constructor for the context to use, and must ensure the loop remains alive for the entire lifetime of the context. Once you're finished with the context, you call `pw_context_destroy()` to destroy it and free its memory.

For these objects, pipewire-rs uses up to three different kinds of types:

### Non-owning type (e.g. `&Context`)
The non-owning type simply provides high-level bindings for a raw C object by being a transparent
wrapper around it, so that a pointer to the raw type can be casted to a pointer to the wrapper type.
It does not perform any management for creating or destroying these types, as it does not own the raw struct.
It being a transparent wrapper means that we can take a pointer to some raw C type, cast it to a reference to the ref type,
and return it to the user without needing to set up extra indirection or worrying about memory management.

### Exclusivly owning type (e.g. `ContextBox<'l>`)
The owning type with exclusive ownership is a smart pointer to an owned object, similar to `Box<T>`,
and usually has bindings to the constructor function of the raw type as well as a `std::ops::Drop`
implementation that invokes the C destructor function on the raw type. \
For other methods, it defers to the non-owning type by providing an `std::ops::Deref` implementation
with the non-owning type as its target. \
If the object references another pipewire object internally (like `pw_context` references a `pw_loop`),
the owned struct also carries a lifetime to ensure the referenced object lives long enough.

### Shared ownership type (e.g. `ContextRc`)
In some cases, lifetimes aren't flexible enough, for example you can't store two instances of the structs `A` and `B<'a>` together in one struct if the instance of `B` holds a reference/lifetime to the instance of `A`.

In these cases, shared ownership can be used to let one type keep another alive internally, e.g. one or multiple `ContextRc` can hold a `LoopRc` internally and keep the loop alive while at least one
context is still using it.

### Example
Implementing wrappers for `pw_context` as explained above looks roughly like the following example.
See the real implementation of module `pw::context` for more details.

```rust
// Usually generated automatically in sys crate
mod sys {
    #[repr(C)]
    pub struct pw_context {/* ... */}

    extern "C" {
        // Memory management functions
        pub fn pw_context_new(..) -> *mut pw_context;
        pub fn pw_context_destroy(context: *mut pw_context);

        // Example for a method that does something with a `pw_context`, but does not involve ownership
        pub fn pw_context_do_something(pw_context: *mut pw_context, ..);
    }
}

/// This struct is a non-owning struct for [`sys::pw_context`].
///
/// It is marked `#[repr(transparent)]` so that we can cast directly from a pointer to the C struct to a pointer to this struct.
#[repr(transparent)]
pub struct Context(sys::pw_context);

impl Context {
    // Each ref type should implement at least the `as_raw` and `as_raw_ptr` functions,
    // so that users of the bindings can choose to use sys functions, etc. themselves.
    pub fn as_raw(&self) -> &sys::pw_context { .. }
    pub fn as_raw_ptr(&self) -> *mut sys::pw_context { .. }

    // Methods on the C type that do not require ownership are bound here on this type.
    pub fn do_something(&self, ..) { .. /* call pw_context_do_something() */ .. }
}


pub struct ContextBox<'l> {
    // The owning type contains a NonNull pointer to the raw type, which provides better safety to the internal implementation than a raw pointer
    ptr: std::ptr::NonNull<sys::pw_context>,
    // Optionally, this struct may also keep a lifetime for other structs it needs to keep alive during its own lifetime.
    // In the case of pw_context, we keep a lifetime to the loop the context uses.
    loop_: PhantomData<&'l Loop>
}

impl<'l> ContextBox<'l> {
    // Bindings to constructors are on the owning type.
    pub fn new(loop_: &'l Loop, ..) -> ContextBox<'l> { .. /* call pw_context_new */ .. }

    // Box types should provide at least `from_raw` and `into_raw` methods to allow conversion
    // between the owning box struct and a pointer to an owned raw struct.
    //
    // The struct returned by from_raw has an unbound lifetime, since the user is responsible
    // for managing lifetimes themselves.
    pub unsafe fn from_raw(raw: std::ptr::NonNull<sys::pw_context>) -> ContextBox<'l> { .. }
    pub fn into_raw(self) -> std::ptr::NonNull<sys::pw_context> { .. }
}

// The owning struct implements `Deref` with the ref type as its target,
// as it is a smart pointer that should also give access to the methods of the managed type.
impl<'l> std::ops::Deref for ContextBox<'l> {
    type Target = Context;

    fn deref(&self) -> &Self::Target { .. }
}

impl<'l> AsRef<Context> for ContextBox<'l> {
    fn as_ref(&self) -> &Context { .. }
}

// The owning type implements the Drop trait to clean up the raw type automatically.
impl<'l> std::ops::Drop for ContextBox<'l> {
    fn drop(&mut self) { .. /* call pw_context_destroy() */ .. }
}

// Implement ContextRc similar to ContextBox, but as a reference-counted type analogous to std::rc::Rc.
#[derive(Clone)]
pub struct ContextRc {
    inner: Rc<ContextRcInner>,
}

impl ContextRc {
    pub fn new(loop_: LoopRc, ..) -> Self { .. /* call pw_context_new() */ .. }

    pub fn downgrade(&self) -> CoreWeak { .. }
}

// Implement Deref and AsRef, same as for ContextBox
impl std::ops::Deref for ContextRc {
    type Target = Context;

    fn deref(&self) -> &Self::Target { .. }
}

impl AsRef<Context> for ContextRc {
    fn as_ref(&self) -> &Context { .. }
}

struct ContextRcInner {
    // Field order is important here:
    // Rust drops field from top to bottom, so having the context first
    // ensures that the context gets dropped before the loop it is using.
    context: ContextBox<'static>,
    loop_: LoopRc
}

pub struct CoreWeak {
    weak: Weak<CoreRcInner>,
}

impl CoreWeak {
    pub fn upgrade(&self) -> Option<CoreRc> {
        self.weak.upgrade().map(|inner| CoreRc { inner })
    }
}
```

## Enums

Different from Rust, C-style enums are simply named integers and do not offer the same type safety.
When writing bindings for these enums in Rust, do not use Rust `enum`s, instead use a Tuple struct wrapping the raw integer type and create public constants for each enum variant.

This has the following advantages:
- Conversions are zero-cost, instead of requiring big match statements mapping the raw integers to enum variants
  and the other way around
- Unknown variants (such as new additions to the C library) don't need to be handled, they are represented as any other variant
  and are simply missing the associated constant on the wrapper type
- As a rust enum would be marked `#[non_exhaustive]` to allow for future additions anyways, the tuple wrapper type
  "feels" almost the same as an enum, a user can create and match over any known variants like with a real enum

### Example

C Header:
```c
enum foo {
    foo_a, foo_b
}
```

Rust bindings:
```rust
mod sys {
    pub type foo = ::std::os::raw::c_uint;
    pub const foo_a: foo = 0;
    pub const foo_b: foo = 1;
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Foo(sys::foo);

impl Foo {
    pub const A: Self = Self(sys::foo_a);
    pub const B: Self = Self(sys::foo_b);

    pub fn from_raw(raw: sys::foo) -> Self {
        Self(raw)
    }

    pub fn as_raw(&self) -> sys::foo {
        self.0
    }
}

impl std::fmt::Debug for Foo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = format!("Foo::{}", match *self {
            Self::A => "A",
            Self::B => "B",
            _ => "Unknown",
        });
        f.write_str(&name)
    }
}
```
