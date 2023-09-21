use qrcode_generator::QrCodeEcc;
use futures_util::future::BoxFuture;
use tgbot::{Api, UpdateHandler};
use tgbot::longpoll::LongPoll;
use tgbot::methods::{SendMessage, SendPhoto};
use tgbot::types::{Update, UpdateKind, InputFile};
struct Handler {
    api: Api,
}

impl UpdateHandler for Handler {
    type Future = BoxFuture<'static, ()>;

    fn handle(&self, update: Update) -> Self::Future {
        println!("got an update: {:?}\n", update);
        let api = self.api.clone();
        Box::pin(async move {
            if let UpdateKind::Message(message) = &update.kind {
                if let Some(text) = message.get_text() {
                    let chat_id = message.get_chat_id();
                    let mut method = None::<SendMessage>;
                    let mut method_other = None::<SendPhoto>;
                    let t = text.data.as_str();
                        if t=="/start" { method = Some(SendMessage::new(chat_id, "Hello! Send me some text(or link) to generate a QR code.".to_string()));}
                        else {
                            let p = r"src\file_output.png";
                            let qr = qrcode_generator::to_png_to_file(t, QrCodeEcc::Low, 1024, p);
                            if let Err(e) = qr {
                                let txt=format!("Oops! An {} error occured!", e);
                                method = Some(SendMessage::new(chat_id, txt));
                            }
                            else {
                                let fp = InputFile::path(p);
                                method_other = Some(SendPhoto::new(chat_id,
                                fp.await.unwrap()));
                            }
                        }
                    
                    if method.is_none() {
                        api.execute(method_other.unwrap()).await.unwrap();
                    }
                    else {
                    api.execute(method.unwrap()).await.unwrap();
                    }
                }
                else {
                    let method = SendMessage::new(update.get_chat_id().unwrap(), "Sorry, I can't understand you!".to_string());
                    api.execute(method).await.unwrap();
                }
            }
            else {
                let method = SendMessage::new(update.get_chat_id().unwrap(), "Sorry, I can't understand you!".to_string());
                api.execute(method).await.unwrap();
            } 
        })
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("TG_BOT_TOKEN").expect("TG_BOT_TOKEN is not set");
    let api = Api::new(token).expect("Failed to create API");
    LongPoll::new(api.clone(), Handler { api }).run().await;
}