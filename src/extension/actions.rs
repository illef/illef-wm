use penrose::{builtin::actions::key_handler, core::bindings::KeyEventHandler, util, x::XConn};

pub fn spawn_with_args<X>(
    program: &'static str,
    args: &'static [&'static str],
) -> Box<dyn KeyEventHandler<X>>
where
    X: XConn,
{
    key_handler(move |_, _| util::spawn_with_args(program, args))
}
