# uSIEM

[![crates.io](https://img.shields.io/crates/v/u-siem.svg?style=for-the-badge&logo=rust)](https://crates.io/crates/u-siem) [![documentation](https://img.shields.io/badge/read%20the-docs-9cf.svg?style=for-the-badge&logo=docs.rs)](https://docs.rs/u-siem) [![MIT License](https://img.shields.io/crates/l/u-siem?style=for-the-badge)](https://github.com/u-siem/u-siem-core/blob/main/LICENSE) [![Rust](https://img.shields.io/github/actions/workflow/status/u-siem/u-siem-core/rust.yml?style=for-the-badge)](https://github.com/u-siem/u-siem-core/workflows/Rust/badge.svg?branch=main&style=for-the-badge)

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

## Components
Each node that receives messages is called a component, and they are managed by the Kernel component. uSIEM does not impose restrictions over how the kernel is designed/developed, but it does for the rest of the components. 
A component is best defined as a piece of code that listens for events, runs in its own thread and it managed by the Kernel. For a component to be managed by the Kernel it must implement the SiemComponent or the SimplifiedComponent trait.
This last one lets us design components focused on functionality, the simplified components are later wrapped inside a SiemComponent like the [SimpleComponent](./src/components/simplified.rs).
```rust
pub trait SiemComponent: Send {
    fn name(&self) -> &'static str {
        "SiemComponent"
    }
    /// Get the channel to this component
    fn local_channel(&self) -> Sender<SiemMessage>;
    /// Sets the channel used to receive/send logs. It's the kernel who sets the channel
    fn set_log_channel(&mut self, sender: Sender<SiemLog>, receiver: Receiver<SiemLog>);

    /// Execute the logic of this component in an infinite loop. Must be stopped using Commands sent using the channel.
    fn run(&mut self) -> SiemResult<()>;

    /// Allow to store information about this component like the state or configurations.
    fn set_storage(&mut self, conn: Box<dyn SiemComponentStateStorage>);

    /// Capabilities and actions that can be performed by this component
    fn capabilities(&self) -> SiemComponentCapabilities;

    /// Allows the Kernel to duplicate this component
    fn duplicate(&self) -> Box<dyn SiemComponent>;

    /// Initialize the component with the datasets before executing run
    fn set_datasets(&mut self, datasets: DatasetHolder);
}
```

The simplified version makes responding to external events a simple task thanks to the on_xxxx methods.
```rust
/// Simplified SIEM component desing. All returned errors are logged to debug except the tick() function that ends the execution
#[allow(unused_variables)]
pub trait SimplifiedComponent : Send {
    /// Allow to store information about this component like the state or configurations.
    fn set_storage(&mut self, conn: Box<dyn SiemComponentStateStorage>) {}

    /// Capabilities and actions that can be performed by this component
    fn capabilities(&self) -> SiemComponentCapabilities;

    /// Allows the Kernel to duplicate this component
    fn duplicate(&self) -> Box<dyn SimplifiedComponent>;

    /// Initialize the component with the datasets before executing run
    fn set_datasets(&mut self, datasets: DatasetHolder) {}

    /// Executed when the component receives a command to execute
    fn on_command(&mut self, header : SiemCommandHeader, action : SiemCommandCall) -> SiemResult<()> {
        Ok(())
    }
    fn on_response(&mut self, header : SiemCommandHeader, action : SiemCommandResponse) -> SiemResult<()> {
        Ok(())
    }
    /// Executed when the component receives a log. Return Ok(None) to filter and remove the log
    fn on_log(&mut self, log : SiemLog) -> SiemResult<Option<SiemLog>> {
        Ok(Some(log))
    }
    /// Called when the component updates i
    fn on_dataset(&mut self, dataset : SiemDataset) -> SiemResult<()> {
        Ok(())
    }
    fn on_alert(&mut self, alert : SiemAlert) -> SiemResult<()> {
        Ok(())
    }
    /// The component received a task to execute
    fn on_task(&mut self, header : SiemCommandHeader, task : SiemTask)-> SiemResult<()> {
        Ok(())
    }
    fn on_task_result(&mut self, header : SiemCommandHeader, result : SiemTaskResult)-> SiemResult<()> {
        Ok(())
    }
    /// Errors are roported as debug
    fn on_notification(&mut self, notifiation : Notification)-> SiemResult<()> {
        Ok(())
    }
    /// Perform actions. Return the number of millisecons to call the tick function again, or None for not (Default). If ab error is returned the component will end its execution.
    fn tick(&mut self) -> SiemResult<Option<u64>> {
        Ok(None)
    }
}
```

```rust
#[derive(Clone)]
struct BasicComponent {}
impl BasicComponent {
    pub fn new() -> Self {
        Self {}
    }
}
impl SimplifiedComponent for BasicComponent {
    fn on_log(&mut self, mut log : SiemLog) -> SiemResult<Option<SiemLog>> {
        log.add_field("PROCESSED", "PROCESSED".into());
        Ok(Some(log))
    }

    fn capabilities(&self) -> SiemComponentCapabilities {
        SiemComponentCapabilities::new(
            LogString::Borrowed("SimpleComponent"),
            LogString::Borrowed("SimpleComponent"),
            LogString::Borrowed(""),
            vec![],
            vec![],
            vec![],
            vec![],
        )
    }

    fn duplicate(&self) -> Box<dyn SimplifiedComponent> {
        Box::new(self.clone())
    }
}
```

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
- [x] User login and third party integration