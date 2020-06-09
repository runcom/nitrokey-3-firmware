use heapless::{
    consts::U1,
    spsc::{
        Consumer,
        Producer,
        Queue,
    },
};

use interchange::Responder;

use crate::api::{Request, Reply};
use crate::error::Error;
use crate::types::ClientId;

#[cfg(feature = "clients-1")]
interchange::interchange! {
    TrussedInterchange: (Request, Result<Reply, Error>)
}

#[cfg(feature = "clients-2")]
interchange::interchange! {
    TrussedInterchange: (Request, Result<Reply, Error>, 2, [None, None])
}

#[cfg(feature = "clients-3")]
interchange::interchange! {
    TrussedInterchange: (Request, Result<Reply, Error>, 3, [None, None, None])
}

#[cfg(feature = "clients-4")]
interchange::interchange! {
    TrussedInterchange: (Request, Result<Reply, Error>, 4, [None, None, None, None])
}

#[cfg(feature = "clients-5")]
interchange::interchange! {
    TrussedInterchange: (Request, Result<Reply, Error>, 5, [None, None, None, None, None])
}

// pub use interchange::TrussedInterchange;

// TODO: The request pipe should block if there is an unhandled
// previous request/reply. As a side effect, the service should always
// be able to assume that the reply pipe is "ready".

// PRIOR ART:
// https://xenomai.org/documentation/xenomai-2.4/html/api/group__native__queue.html
// https://doc.micrium.com/display/osiiidoc/Using+Message+Queues

pub struct ServiceEndpoint {
    pub interchange: Responder<TrussedInterchange>,
    // service (trusted) has this, not client (untrusted)
    // used among other things to namespace cryptographic material
    pub client_id: ClientId,
}

// pub type ClientEndpoint = Requester<TrussedInterchange>;

// in testing, this just directly calls service.process()
// in reality, this should rtfm::pend() the interrupt with handler triggering the service
pub trait Syscall {
    fn syscall(&mut self);
}
