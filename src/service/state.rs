use crate::ServiceAuth;

/* Type states used for building services */
pub struct Unconfigured;
pub struct Unauthenticated;
pub struct OnlyHost;
pub struct OnlyAuth;
pub struct Authenticated;

/* Marker trait each struct can be used with trait bounds e.g S: ServiceState */
pub trait ServiceState {}

impl ServiceState for Unconfigured {}
impl ServiceState for Unauthenticated {}
impl ServiceState for Authenticated {}
impl ServiceState for OnlyHost {}
impl ServiceState for OnlyAuth {}

/* Util trait that can be impl on all `Authenticated`` services to ensure getters exist */
pub trait ConfiguredService {
    fn auth(&self) -> ServiceAuth;
    fn host(&self) -> String;
}
