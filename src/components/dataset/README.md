# Datasets

The Datasets are similar to the QRadar reference sets. They store information such as IP, IOC ... and can be completed almost in real time from the information collected from the events.
A Dataset is synchronized across multiple SIEM instances using a PUB / SUB pattern. Each instance, when started, loads the entire dataset from the DDBB and subscribes to changes to that particular dataset.
Only delta changes (patch) are sent, but we can also do a full update. 

The [Datasets](https://github.com/u-siem/u-siem-core/blob/a4009b5d759263b4a9b7a6ca1c2a8d36d391bfef/src/components/dataset/mod.rs#L24) are fixed to simplify the development of new modules: 
* GeoIp to map a IP to a country, city...
* IpHost: Association of a IP with a hostname (dynamically populated)
* IpMac: Association of a IP with a MAC address (populated using DHCP logs)
* HostUser: Association of a workstation with the user it belongs to.
* HostMac: If we know the MAC address of each workstation it will help us enchance DHCP logs.
* BlockIp: EDL to be blocked in the firewall 
* IpCloudService: Each Cloud service publish the networks it has daily. This will enchance firewall logs for example.
* IpCloudProvider: To detect connections to Cloud Providers like a IP that belongs to AWS.

It also let us define custom datasets:
* CustomMapText: It has associated a name and a Map of pairs Key-Value(String).
* CustomTextList: Key-Value maps where the value is a list of strings.
...

This will only work effectively when the instance of uSIEM is for a single client. In multi tenant services, will be needed dedicated instances for each tenant, this will simplify the calculation of the cost too and simplify the development of the SIEM in our side.

#### About BlockLists
The blocklists are intended to be used internally by the SIEM not directly by an operator. So, if we want to add an element in a block list, we first need to define in a component the appropiate command, like FILTER_IP and define the behaviour for that command, normally it will be the components that manages the datasets, because it knows how to handle the dataset in the DDBB.

## TODO List

- [x] Block list email sender
- [x] DNS-IP association dataset. If we have DNS logs, we can populate this dataset with the real IPs associated with that domain (If we try to resolve them in another place it may be different).
- [ ] Datasets for WebServers: WebHosted (name of the app hosted in the server); WebAppTechnology (If the APP uses PHP, .NET, JavaScript, Java...) this is useful to detect SCANNERS; SlowWebRequest if some request like "/download/all_logs" are always slow, we can tag them as to not interfere with the rules used to detect BlindSQL injections that uses the sleep command or Web exploits that downloads excessive data.
- [x] Authentication dataset: pairs user/secret only to be used by user facing components: inputs, outputs, comander, database...
- [ ] uSIEM user dataset: Local auth for uSIEM.
- [ ] Dataset with the list of hosts that send logs and the last time they do it.