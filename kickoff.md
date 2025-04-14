# Kick-off meeting

Here's a list of the few questions that one might ask to inquire about a project (based on experience).

## Project development

- What is the state of the project?
- Is there any documentation/specs available?
- What is the end goal of the project; deployed once, updated?
- Will the project be maintained? (How long?)
- How many teams/developpers?
- How much time was invested?
- What developpment management was used (e.g, Agile)?
- Are there any unit or integration tests?
- Is your project based on an existing implementation ? open-source implementation ?

## Security review

- Is it the first time a security review is performed (if not, can we have access to the previous reports)?
- What perspective of threats?
- Are there any expected vulnerabilities?
- What mostly require to be evaluated?

## Organization

The project manager should already cover these, but make sure that these questions were asked.

- How can we have access to the codebase?
- How many man-days are expected for the project? (usually, 1 MD/1,000 LOC)
- Do we plan to have regular meetings to present progress? (weekly/bi-weekly are preferred)
- How can we contact you in case we have to escalate a critical finding? (Wire is preferred)
- How can we contact you in case we just have quick questions? (Wire is preferred)
- What feedback do you expect? (Excel sheet is preferred)
- Are you planning to publish the report ?

## Notes

*Insist* that you _NEED_ the code on a local machine to work and **categorically refuse to work in _any_ other circumstances**.

We normally sell 1 MD per 1,000 LOC, but the ramp-up does not scale very well with this rule, so don't hesitate warn the client that the schedule is only *tentative* due to the ramp-up (especially with large codebases of >10K LOC).

As a result of the previous point, make sure to track the time spent and the LoC covered *every* day and raise awareness at the end of the week in case you deviate too much from the plan.

Sometimes, the LoC number is not a good metric of progress as you need to come back several times to the same lines to understand what is going on (especially true with Java code bases). In this case, provide a *clear* plan (ideally with bullet points) of all the things that will be covered and their approximated time load.

# Code review methods

In case the client is interested, we offer different ways of reviewing code.

### Single report

A single report is being written, findings are reported as discovered, feedback is provided mainly during meetings.

**Advantages**
- Final report is continuously written, so the client can comment on it and ask further questions as time goes on.

**Disadvantages**
- Inconvenient to keep track of vulnerabilities (usually send various versions of the report, which can be a mess).

### Excel sheet with findings

An Excel document regroups all the findings uncovered, separated in different sheets.

**Advantages**
- Convenient to keepi track of vulnerabilities and progress.
- The client has access to early reviews of the code and can already comment, accept, reject findings.

**Disadvantages**
- The client needs to be technical (usually written in a computer-security lingo).
- If a final report is asked, this requires a compilation of the findings in a more digestible format in the end.

### Fine-tooth comb

Aymeric's method with the full Excel document with all the different parts of the codebase.

**Advantages**
- Most detailed feedback covering rigid coding practices which not only covers current state of the project, but also prevent future vulnerabilities during maintenance.
- Good things are reported, which can make the client happy.
- Findings have a large scale: Good, Comment, Typography, Style, Warning, Pitfall, Wrong, Bug, Security.

**Disadvantages**
- Time-consuming process, as it requires filling an entire Excel document for all packages, files, classes, functions, attributes, ...
- Does not scale with large code bases.
- Most of the time, the client doesn't care about these.
