Pulses are just a way to manage data and track when that data changed. It's somewhat equivalent to Signals.
In this crate we have 2 kinds of Pulses, 1 that is Observed, where the dependencies are functions, and the other
simply tells it's 'Owner' what is in need of changes
