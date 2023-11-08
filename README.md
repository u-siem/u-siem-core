# u-siem-core
[![Documentation](https://docs.rs/u-siem/badge.svg)](https://docs.rs/u-siem) ![crates.io](https://img.shields.io/crates/v/u-siem.svg) ![workflow](https://github.com/u-siem/u-siem-core/actions/workflows/rust.yml/badge.svg)

Framework definitions that allow building a custom SIEM. Code you own SIEM like you code your own web page.

## Motivation
I have seen many times updates in rules that break things or inneficient regular expressions that slow down the entire SIEM. 
With the aproach of uSIEM, all changes to the SIEM are traced and can be reversed using a version control system like GIT.
Also you can have a team designing rules with SIGMA in a different repository and join the SIEM code and the rules using
CI/CD tools like Jenkins that can run a integration test to check if the new changes breaks the system and then deploy
the new version.

Some benchmarks (single-thread):
| Log Source        | Events/second     |
|-------------------|-------------------|
|Suricata(JSON)     |261780             |
|OpnSense Firewall  |750127             |

You can see a more updated document here: https://github.com/u-siem/parser-benchmarks


## Architecture

```
                                                        |---------------|                
                                                 |----> | GatheringNode |------------------>|
                                                 |      |---------------|                   |
                                                 |                                          |
|---------|       |------------|       |------------------|       |--------------|          |
|InputNode| ----> | ParsingNode| ----> |  Enrichment Node | ----> | IndexingNode |          |
|---------|       |------------|       |------------------| |     |--------------|          |
                                                            |                               |
                                                            |   |----------|       |--------------|       |----------|           
                                                            --> | RuleNode | ----> | AlertingNode | ----> | SoarNode |
                                                                |----------|       |--------------|       |----------|
```
Note: It has changed quite a lot and is still changing.

## Node Types

### Kernel
The kernel will be executed in a single thread with high priority and will be responsible of scaling the number of nodes of each type if it detects a congestion in a element. It will alse route messages between nodes.

### Commander
This component will accept commands from a user and sent it to be routed by the kernel to specific nodes.

### InputNode
It ingests logs and process them.
We can support elasticsearch type (Like API-REST) or syslog.

### ParsingNode
This node will be the most important and will be able to parse the logs sent to him by the input node.
There will be a special node still being designed to process logs like MySQL or Linux logs that are multiline.

### EnrichmentNode
It adds information about the IP, if its in a blocklist, if its a AmazonWebServices/Azure/GoogleCloud IP, if the IP has never been seen it then it contacts the GatheringNode to find information about that IP. It adds the tag "never_seen_ip" in that cases. It uses datasets to access information in a non-blocking form. See https://1drv.ms/p/s!AvHbai5ZA14wjV9J4rbBlSWyIw0t?e=AgBWNf

### GatheringNode
Consults feeds or databases like AbuseIP/Virus total to know if the IP is malicios or not, the same with domains and Hashes. It then updates the appropiated Dataset with the info to enrich future logs.

### IndexingNode
Send logs to index in the database (elasticsearch/SQLite/Postgres...) and queries them when asked.

### RuleNode
Set conditions for logs and fires alerts when the conditions matches.
If a Rule is battletested, then we can tell the SOAR node to do things.
https://github.com/Neo23x0/sigma/tree/master/rules/windows

### AlertNode
Creates alerts and sents them to another SIEM or stores them locally to use the native UI. Uses templates for the alerts.

### SoarNode
Do actions automatically, like blocking IPs, domains...
OpnSense supports blocking IPs with a simple API-REST call, the same for Cortex XDR.
For PaloAlto: https://panos.pan.dev/docs/apis/panos_dag_qs
Work in progress: define a custom trait that can be used with a common component as to simplify design. So we only need to import a library that defines the actions to be done (like an API call) and works in any custom SOAR component.

An idea: Apply multiple simple rules (like DarkTrace) does to calculate the threat score associated with the event. That score is added to the total score of a user in a period of time (Slicing Window). It will be implemented in redis wit a ScoreSet of users-scores in periods of 15 min with each removed after 24 hours by default.

## [Datasets](./src/components/dataset/README.md)

The Datasets are similar to the QRadar reference sets. They store information such as IP, IOC ... and can be populated almost in real time from the information extracted from the logs.

## Internal Events
Each component works like a single entity. It can receive specific commands like STOP_COMPONENT, ISOLATE_IP, LOG_QUERY or specific commands only intended to be used in that component; sent responses to the commands received; process a log; sent notifications (the logging system of uSIEM); receive Dataset updates or fire Alerts. See: https://github.com/u-siem/u-siem-core/blob/main/src/components/common.rs

## Normalziation
To simplify the design and enforce a common way of doing things, it has been designed a object that contains the basic information of a log and that maps the events to normalized fields: https://github.com/u-siem/u-siem-core/blob/main/src/events/mod.rs#L26

```rust
    pub fn set_event(&mut self, event: SiemEvent) {
        match &event {
            SiemEvent::Firewall(fw) => {
                self.add_field(field_dictionary::SOURCE_IP, SiemField::IP(fw.source_ip().clone()));
                self.add_field(field_dictionary::DESTINATION_IP, SiemField::IP(fw.destination_ip().clone()));
                self.add_field(field_dictionary::SOURCE_PORT, SiemField::U32(fw.source_port as u32));
                self.add_field(field_dictionary::DESTINATION_PORT, SiemField::U32(fw.destination_port as u32));
                self.add_field(field_dictionary::EVENT_OUTCOME, SiemField::Text(Cow::Owned(fw.outcome().to_string())));
                self.add_field(field_dictionary::IN_INTERFACE, SiemField::Text(Cow::Owned(fw.in_interface().to_string())));
                self.add_field(field_dictionary::OUT_INTERFACE, SiemField::Text(Cow::Owned(fw.out_interface().to_string())));
                self.add_field(field_dictionary::SOURCE_BYTES, SiemField::U32(fw.out_bytes));
                self.add_field(field_dictionary::DESTINATION_BYTES, SiemField::U32(fw.in_bytes));
                self.add_field(field_dictionary::NETWORK_TRANSPORT, SiemField::Text(Cow::Owned(fw.network_protocol().to_string())));
            },
```
Another normalization aspect that is really interesting is in the categorization of WebProxys: https://github.com/u-siem/u-siem-core/blob/main/src/events/webproxy.rs#L85

Thus all the categories of the different manufacturers will be reduced to the following:
```rust
pub enum WebProxyRuleCategory {
    Abortion,
    MatureContent,
    Alcohol,
    AlternativeSpirituality,
    ArtCulture,
    Auctions,
    AudioVideoClips,
    Trading,
    Economy,
    Charitable,
    ...
```
## TODO List

- [x] Log structure
- [x] Design of components with local horizontal scalability: The kernel must be able to increase the number of threads assigned to a components if there is a congestion in the event queue.
- [x] Prometheus metrics: processed events of each type, errors, users connected, number of querys...
- [x] Event types with fields.
- [x] Custom error system
- [x] Components and kernel interfaces. The components must be registered in the kernel, but the kernel in another instance must be able to know that they exist.
- [x] SIEM inter-Kernel channels, to allow horizontal scaling (Redis channels). Depends on the kernel implementation.
- [x] Datasets that allow Real-Time log enhancement.
- [x] User role design. Inspiration from [PaloAlto Cortex XDR](https://docs.paloaltonetworks.com/cortex/cortex-xdr/cortex-xdr-pro-admin/get-started-with-cortex-xdr-pro/manage-cortex-xdr-roles/administrative-roles.html)
- [ ] Behavour Engine component: Inspiration from Darktrace that uses a lot of small rules to generate a threat score for the event and increse the total score for the user.
- [ ] Active Directory integration for enrich event logs related to users. 
- [ ] Desing a lightweight components that works as agents to get logs from Cloud related sources [Agent Ingestion Components](https://docs.paloaltonetworks.com/cortex/cortex-xdr/cortex-xdr-pro-admin/external-data-ingestion/ingest-authentication-logs-and-data/ingest-authentication-logs-and-data-from-azure-ad.html)
- [x] SIGMA rules support.
- [x] Enforced storage schema for logs. Each source extracts multiple fields with different names. In elastic its not recomended to have more than 1000 fields. Also, it must allow renaming of fields because ECS uses dots in the field names but the majority of databases cant.
- [ ] GDPR included for logs: An analyst does not need to know information about users, such as the websites they visit or the emails they receive. Integrated into the Storage Schema, like the url related fields must be stored encrypted for WebProxy events or the email.subject, email.files or email.source.user.name for Mail events.
- [ ] Mantaince calendar. Used to disable alerting of events related to device configurations, like FortiSIEM does.
- [x] Internacionalization of texts