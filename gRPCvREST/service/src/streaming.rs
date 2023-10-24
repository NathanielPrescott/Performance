use actix::{Actor, ActorContext, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

#[derive(Debug, Clone, Default)]
struct MessageWebSocket;

async fn ws_message(request: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(MessageWebSocket, &request, stream)
}

impl Actor for MessageWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MessageWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(msg) = msg {
            match msg {
                ws::Message::Text(text) => ctx.text(text),
                ws::Message::Binary(bin) => ctx.binary(bin),
                ws::Message::Ping(bytes) => ctx.pong(&bytes),
                ws::Message::Close(reason) => {
                    ctx.close(reason);
                    ctx.stop();
                }
                _ => (),
            }
        } else {
            ctx.stop();
        }
    }
}
