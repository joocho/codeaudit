# The Complete Guide To Unlawful Crypto Code Audit Scoping
feedback: tommaso.gagliardoni@kudelskisecurity.com

## Introduction

The goal of this guide is to provide rules and guidelines on how to scope code audit requests from customers.

Glossary:
 - MD = "man-day" = 8 hours of work
 - LoC = "lines of code"
 - SoW = "Statement of Work" (contract with the client)
 - PM = "Project Manager"
 - PSM = "Pre-Sales Manager"
 - SM = "Sales Manager"
 
## Basics

### Philosophy: 

Client is requesting Kudelski to audit their code and to write a report. Why? There is usually two possible reasons:

1) Client is concerned about the security of their product. These clients are usually less time-pressed and will go to great length to fix whatever is found in the audit report.

2) Client is under regulatory/marketing/investor pressure. These clients will be less demanding in terms of quality of the report, but they will often have very aggressive time demands.

Regardless, it is important to remember the following: even if the customer might seem minor, obscure or weird, there is no way to foresee what impact the public release of the audit will have. Good or bad audits can influence dramatically the reputation of Kudelski Security and the Group as a whole. We have cases where customers kept coming to us because of some obscure report we published in 2019. We also still get the occasional shit because of some flaws we missed in 2019.

So, please do your best to make this work, even if you're a middle-manager planning of leaving the company in 6-12 months, thanks.


### Principles:

1) Code auditing is a profession. It's not something you can learn in a week. If you are planning to expand the auditors' team and you are hiring someone junior, or without previous auditing experience, consider a long period of training, 3 months minimum to ramp-up at a barely-functional level, where the new hire can "shadow" a more experienced team, but expect 6 months or more.

2) There is automated code audit (with tools such as semgrep, fuzzers, etc) and manual audits (done by a human). The latter is what clients usually want, it's more expensive and niche. It doesn't mean that you should not use automated tools during an audit, but in general *every single line* of the code must be read by human eyes.

3) The more complex the code, the more time and skills the audit requires.

4) In particular, you must treat an audit of *cryptographic* code as fundamentally different than any other security code audit. This is a niche service (few reliable competitors) that bills up to 3-4k USD *per man-day*, while generic code audit bills at much lower rates, comparable to services like, e.g., penetration testing.

5) The larger the codebase (higher number of LoCs), the more time the audit takes.

6) Part of the necessary time can be reduced by increasing the number of auditors (people) working on the project, most part cannot.

7) Increasing the size of the team of auditors has significant returns in terms of accuracy (higher chances of detecting vulnerabilities) but diminishing returns on completion time.

Finally, the most important principle:

8) No code audit, regardless how extensive, should be seen as a guarantee that all vulnerabilities have been found and addressed. So, don't feel overpressed if you miss something.

And its corollary:

9) However, missing a vulnerability in general makes you and the company look bad. Are you familiar with the concept of Seppuku?


### Rules:

1) Never do an audit alone. Always at least two auditors working in tandem, more than two if not very expert or if the project is large.

2) Do not double-book people on two audits at a time. The context-switching and loss of focus is *deadly*. 1 person = 1 project at a time.

3) Code-auditing is somewhat draining. Consider adding buffer time between engagements, also because there might be delays that drag the previous project's completion, and you don't want to have audits back-to-back, or worse, break rule 2) above.

4) At some point there is a phase where you will send a draft report to the client, and you will be waiting for their feedback before reviewing and finalizing the report. Do not give unlimited time between these draft and review phases, make the client commit *by contract* to a maximum amount of time (usually one week) after which the draft report has to be consider final and the project concluded. Otherwise, you risk the client dragging their feet forever and messing up all your calendar (yep, been there, done that). Take that week as a break to do your other homework.

5) *Never commit to an audit without first seeing the code, ever.* Sometimes this requires an NDA in place before even scoping the work, do it if requested. Also, when starting the work, make sure the code matches what you scoped (more on this later).

6) Do not commit to a timeline if the client does not push for it. In any case, never commit to a timeline without first completing the scoping phase and having the SoW ready for signing. Clarify on the SoW that the given date is indicative and subject to changes depending on the client signing the SoW before others close that opportunity window (first sign = first serve).

7) Include a PM. Your auditors will be already very busy deciphering that messy code, don't burden them with timekeeping and emails. You should really, REALLY include a PM.

Finally, the most important rule:

8) Learn to say No. Some audit requests just do not make any sense, or they are not strategic enough, or they are too risky from a reputational and logistical point of view. Unless you are literally starving for money: Just. Say. No.


## What We Do

A "crypto code audit" can actually mean any of these three things:

1) The most straightforward scenario is a "raw" code review. The customer has written all the code from scratch, with or without external dependencies, and they are asking us to find vulnerabilities in what they wrote. However, documentation might be scarce or missing, the code might implement a novel, homebrewn or obscure scheme, and in general it might not be clear what the code is supposed to do. That's fine: what we are looking for here is "just" common programming pitfalls and naive crypto things, such as mishandled exceptions, buffer overflows, encryption in ECB mode, missing authentication, bad randomness, etc.

2) Another very common scenario is "protocol specs matching". In this case there is some specs document available, such as standard guidelines or a whitepaper, but most often just a shitty academic paper, and the client wants to make sure that the code actually implements the specs. This is usually complex, as it requires both checking the docs and the code. Also, sometimes the code MUST implement details that are not specified in the docs (typical cases: the docs might not specify how to hash concatenated strings, or it might not include a unique session id in some protocol round whereas the code does, etc). Understanding what is necessary and what is not is an arcane art that is learnt over time.

3) The most abstract possibility is "architecture review". In this case, it is not even necessary to have a code at all. What is happening here is that the client wants to build a detailed specs document starting from a less formal "idea paper" (usually a whitepaper or academic paper). The client wants first of all to make sure that the idea paper makes sense, so checking if there is obvious flaws in the crypto construction, infrastructure implementation, etc. Then, make sure that the docs are a good formalization of the idea. The final goal would be to ship the docs to the client's engineering team for implementing the scheme.

Most usual case according to our experience is the client requesting 1) + 2). From a pricing point of view, 2) should be easier but usually takes more time, follows 1), and finally 3) which is usually fastest but requires extremely specialized skills (so usually bills higher per MD). Practically, it is safe to treat all these activities at the same rate.

The client might have special requests that we are able to cover:
 - Zeroization, meaning: make sure that secret values are safely erased and not left in memory after use. This can only be done reliably for a bunch of programming languages (C, Rust, ASM, etc) and checking that it is done correctly is much more difficult that one could expect. For Rust, in particular, we would usually need to pull in some Rust expert to check.
 - Timing side-channels (constant time), meaning: make sure that secret-dependent operations (such as signing or decrypting) run in time not dependent on the secret. This is also very tricky.
 - Auditing dependencies: This usually does not make a lot of sense, but if the client requires it we can usually do that.
 - Fuzzying: it depends, some members of the team have previous experience with that, it's done through automated tooling anyway, but it might not always make sense.


## What We Do Not Do

Some special requests we are usually not able to accommodate. That doesn't mean we, as KS, cannot do it, but will need someone else in the company to do these.

 - Validating new schemes. For example, checking or coming up with new cryptographic proofs for novel schemes the client has developed. In theory, a bunch of people in the Research team (theory-oriented cryptographers) could be able to do that. However, that's a very risky task, both in terms of time (we have no idea how long it might take), in terms of outcome (most likely we find flaws, and then the customer will keep bothering us with "fixes" forever), and in term of reputation risk. So, we don't do these.
 - Hardware side-channels. Better ask IoT Labs for this.
 - Secure enclaves, such as Intel's SGX. AFAIK we don't have experts in these but other teams for sure do.
 - Smart contracts. Ask Advisory.

## Workflow

This is how, *in ideal circumstances*, an engagement should go.

1. Client reaches out to us, either through a referral, through the website, or through internal contacts. A PSM is assigned.

2. PSM gets in preliminary touch with client, collects info, and evaluates:

    2.1. Is there any obvious blocker that makes the request impossible? I.e., is the client from North Korea? Is the client in the blacklist below? Does the client need the job to be done in one week? If so, don't even bother us, just find a way to say No.

    2.2. Does this look like a standard crypto audit engagement request? If so, proceed to 3.

    2.3. If not, or unsure, pull in either Tommaso, or Nathan, or someone from Advisory in a call with the client to clarify the ask.

3. PSM communicates to the client what we need to proceed further: access to the code to be audited, any relevant documentation we might need, and any other useful context.

    3.1. Is the client fine with that? Then go to 4.

    3.2. Is the client reluctant to share code? Try to accommodate (sign NDA, whatever), just don't even go to 4. until we have the frickin' code.

    3.3. The client only wants "a ballpark estimate" without sharing the code? Then PSM needs to ask to the client at the very minimum the following:
     - what the code does and how (please provide a few paragraphs). 
     - What language and how many lines of code *IN SCOPE* (excluding comments and blank lines).
     - Is this based on public schemes or is it custom/proprietary?
     
    PSM collects these info and sends them to Advisory, who will then be able to provide the "ballpark estimate" using this guide. In any case, this is the best we can do at this point. If the clients really wants to proceed, they need to give us access to the code as from above. Then, proceed to 4.

4. Scoping is performed on the task using this guide, and (if no blockers are identified) PSM builds a SoW draft based on this.

5. The SoW draft *must* be read and greenlighted by the team who will deliver the task.

6. The SoW draft (and pricing) is presented to the client, requesting for feedback, and noting that before finalizing the SoW a crucial information must be provided by the client: The commit number (hash) of the code to be audited. If this is not available, then PSM shall request the customer to acknowledge "freezing" of the code to a supplied archive, and a hash of the provided code will be included in the SoW.

7. A final check is performed to verify that the final frozen code provided by the client does not differ substantially from that which the scoping was based on (yes, we had a customer tricking us by adding much more code at this stage).

8. Final SoW is presented to the client. As soon as the SoW is signed, the following happens:
    - The auditors team allocates the necessary time on their calendar.
    - A SM and a PM are assigned.
    - Project number for Dynamics is assigned.
    - A kickoff meeting is scheduled by the PM.

9. At the kickoff meeting must be present: representative(s) from the client (possibly including technical people), PM, auditor team. During the kickoff, usual things should happen, most important of which is probably to establish a common communication channel (Slack, Telegram, etc) where technical questions can be asked and answered quickly.

10. The code audit starts. Auditors must inspect the code, often two or more of them inspecting the same parts of the code rather than just "splitting the code". Findings and observations are collected, and a draft report starts taking shape.

11. When the draft report is concluded, it is reviewed internally for Q&A and then it is delivered to the customer through the PM. At this point the time starts ticking and the customer has a previously agreed amount of time (usually 7 days but could be more) to read the report, and provide black on white observations, feedback, edit requests, and patched code.

12. Then, the auditors collect all this material and amend the report into a final version (this usually shouldn't take more than a week) before delivering it to the client and concluding the engagement. This is also the moment to ask the client whether they also want a version of the report for public release or if they'd rather keep it confidential for themselves.

## Scoping Time

How do you scope time? There is four components to scope the required time for an audit:
1. main audit time
2. ramp-up
3. review
4. extras

As a first approximation, you just compute these 4 components separately and add them up together.

### Main Audit Time

Take the number of LoC in scope (excluding blank lines, but you can and should add one-half or one-third LoC for every line of comments, in order to account for very comment-heavy code).

Divide this number by 1000 for "standard" languages (Go, Python, etc) or by 600 for "difficult" languages (C, Rust, Assembly, personally I include Java and Javascript here as well, but this can vary on the auditors' experience and preferences). If the auditors are junior, or not familiar with the language, use something like 700 and 400 instead. As they mature experience, you can use 850 and 500 respectively, until they are skilled and they can use 1000 and 600 as default.

The number you obtain is the number of "rough man-days". In theory, this should be multiplied by the number of auditors, because in ideal circumstances you want all auditors to parse all the code rather than splitting the task among themselves. However, from our experience this is not the case, as experienced auditors can manage to split the code and only check with each other if there's doubts.

So, this is the first component IN MAN-DAYS.

### Ramp-Up Time

This is the time necessary to digest the required documentation. It varies wildly depending on the complexity of the documentation. It can go anywhere from 1 day (user manual of a simple tool the client wants to audit) to a week or more (novel multi-party computation protocol), PER DOCUMENT, PER AUDITOR. If no previous experience, double this time. If already familiar but not expert, halve it. Ramp-up time does not split among auditors! It's given in term of DAYS, not man-days!

Example 1: Client wants to audit a GG18 library. GG18 is a quite complex MPC scheme, but the auditors have already done this, albeit long ago, so assume 2-3 days ramp-up.

Example 2: Client wants to audit a CGGMP21 library, extremely complex stuff. Auditors are somewhat familiar with MPC schemes but have never seen CGGMP21 before. Consider 5-6 days.

Example 3: Client wants to audit a library for compiling zk-SNARK circuits using the latest state of the art. Auditors are familiar with Groth16 only. This will require them to ramp-up over a long line of academic papers of incremental complexity. Consider 3 weeks ramp-up time at minimum.

Ranking of common MPC schemes (from easier to hardest) in terms of ramp-up time (not in terms of ease of audit itself): BLS, FROST, GG18, DKLS, CGGMP*.

It must be stressed that ramp-up time is unavoidable. You cannot "get cheap" on it. On the other hand, it might often make sense to "eat up" the ramp-up time, meaning you give this time to the auditors before starting the engagement, but you do not bill it to the customer, basically you give it for free. This makes sense in case the ramp-up is necessary for the auditors to acquire skills that will certainly be useful in the future for other projects and you want to invest on the team. For example, if the auditors need to ramp-up on GG18 (which is the basis of 99% of code audits we do), it doesn't make sense to bill it to the customer: This is skill that, in a certain sense, the customer expects the auditors to already have. Be sure that other competitors will not bill it to the customer either, so the customer knows better and will shop around. The end result is that your auditors will not get the chance of picking up GG18 experience, and you will find yourself in the same situation in the future. Just eat that ramp-up time once and be done with it!

Remember that this second time component must be MULTIPLIED by the number of auditors in order to get a MD number.

### Review

This is the time necessary to read, inspect, absorb all customer's feedback after the first draft report and create the final report. In optimal circumstances, it's something like 4-6 MD. But if the project is big, or if a ton of findings have popped up during the audit, you might want to increase it. Regardless of the number of MD you have here, just to be safe consider at least one working week (5 days) of calendar time for the auditors'team, so to add a bit of buffer time.

### Extras

The main extra component is creating the initial draft report. Assume 5-6 MD for this, more if the project is very large.

Remember to add PM time, and maybe 1 MD to do initial housekeeping (setting up local repository, compile/troubleshoot and run automatic tools, etc.).


## Good Customers

The following clients earned a special mention for being particularly good to work with:
 - Algorand
 - Amis
 - DFNS
 - ING
 - Protocol Labs
 
## Bad Customers

The following clients earned a special mention for being particularly bad to work with:
 - BitGo
 - Coinbase
 - SafeHeron (BLACKLISTED)
 - Thorchain
