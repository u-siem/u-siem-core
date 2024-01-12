//https://www.elastic.co/guide/en/ecs/current/index.html
// Some of this events are automatically created when you map a SiemLog to a SiemEvent. The object field types are not supported for simplicity in uSIEM.
// If needed join the values by the character "\n" into a single String. Useful for file names.
pub static EVENT_OUTCOME: &str = "event.outcome";
/// The action captured by the event. This describes the information in the event. It is more specific than event.category. Examples are group-add, process-started, file-created. The value is normally defined by the implementer.
pub static EVENT_ACTION: &str = "event.action";
/// event.category represents the "big buckets" of ECS categories. For example, filtering on event.category:process yields all events relating to process activity. Valudes: authentication, configuration, database, driver, file, host, iam, intrusion_detection, malware, network, package, process, web
pub static EVENT_CATEGORY: &str = "event.category";
/// Some event sources use event codes to identify messages unambiguously, regardless of message language or wording adjustments over time. An example of this is the Windows Event ID.
pub static EVENT_CODE: &str = "event.code";

pub static USER_NAME: &str = "user.name";
pub static USER_DOMAIN: &str = "user.domain";
pub static SOURCE_IP: &str = "source.ip";
pub static SOURCE_PORT: &str = "source.port";
/// Amount of bytes sent by the local host
pub static SOURCE_BYTES: &str = "source.bytes";
pub static DESTINATION_IP: &str = "destination.ip";
pub static DESTINATION_PORT: &str = "destination.port";

/// Amount of bytes sent by the remote host
pub static DESTINATION_BYTES: &str = "destination.bytes";

pub static NETWORK_TRANSPORT: &str = "network.transport";
pub static NETWORK_PROTOCOL: &str = "network.protocol";
pub static NETWORK_DURATION: &str = "network.duration";

pub static IN_INTERFACE: &str = "observer.ingress.interface";
pub static OUT_INTERFACE: &str = "observer.egress.interface";

pub static OBSERVER_IP: &str = "observer.ip";
pub static OBSERVER_NAME: &str = "observer.name";

pub static URL_FULL: &str = "url.full";
pub static URL_DOMAIN: &str = "url.domain";
pub static URL_PATH: &str = "url.path";
pub static URL_QUERY: &str = "url.query";

pub static HTTP_REQUEST_METHOD: &str = "http.request.method";
pub static HTTP_RESPONSE_MIME_TYPE: &str = "http.response.mime_type";
pub static HTTP_RESPONSE_STATUS_CODE: &str = "http.response.status_code";

pub static RULE_NAME: &str = "rule.name";
pub static RULE_CATEGORY: &str = "rule.category";
pub static RULE_ID: &str = "rule.id";

pub static DNS_OP_CODE: &str = "dns.op_code";
pub static DNS_ANSWER_CLASS: &str = "dns.answer.class";
pub static DNS_ANSWER_NAME: &str = "dns.answer.name";
pub static DNS_ANSWER_TYPE: &str = "dns.answer.type";
pub static DNS_ANSWER_TTL: &str = "dns.answer.ttl";
pub static DNS_ANSWER_DATA: &str = "dns.answer.data";
pub static DNS_QUESTION_CLASS: &str = "dns.question.class";
pub static DNS_QUESTION_NAME: &str = "dns.question.name";
pub static DNS_QUESTION_TYPE: &str = "dns.question.type";
pub static DNS_RESOLVED_IP: &str = "dns.resolved_ip";

pub static DHCP_RECORD_TYPE: &str = "dhcp.type";

pub static TAG_REPROCESS: &str = "reprocess_log";

/// Write Ahead Log ID
pub static WAL_ID: &str = "wal_id";
