use crate::prelude::SiemResult;

/// Receives request for the execution of commands
pub trait SiemService<Req> {
    type Response;

    fn call(&self, req : Req) -> SiemResult<Self::Response>;
}

#[test]
fn should_implement_servie() {
    struct MyService;
    impl SiemService<u8> for MyService{
        type Response = u64;
        fn call(&self, req : u8) -> SiemResult<Self::Response> {
            Ok(0)
        }
    }
}