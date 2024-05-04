---
title: "Home Network Without ISP Equipment"
date: 2024-04-05T00:00:00+00:00
tags: ["net"]
type: post
showTableOfContents: true
---

I've been mildly dissatisfied with my ISP and my home network setup for quite
a while now, but it so far hasn't pushed past my threshold to merit addressing.
But the situation isn't static. Alternatives have improved, the quality of
service has degraded, the cost has increased, and I was planning on overhauling
the situation for a coming move anyway.

My current setup is what you would call the ISP's Default Offering. You get
an All-in-One box that does Access, Switching, Routing, and Wireless. I do like
the simplicity of it, so long as it ticks all the boxes. And my boxes are as
follows:
* All desktops are to be connected via Ethernet cable, for bandwidth purposes.
* Whatever method is used for the living room, it must suffice for HD streaming.
* No rented equipment. Loaned is fine.

At first, the ISP did not charge for the ISP-provided box. Over time, their
policy changed and they started charging a small monthly amount for the box.
This wouldn't bother me so much so long as I had the option to buy my own
equipment and replace the ISP-provided one. But the Telecom regulator where I
live is, if we're being charitable, weak and ineffectual on these matters.
And so, my options for ditching the ISP's equipment and using my own are up to
the ISP.

The market here (and the bill) is split into Infrastructure Providers and
Internet Service Providers. The former build out the actual physical
infrastructure, last-mile deployments and the like. The latter manage your,
well, Internet Service, supporting connections via one or more Infrastructure
Providers. I think of them sort of like MVNOs.

The access technologies supported by the infrastructure providers determine
the sort of equipment you'd need to own be able to connect. Because the
regulation here is weak, there's no standardization of what the providers *must*
let you do with your own equipment. So some infrastructure providers let you
connect from a set of supported aftermarket products if you wish, but others
don't support that at all.

As Fiber-to-the-Home technologies have been steadily eating into the marketshare
of the older [VDSL](https://en.wikipedia.org/wiki/VDSL) and
[DOCSIS](https://en.wikipedia.org/wiki/DOCSIS) based offerings, I went to
investigate what options I might have as a consumer wanting to transition to
connection via Fiber using privately-owned equipment. Up to some outliers or
legacy installations, I understood that the market's offerings are
overwhelmingly based on [PON](https://en.wikipedia.org/wiki/Passive_optical_network).

One of the infrastructure providers here publishes a list of products it
supports. Instructively, the list contains an "Equipment Type" column, which
subdivides the offerings into three categories: "SFP ONT", "BRIDGE ONT", and
"CPE GW ONT".

So far, my interactions with network equipment never forced me to understand
what [SFP](https://en.wikipedia.org/wiki/Small_Form-factor_Pluggable) is.
Now was the time. At first, I thought this was just "ports for fiber cables",
directly analogous to RJ45 Ethernet connectors. Not so. Apparently, SFP ports
are meant for you to stick a transceiver for doing the low-level communication
in the technology of your choice, be it Ethernet or Fiber. Lobste.rs user `fanf`
refers us to [MAUs](https://en.m.wikipedia.org/wiki/Medium_Attachment_Unit),
considering SFP to be a modern iteration of that.

So, PON fits into this story as a possible technology for you to stick a
corresponding transceiver into an SFP cage. Now we can understand the three
categories above: "SFP ONT" means just a PON transceiver, a "modem on a stick".
Stick a supported one into an SFP port and you're set. "BRIDGE ONT" products are
media converter boxes, functioning as network bridges between a port that has a
hard-wired PON transceiver, and (usually) an RJ45 Ethernet port.
Lastly, "CPE GW ONT" products are All-in-One boxes, which function as routers
and have a hard-wired PON port for WAN.

On to requirements. I want the equipment I purchase to last a long time. The
setting is residential, without a fancy home lab -- just consumer electronics.
As far as access technology is concerned, it seems to me that any small
home/office router with an SFP+ port (10Gbit/s) will suffice. It would be an
advantage for the other ports to be Ethernet, as that's what I will end up
connecting consumer electronics to, and spending extra on SFP Ethernet
transceivers seems wasteful. I would also like the cooling to be passive to
reduce noise and have less moving parts.

For the SFP+ port I will likely need to get a transceiver supported by the
Infrastructure Provider. All the supported ones right now are 1-2.4Gbit GPON,
but there are indications that there will be deployments of XGS-PON (10Gbit)
coming soon, so I wonder when they will publish which XGS-PON transceivers
they support so I could skip buying a GPON transceiver. There's an entire
website devoted to these little devices, which may come in handy for making
a selection: [Hack GPON](https://hack-gpon.org/).

So far, for the Integrated-WiFi alternative, the
[MikroTik RB4011iGS Router](https://mikrotik.com/product/rb4011igs_5hacq2hnd_in)
caught my eye. For the WiFi-via-APs alternative, the
[MikroTik RB5009UPr Router](https://mikrotik.com/product/rb5009upr_s_in)
caught my eye. Both have one SFP+ port, passive cooling, and a generous
amount of Ethernet ports. For the latter, there's one 2.5G Ethernet port, and
the Ethernet ports support PoE-out, which is nice for the APs.

For wireless connections, if the router doesn't directly support WiFi, it might
be nice to connect access points via PoE, simplifying cabling. So in that case
the router would need to support PoE-out while the access point would need to
support PoE-in. I will also prefer picking access points supporting newer WiFi
standards over older ones. I'm yet to settle on anything in particular, but
it looks like MicroTik has many compact and unobtrusive offerings.
