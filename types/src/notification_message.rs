///! Helpers for NotificationMessage types
use date_time::DateTime;
use encoding::DecodingLimits;
use extension_object::ExtensionObject;
use node_ids::ObjectId;
use service_types::{NotificationMessage, MonitoredItemNotification, DataChangeNotification};

impl NotificationMessage {
    pub fn data_change(sequence_number: u32, publish_time: DateTime, monitored_items: Vec<MonitoredItemNotification>) -> NotificationMessage {
        let data_change_notification = DataChangeNotification {
            monitored_items: Some(monitored_items),
            diagnostic_infos: None,
        };

        trace!("data change notification = {:?}", data_change_notification);

        // Serialize to extension object
        let notification_data = ExtensionObject::from_encodable(ObjectId::DataChangeNotification_Encoding_DefaultBinary, &data_change_notification);
        NotificationMessage {
            sequence_number,
            publish_time,
            notification_data: Some(vec![notification_data]),
        }
    }

    pub fn keep_alive(sequence_number: u32, publish_time: DateTime) -> NotificationMessage {
        NotificationMessage {
            sequence_number,
            publish_time,
            notification_data: None,
        }
    }

    pub fn data_change_notifications(&self, decoding_limits: &DecodingLimits) -> Vec<DataChangeNotification> {
        let mut result = Vec::with_capacity(10);
        if let Some(ref notification_data) = self.notification_data {
            // Dump out the contents
            for n in notification_data {
                if n.node_id != ObjectId::DataChangeNotification_Encoding_DefaultBinary.into() {
                    continue;
                }
                if let Ok(notification) = n.decode_inner::<DataChangeNotification>(decoding_limits) {
                    result.push(notification);
                }
            }
        }
        result
    }
}
