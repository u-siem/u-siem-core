# Datasets

The Datasets are similar to the QRadar reference sets. They store information such as IP, IOC ... and can be completed almost in real time from the information collected from the events.
A Dataset is synchronized across multiple SIEM instances using a PUB / SUB pattern. Each instance, when started, loads the entire dataset from the DDBB and subscribes to changes to that particular dataset.
Only delta changes are sent, but we can also do a full update. 