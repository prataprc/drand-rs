[![crates.io](https://img.shields.io/crates/v/drand.svg)](https://crates.io/crates/drand)
[![Documentation](https://docs.rs/drand/badge.svg)](https://docs.rs/drand)


# Distributed randomness beacon.

The main challenge in generating good randomness is that:

* No party involved in the randomness generation process should
  be able to predict or bias the final output.
* A drand network is not controlled by anyone of its members.
* There is no single point of failure, and none of the drand
  server operators can bias the randomness generated by the network.

This a client library in rust to access a drand-group

* Generate distributed randomness.
* Verifiable.
* Unpredictable.
* Unbiased random numbers.
* Available.

**Refer: http://drand.love**

A simplified sequence of steps involved in generating randomness,
on the server side:

* Network of nodes participate in generating random number.
* Agree on a threshold parameter.
* Each node generate random number.
* Broadcast partial signature.
* The final signature is a regular Boneh–Lynn–Shacham signature.
* Final signature can be verified against by the rest of the network.
* Randomness is simply the hash of that signature

**Notes:**

* Each node first generates a long-term public/private key pair.
* All of the public keys are written to a group file together
  with some further metadata required to operate the beacon.
* The nodes perform a distributed key generation (DKG) protocol
  to create the collective public key and one private key share per
  server.

**Reference:**

https://en.wikipedia.org/wiki/Threshold_cryptosystem
https://en.wikipedia.org/wiki/Secret_sharing
https://en.wikipedia.org/wiki/Verifiable_secret_sharing
https://en.wikipedia.org/wiki/Distributed_key_generation
