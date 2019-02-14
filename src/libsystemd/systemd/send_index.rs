use typename          :: { TypeName                        };
use actix             :: { prelude::*                      };
use serde_derive      :: { Serialize, Deserialize          };
use ekke_io           :: { IpcMessage, ConnID, MessageType };
use slog              :: { info                            };

use crate::Systemd;

#[ derive( Debug, Clone, Serialize, Deserialize, Message, TypeName ) ] #[rtype(result="IpcMessage")]
//
pub struct SendHtmlIndex
{
	pub app_name: String,
	pub conn_id : ConnID,
}

#[ derive( Debug, Clone, Serialize, Deserialize, Message, TypeName ) ]
//
pub struct SendHtmlIndexResponse
{
	pub response: String
}



impl Handler<SendHtmlIndex> for Systemd
{
	type Result = IpcMessage;

	fn handle( &mut self, msg: SendHtmlIndex, _ctx: &mut Context<Self> ) -> Self::Result
	{
		info!( self.log, "Systemd: Received app registration for app: {}", msg.app_name );

		let service = "SendHtmlIndexResponse".to_string();
		let payload = SendHtmlIndexResponse{ response: "<html><body>Systemd</body></html>".to_string() };
		let ms_type = MessageType::Response;
		let conn_id = msg.conn_id;

		IpcMessage::new
		(
			  service
			, payload
			, ms_type
			, conn_id
		)
	}
}

