pub static CONTINUE: u32 = 100;
pub static SWITCHING_PROTOCOLS: u32 = 101;
pub static PROCESSING: u32 = 102;

// 2×× SUCCESS
pub static OK: u32 = 200;
pub static CREATED: u32 = 201;
pub static ACCEPTED: u32 = 202;
pub static NON_AUTHORITATIVE_INFORMATION: u32 = 203;
pub static NO_CONTENT: u32 = 204;
pub static RESET_CONTENT: u32 = 205;
pub static PARTIAL_CONTENT: u32 = 206;
pub static MULTI_STATUS: u32 = 207;
pub static ALREADY_REPORTED: u32 = 208;
pub static IMUSED: u32 = 226;

// 3×× REDIRECTION
pub static MULTIPLE_CHOICES: u32 = 300;
pub static MOVED_PERMANENTLY: u32 = 301;
pub static FOUND: u32 = 302;
pub static SEE_OTHER: u32 = 303;
pub static NOT_MODIFIED: u32 = 304;
pub static USE_PROXY: u32 = 305;
pub static TEMPORARY_REDIRECT: u32 = 307;
pub static PERMANENT_REDIRECT: u32 = 308;

// 4×× CLIENTERROR
pub static BAD_REQUEST: u32 = 400;
pub static UNAUTHORIZED: u32 = 401;
pub static PAYMENT_REQUIRED: u32 = 402;
pub static FORBIDDEN: u32 = 403;
pub static NOT_FOUND: u32 = 404;
pub static METHOD_NOT_ALLOWED: u32 = 405;
pub static NOT_ACCEPTABLE: u32 = 406;
pub static PROXY_AUTHENTICATION_REQUIRED: u32 = 407;
pub static REQUEST_TIMEOUT: u32 = 408;
pub static CONFLICT: u32 = 409;
pub static GONE: u32 = 410;
pub static LENGTH_REQUIRED: u32 = 411;
pub static PRECONDITION_FAILED: u32 = 412;
pub static PAYLOAD_TOO_LARGE: u32 = 413;
pub static REQUEST_URI: u32 = 414;
pub static UNSUPPORTED_MEDIA_TYPE: u32 = 415;
pub static REQUESTED_RANGE_NOT_SATISFIABLE: u32 = 416;
pub static EXPECTATION_FAILED: u32 = 417;
pub static TEAPOT: u32 = 418;
pub static MISDIRECTED_REQUEST: u32 = 421;
pub static UNPROCESSABLE_ENTITY: u32 = 422;
pub static LOCKED: u32 = 423;
pub static FAILED_DEPENDENCY: u32 = 424;
pub static UPGRADE_REQUIRED: u32 = 426;
pub static PRECONDITION_REQUIRED: u32 = 428;
pub static TOO_MANY_REQUESTS: u32 = 429;
pub static REQUEST_HEADER_FIELDS_TOO_LARGE: u32 = 431;
pub static CONNECTION_CLOSED_WITHOUT_RESPONSE: u32 = 444;
pub static UNAVAILABLE_FOR_LEGAL_REASONS: u32 = 451;
pub static CLIENT_CLOSED_REQUEST: u32 = 499;

// 5×× SERVERERROR
pub static INTERNAL_SERVER_ERROR: u32 = 500;
pub static NOT_IMPLEMENTED: u32 = 501;
pub static BAD_GATEWAY: u32 = 502;
pub static SERVICE_UNAVAILABLE: u32 = 503;
pub static GATEWAY_TIMEOUT: u32 = 504;
pub static HTTP_VERSION_NOT_SUPPORTED: u32 = 505;
pub static VARIANT_ALSO_NEGOTIATES: u32 = 506;
pub static INSUFFICIENT_STORAGE: u32 = 507;
pub static LOOP_DETECTED: u32 = 508;
pub static NOT_EXTENDED: u32 = 510;
pub static NETWORK_AUTHENTICATION_REQUIRED: u32 = 511;
pub static NETWORK_CONNECT_TIMEOUT_ERROR: u32 = 599;
