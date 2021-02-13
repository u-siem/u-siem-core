# Logs and Events

A device generates a lot of logs, a lot of them have little to none value, but the ones that have are classified as events.
All logs need information about the system that generated the log: Product, Vendor, Hostname... All these fields are organized in the *observer* field.

The possible event categories are:

* Firewall: have information about IP connections.
* Intrusion: logs related to IDS/IPS like Suricata, Snort, OSSEC, Wazuh, NGFW... 
* Assessment: The output of vulnerability scanners or policy enforcers. 
* WebProxy: Browser proxy information.
* WebServer: Web application servers, Adaptative Distribution Content or LoadBalancers for HTTP traffic.
* Sandbox: Like an antivirus, a Sandbox retrieves information about a file being malicious or not. Can be used to extract filenames, hashes or other relevant information to update a dataset of known hashes and trigger queries.
* Antivirus: Antivirus related events: virus detected/blocked...
* DLP: Data Loss Prevention
* Partitioned:  Some devices like email gateways generates a large number of logs when an email arrives: Header processing, AV scan, attachment information... In those cases, each log is associated with an action using a trace ID or a transaction ID.
* EDR: Endpoint Detection and Response devices, also EPP.
* Mail: Mail events. Ex: Microsoft Exchange, IronPort, Office 365...
* DNS: Used to detect queries to malicious sites.
* DHCP: logs associating an IP with a MAC address.
* Auth: Authentication related logs. Logins to devices or services.
* Endpoint: Local events related to servers or workstations: OS events, log files cleaned, user/groups modification, firewall rules...
* JSON: JSON formatted events.
* Unknown