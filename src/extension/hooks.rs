use penrose::{
    core::{hooks::ManageHook, State},
    x::{XConn, XConnExt},
    Result, Xid,
};

#[derive(Debug)]
pub struct FocusTag(pub &'static str);
impl<X: XConn> ManageHook<X> for FocusTag {
    fn call(&mut self, _client: Xid, state: &mut State<X>, x: &X) -> Result<()> {
        x.modify_and_refresh(state, move |client_set| {
            client_set.focus_tag(self.0);
        })
    }
}
