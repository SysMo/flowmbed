use embedded_svc::mqtt::client::{self, QoS, MessageImpl, Message, Connection};
use embedded_svc::utils::mqtt::client::ConnState;
use esp_idf_svc::mqtt::client::{MqttClientConfiguration, EspMqttClient};
use std::collections::HashMap;
use flowmbed_peripherals::mqtt::{MqttServiceOptions, MqttPublisher, StrMessage, MqttSubscriber, MqttService};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use esp_idf_sys::{EspError};

pub struct EmbeddedMqttService {
  client: Arc<Mutex<EspMqttClient<ConnState<MessageImpl, EspError>>>>,
  publisher: Option<mpsc::Sender<StrMessage>>,
  subscribers: Arc<Mutex<HashMap<String, mpsc::Sender<String>>>>,
}

impl EmbeddedMqttService {
  pub fn with<F>(opts: MqttServiceOptions, f: F) where F: FnOnce(&mut dyn MqttService) {
    let config = MqttClientConfiguration {
      client_id: Some(&opts.client_id),
      crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
      username: Some(&opts.user),
      password: Some(&opts.password),      
      // pub task_stack: usize,
      // pub buffer_size: usize,
      // pub out_buffer_size: usize,
      ..Default::default()
    };
    // let mut client = mqtt::CreateOptionsBuilder::new()
    //   .server_uri(opts.host)
    //   .client_id(opts.client_id)
    //   .persistence(None)
    //   .create_client().unwrap();

      // let mut msg_receiver = client.get_stream(25);
      // let client_arc = Arc::new(client);

    log::info!("Connecting to the MQTT server...");

    let (raw_client, mut connection) = 
      EspMqttClient::new_with_conn(opts.host, &config).unwrap();

    let client = Arc::new(Mutex::new(raw_client));
    let subscribers = Arc::new(Mutex::new(HashMap::new()));

    let mut service = Self {
      client: client.clone(),
      publisher: None,
      subscribers: subscribers.clone(),
    };


    std::thread::Builder::new()
      .name("mqtt_receiver".to_owned())
      .spawn(move || {
        loop {
          std::thread::sleep(std::time::Duration::from_millis(100));
          while let Some(msg) = connection.next() {
            match msg {
                Err(e) => log::info!("MQTT Message ERROR: {}", e),
                Ok(event) => match event {
                    // client::Event::BeforeConnect => todo!(),
                    client::Event::Connected(_) => log::info!("Connected to the MQTT server"),
                    // client::Event::Disconnected => todo!(),
                    client::Event::Received(msg) => {
                      let topic = msg.topic().unwrap();
                      let payload = std::str::from_utf8(msg.data()).unwrap();
                      let s = subscribers.lock().unwrap();
                      match s.get(topic) {
                        Some(sub) => {
                          log::info!("Received [{}] {}", topic, payload);
                          match sub.send(payload.to_owned()) {
                            Ok(_) => (),
                            Err(_) => {
                              log::warn!("no receiver!")
                            },
                          }
                        },
                        None => (),
                      }
  },
                    _ => (),
                },
            }
          }
        }
      }).unwrap();

    // if let Err(err) = block_on(async {
    //   println!("Connecting to the MQTT server...");
    //   client_arc.connect(conn_opts).await?;
    //   println!("Connected to the MQTT server");

    //   let subscribers = service.subscribers.clone();

    //   std::thread::Builder::new()
    //     .name("mqtt_receiver".to_owned())
    //     .spawn(move || {
    //     block_on(async {
    //       while let Some(msg_opt) = msg_receiver.next().await {
    //         if let Some(msg) = msg_opt {
    //           log::info!("Receiving message {}", msg);
    //           let topic = msg.topic();              
    //           let payload = std::str::from_utf8(msg.payload()).unwrap();
    //           let s = subscribers.lock().unwrap();
    //           match s.get(topic) {
    //             Some(sub) => {
    //               match sub.send(payload.to_owned()) {
    //                 Ok(_) => (),
    //                 Err(_) => {
    //                   log::warn!("no receiver!")
    //                 },
    //             }
    //             },
    //             None => (),
    //           }
    //           // if (Some(sender) = client_arc.send)
              
    //         }
    //       }  
    //     });
    //   }).unwrap();
  
    let (publisher_tx, publisher_rx) = mpsc::channel::<StrMessage>();
    service.publisher = Some(publisher_tx);
    
    std::thread::Builder::new()
      .name("mqtt_sender".to_owned())
      .spawn(move || {
        loop {
          let msg = publisher_rx.recv().unwrap();
          log::info!("Publishing message: [{}] {}", msg.topic,  msg.payload);
          client.lock().unwrap().publish(
            &msg.topic, QoS::AtMostOnce, false, msg.payload.as_bytes()
          ).unwrap();
        }
      }).unwrap();

    //   Ok::<(), mqtt::Error>(())
    // }) {
    //   eprintln!("Failed connecting to MQTT!");
    //   eprintln!("{}", err);
    // }
    // println!("Returned from connect!");


    f(&mut service);
  }

}

impl MqttService for EmbeddedMqttService {
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
    self.client.lock().unwrap().subscribe(topic, QoS::AtMostOnce).unwrap();
    MqttSubscriber { subscriber_rx }
  }
}

