use super::Component;

pub trait MailBox<C : Component> {
    fn handle(self, ctx: &mut C::Context);
}
