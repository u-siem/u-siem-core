# u-siem-core
Framework definitions that allow to build a custom SIEM. Code you own SIEM like you code your own web page.

## Motivation
I have seen many times updates in rules that break things or inneficient regular expressions that slow down the entire SIEM. 
With the aproach of uSIEM, all changes to the SIEM are traced and can be reversed using a version control system like GIT.
Also you can have a team designing rules with SIGMA in a different repository and join the SIEM code and the rules using
CI/CD tools like Jenkins that can run a integration test to check if the new changes breaks the system and then deploy
the new version.

In order to mitigate this problems uSIEM takes a different approach, instead of making dynamic changes to the SIEM like new parsers or extractors, you must define a code that does it, if you want a new field, you code it with his own testing suit.

Some benchmarks (single-thread):
| Log Source        | Events/second     |
|-------------------|-------------------|
|Suricata(JSON)     |261780             |
|OpnSense Firewall  |750127             |



## Architecture

```
                                   |---------------|                
                            |----> | GatheringNode |------------------>|
                            |      |---------------|                   |
                            |                                          |
|---------|       |------------------|       |--------------|          |
|InputNode| ----> | EnchancementNode | ----> | IndexingNode |          |
|---------|       |------------------| |     |--------------|          |
                                       |                               |
                                       |   |----------|       |--------------|       |----------|           
                                       --> | RuleNode | ----> | AlertingNode | ----> | SoarNode |
                                       |   |----------|       |--------------|       |----------|
                                       |                             |
                                       |                             |
                                       |   |------------|            |
                                       --->|BehavourNode| ---------->|
                                           |------------| 
```
## Node Types

### InputNode
It ingests logs and process them.
We can support elasticsearch type (Like API-REST) or syslog.

### EnchancemetNode
It adds information about the IP, if its in a blacklist, if its a AmazonWebServices/Azure/GoogleCloud IP, if the IP has never been seen it then it contacts the GatheringNode to find information about that IP. It adds the tag "never_seen_ip" in that cases. It uses datasets to access information in a non-blocking form. See https://1drv.ms/p/s!AvHbai5ZA14wjV9J4rbBlSWyIw0t?e=AgBWNf

### GatheringNode
Consults feeds or databases like AbuseIP/Virus total to know if the IP is malicios or not, the same with domains and Hashes.

### IndexingNode
Send logs to index in the database (elasticsearch) and queries them when asked.

### RuleNode
Set conditions for logs and fires alerts when the conditions matches.
If a Rule is battletested, then we can tell the SOAR node to do things.
https://github.com/Neo23x0/sigma/tree/master/rules/windows

### AlertNode
Creates alerts and sents them to another SIEM or stores them locally to use the native UI. Uses templates for the alerts.

### SoarNode
Do actions automatically, like blocking IPs, domains...
OpnSense supports blocking IPs with a simple API-REST call, the same for Cortex XDR.

### BehavourNode
Apply multiple simple rules (like DarkTrace) does to calculate the threat score associated with the event. That score is added to the total score of a user in a period of time (Slicing Window). It will be implemented in redis wit a ScoreSet of users-scores in periods of 15 min with each removed after 24 hours by default.

## TODO List

- [x] Log structure
- [ ] Event types with fields.
- [ ] Components and kernel interfaces. The components must be registered in the kernel, but the kernel in another instance must be able to know that they exist.
- [ ] SIEM inter-Kernel channels, to allow horizontal scaling (Redis channels).
- [ ] Datasets that allow Real-Time log enhancement.
- [ ] User role design. Inspiration from [PaloAlto Cortex XDR](https://docs.paloaltonetworks.com/cortex/cortex-xdr/cortex-xdr-pro-admin/get-started-with-cortex-xdr-pro/manage-cortex-xdr-roles/administrative-roles.html)
- [ ] Behavour Engine component: Inspiration from Darktrace that uses a lot of small rules to generate a threat score for the event and increse the total score for the user.
- [ ] Active Directory integration for enchance event logs related to users. 
- [ ] Desing a lightweight components that works as agents to get logs from Cloud related sources [Agent Ingestion Components](https://docs.paloaltonetworks.com/cortex/cortex-xdr/cortex-xdr-pro-admin/external-data-ingestion/ingest-authentication-logs-and-data/ingest-authentication-logs-and-data-from-azure-ad.html)
- [ ] SIGMA rule engine with redis as a Working Memmory.
- [ ] Enforced storage schema for logs. Each source extracts multiple fields with different names. In elastic its not recomended to have more than 1000 fields. Also, it must allow renaming of fields because ECS uses dots in the field names but the majority of databases cant.
- [ ] GDPR included for logs: An analyst does not need to know information about users, such as the websites they visit or the emails they receive. Integrated into the Storage Schema, like the url related fields must be stored encrypted for WebProxy events or the email.subject, email.files or email.source.user.name for Mail events.
- [ ] Support for Threat HUnting using Jupyter.
- [ ] Mantaince calendar. Used to disable alerting of events related to device configurations, like FortiSIEM does.