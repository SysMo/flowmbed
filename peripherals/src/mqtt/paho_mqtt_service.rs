use paho_mqtt as mqtt;
use futures::executor::block_on; 
use futures::stream::{Stream, StreamExt};
use std::collections::HashMap;
use super::mqtt_service::{MqttServiceOptions, MqttPublisher, StrMessage, MqttSubscriber, MqttService};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::pin::Pin;

pub struct PahoMqttService {
  client: Arc<mqtt::AsyncClient>,
  publisher: Option<mpsc::Sender<StrMessage>>,
  subscribers: Arc<Mutex<HashMap<String, mpsc::Sender<String>>>>,
}

impl PahoMqttService {
  pub fn with<F>(opts: MqttServiceOptions, f: F) where F: FnOnce(&mut dyn MqttService) {
    let mut client = mqtt::CreateOptionsBuilder::new()
      .server_uri(opts.host)
      .client_id(opts.client_id)
      .persistence(None)
      .create_client().unwrap();

      let mut msg_receiver = client.get_stream(25);
      let client_arc = Arc::new(client);

    let mut paho = Self { 
      client: client_arc.clone(),
      publisher: None,
      subscribers: Arc::new(Mutex::new(HashMap::new())),
    };

    let ssl_opts = mqtt::SslOptionsBuilder::new()
    // .trust_store(trust_store)?
    // .key_store(key_store)?
    .finalize();

  let conn_opts = mqtt::ConnectOptionsBuilder::new()
    .ssl_options(ssl_opts)
    .user_name(opts.user)
    .password(opts.password)
    .finalize();


    if let Err(err) = block_on(async {
      println!("Connecting to the MQTT server...");
      client_arc.connect(conn_opts).await?;
      println!("Connected to the MQTT server");

      let subscribers = paho.subscribers.clone();

      std::thread::Builder::new()
        .name("mqtt_receiver".to_owned())
        .spawn(move || {
        block_on(async {
          while let Some(msg_opt) = msg_receiver.next().await {
            if let Some(msg) = msg_opt {
              log::info!("Receiving message {}", msg);
              let topic = msg.topic();              
              let payload = std::str::from_utf8(msg.payload()).unwrap();
              let s = subscribers.lock().unwrap();
              match s.get(topic) {
                Some(sub) => {
                  match sub.send(payload.to_owned()) {
                    Ok(_) => (),
                    Err(_) => {
                      log::warn!("no receiver!")
                    },
                }
                },
                None => (),
              }
              // if (Some(sender) = client_arc.send)
              
            }
          }  
        });
      }).unwrap();
  
    let (publisher_tx, publisher_rx) = mpsc::channel::<StrMessage>();
    paho.publisher = Some(publisher_tx);
    
    std::thread::Builder::new()
      .name("mqtt_sender".to_owned())
      .spawn(move || {
        loop {
          let msg_str = publisher_rx.recv().unwrap();
          log::info!("Publishing message: [{}] {}", msg_str.topic, msg_str.payload);
          let msg = mqtt::Message::new(msg_str.topic, msg_str.payload, mqtt::QOS_0);
          client_arc.publish(msg);
        }
      })?;

      Ok::<(), mqtt::Error>(())
    }) {
      eprintln!("Failed connecting to MQTT!");
      eprintln!("{}", err);
    }
    println!("Returned from connect!");


    f(&mut paho);
  }

}

impl MqttService for PahoMqttService {
  fn publisher(&self, topic: &str) -> MqttPublisher {
      let publisher_tx = self.publisher.as_ref().unwrap().clone();
      MqttPublisher {
        topic: topic.to_owned(),
        publisher_tx: publisher_tx
      }
  }
  
  fn subscriber(&mut self, topic: &str) -> MqttSubscriber {
    let (subscriber_tx, subscriber_rx) = mpsc::channel::<String>();
    self.subscribers.lock().unwrap().insert(topic.to_owned(), subscriber_tx);
    self.client.subscribe(topic, mqtt::QOS_0);
    MqttSubscriber { subscriber_rx }
  }
}



#[cfg(feature = "desktop")]
pub struct PahoAsyncMqttService {
  pub client: mqtt::AsyncClient,
  pub stream: Pin<Box<dyn Stream<Item = Option<StrMessage>>>>
}

impl PahoAsyncMqttService {
  pub async fn new(opts: &MqttServiceOptions, topic: &str) -> anyhow::Result<Self> {
    let mut client = mqtt::CreateOptionsBuilder::new()
      .server_uri(&opts.host)
      .client_id(&opts.client_id)
      .persistence(None)
      .create_client().unwrap();
    
    let ssl_opts = mqtt::SslOptionsBuilder::new()
      // .trust_store(trust_store)?
      // .key_store(key_store)?
      .finalize();
  
    let conn_opts = mqtt::ConnectOptionsBuilder::new()
      .ssl_options(ssl_opts)
      .user_name(&opts.user)
      .password(&opts.password)
      .finalize();
    
    let msg_receiver = client.get_stream(25);

    println!("Connecting to the MQTT server...");
    client.connect(conn_opts).await?;
    println!("Connected to the MQTT server");
    println!("Subscribing to {}", topic);
    client.subscribe(topic, paho_mqtt::QOS_0);
    

    Ok(Self {
      client: client,
      stream: Box::pin(msg_receiver.map(|msg_opt| {
        msg_opt.map(|msg| {
          let topic = msg.topic();              
          let payload = std::str::from_utf8(msg.payload()).unwrap();
          StrMessage { 
            topic: topic.to_owned(), 
            payload: payload.to_owned() 
          }
        })
      }))
    })
  }  
}