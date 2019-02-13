//! This is the actual functionality for the ekke framework server. The binary contains just a very basic main function. All functionality is exposed through this library so you could build against it if needed.
//
#![ feature( await_macro, async_await, futures_api, nll, stmt_expr_attributes, never_type ) ]

mod systemd;
mod errors;

pub use systemd::
{
	  Systemd
};



pub mod services
{
	pub use crate::systemd::SendIndex;
}


use crate::services::*;
use ekke_io::{ IpcConnTrack, Dispatcher };

pub(crate) fn service_map( msg: IpcConnTrack, d: &Dispatcher )
{
    match msg.ipc_msg.service.as_ref()
    {
        "SendIndex" => d.deserialize::<SendIndex>( msg ),
        _ =>(),
    }
}
