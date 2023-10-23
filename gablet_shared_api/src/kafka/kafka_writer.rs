use std::{io::Write, time::Duration};

use kafka::producer::{Producer, Record};

use crate::credentials::Credentials;

pub struct KafkaWriter {
    producer: Producer,
    topic: String,
}

impl KafkaWriter {
    pub fn new(producer: Producer, topic: &str) -> KafkaWriter {
        KafkaWriter {
            producer,
            topic: topic.to_owned(),
        }
    }

    pub fn from_credentials(creds: &Credentials) -> KafkaWriter {
        let kafka_creds = creds.kafka.as_ref().expect("Missing kafka credentials");
        let producer = Producer::from_hosts(kafka_creds.hosts.clone())
            .with_ack_timeout(Duration::from_secs(1))
            .with_required_acks(kafka::producer::RequiredAcks::One)
            .create()
            .expect("Failed to create kafka writer");

        KafkaWriter::new(producer, "logs")
    }
}

impl Write for KafkaWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self.producer.send(&Record::from_value(&self.topic, buf)) {
            Ok(()) => Ok(buf.len()),
            Err(err) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Kafka error: {}", err.to_string()),
            )),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
