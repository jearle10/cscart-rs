use crate::{service::state::*, Resource, ServiceAuth};
use std::marker::PhantomData;

/*
Share logic and utils for constructing service config
Each actual service aliases this Service struct and optionally implements the appropriate methods
*/
pub struct ServiceConfig<T: ServiceState = Unauthenticated> {
    pub(crate) _marker: PhantomData<T>,
    pub(crate) auth: Option<ServiceAuth>,
    pub(crate) resource: Option<Resource>,
    pub(crate) host: Option<String>,
}

impl ServiceConfig<Unauthenticated> {
    pub fn with_resource(resource: Resource) -> ServiceConfig<Unauthenticated> {
        Self {
            _marker: PhantomData,
            auth: None,
            resource: Some(resource),
            host: None,
        }
    }

    pub fn host(self, host: &str) -> ServiceConfig<OnlyHost> {
        ServiceConfig::<OnlyHost> {
            _marker: PhantomData,
            host: Some(host.into()),
            auth: None,
            resource: self.resource,
        }
    }

    pub fn auth(self, auth: ServiceAuth) -> ServiceConfig<OnlyAuth> {
        ServiceConfig::<OnlyAuth> {
            _marker: PhantomData,
            host: None,
            auth: Some(auth),
            resource: self.resource,
        }
    }
}

impl ServiceConfig<OnlyHost> {
    pub fn auth(self, auth: ServiceAuth) -> ServiceConfig<Authenticated> {
        ServiceConfig::<Authenticated> {
            _marker: PhantomData,
            host: self.host,
            auth: Some(auth),
            resource: self.resource,
        }
    }
}

impl ServiceConfig<OnlyAuth> {
    pub fn host(self, host: &str) -> ServiceConfig<Authenticated> {
        ServiceConfig::<Authenticated> {
            _marker: PhantomData,
            host: Some(host.into()),
            auth: self.auth,
            resource: self.resource,
        }
    }
}

/*
Blanket impl of getters for auth info. Any service that aliases the Service struct
also gets these methods
*/
impl ConfiguredService for ServiceConfig<Authenticated> {
    fn auth(&self) -> ServiceAuth {
        self.auth.as_ref().unwrap().clone()
    }

    fn host(&self) -> String {
        self.host.as_ref().unwrap().clone()
    }
}
