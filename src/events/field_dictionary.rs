//https://www.elastic.co/guide/en/ecs/current/index.html
// Some of this events are automatically created when you map a SiemLog to a SiemEvent. The object field types are not supported for simplicity in uSIEM.
// If needed join the values by the character "\n" into a single String. Useful for file names.
pub static EVENT_OUTCOME: &'static str = "event.outcome";
/// The action captured by the event. This describes the information in the event. It is more specific than event.category. Examples are group-add, process-started, file-created. The value is normally defined by the implementer.
pub static EVENT_ACTION: &'static str = "event.action";
/// event.category represents the "big buckets" of ECS categories. For example, filtering on event.category:process yields all events relating to process activity. Valudes: authentication, configuration, database, driver, file, host, iam, intrusion_detection, malware, network, package, process, web
pub static EVENT_CATEGORY: &'static str = "event.category";
/// Some event sources use event codes to identify messages unambiguously, regardless of message language or wording adjustments over time. An example of this is the Windows Event ID.
pub static EVENT_CODE: &'static str = "event.code";

pub static USER_NAME: &'static str = "user.name";
pub static USER_DOMAIN: &'static str = "user.domain";
pub static SOURCE_IP: &'static str = "source.ip";
pub static SOURCE_PORT: &'static str = "source.port";
pub static DESTINATION_IP: &'static str = "destination.ip";
pub static DESTINATION_PORT: &'static str = "destination.port";


