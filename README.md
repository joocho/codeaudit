# Code Audit
Project to gather all code audit good practices.

## Scoping

See [Scoping guide](scoping.md)

## Good practices
- Zeroize sensible data when not used.
- SECURITY.md file in place with up to date information: https://docs.github.com/en/code-security/getting-started/adding-a-security-policy-to-your-repository

## Language specific
- [Go](golang/golang.md)
- [Rust](rust/rust.md)

## Cryptography specific
- [General methodology](crypto/methodology.md)
- [Nonces and Ephemeral Keys](crypto/nonces_and_ephemeral_keys.md)
- [Hash](crypto/hash.md)
- [MAC](crypto/MAC.md)
- [ECDSA](crypto/ecdsa.md)
- [Diffie Hellman](crypto/Diffie Hellman.md)
- [RSA](crypto/RSA.md)

## Test, code coverage and fuzzing
Verify the code coverage and tests are implemented properly and run frequently. After a code audit, tests verifying regressions should be implement on all the findings. Fuzz testing should be enabled.

## Coding guideline
- https://www.owasp.org/index.php/OWASP_Secure_Coding_Practices_-_Quick_Reference_Guide
- https://github.com/veorq/cryptocoding
- https://owasp.org/www-pdf-archive/OWASP_Code_Review_Guide_v2.pdf

## Vulnerabilities documentation
Document the findings using *Common Weaknesses Enumeration* (CWE): http://cwe.mitre.org/index.html (e.g., a *Stack-based Buffer Overflow* can be referenced with this link: https://cwe.mitre.org/data/definitions/121.html).

## Checklists
**Generic**
- https://arch.simplicable.com/arch/new/secure-code-review-checklist
- https://www.evoketechnologies.com/blog/code-review-checklist-perform-effective-code-reviews/
- https://www.synopsys.com/blogs/software-security/code-review-checklist/
- https://courses.cs.washington.edu/courses/cse403/10wi/lectures/security_audit_checklist.pdf
- https://jesseheines.com/~heines/91.462/Resources/CodeReviewChecklists/StopMoreBugsWithOurCodeReviewChecklist_FogCreekBlog_2015-03-23.pdf

**Java**
- https://dzone.com/articles/java-code-review-checklist

**Rust**
- https://research.kudelskisecurity.com/2019/02/07/auditing-rust-crypto-the-first-hours/

## Tools

In the future, it would be great if we systematically ran a static analysis tool (e.g., SonarQube or Atlassian's Crucible) over the codebase to obtain a baseline for the code audit.

**Generic**
- https://www.sonarqube.org/
- https://www.atlassian.com/software/crucible
- https://sonarcloud.io/
- https://sourceforge.net/projects/agnitiotool/
- https://www.checkmarx.com/
- https://www.synopsys.com/software-integrity/security-testing/static-analysis-sast.html
- https://github.com/ZupIT/horusec
- https://github.com/insidersec/insider
- https://github.com/MobSF/Mobile-Security-Framework-MobSF
- https://www.shiftleft.io/nextgen-static-analysis/
- https://www.parasoft.com/
- https://www.veracode.com/

*Grep-based*
- https://github.com/wireghoul/graudit/
- https://github.com/returntocorp/semgrep
- https://sourceforge.net/projects/visualcodegrepp/

**Java**
- https://www.grammatech.com/
- https://discotek.ca/deepdive.xhtml
- https://find-sec-bugs.github.io/
- https://spotbugs.github.io/
- https://discotek.ca/sinktank.xhtml
- https://wiki.owasp.org/index.php/Category:OWASP_Code_Crawler
- https://wiki.owasp.org/index.php/OWASP_LAPSE_Project
- https://wiki.owasp.org/index.php/Category:OWASP_Orizon_Project
- https://pmd.github.io/ (**NOTE**: only check code *quality*)

**C/C++**
- http://cppcheck.sourceforge.net/
- https://www.grammatech.com/codesonar-cc
- http://splint.org/
- http://www.dwheeler.com/flawfinder/
- https://www.perforce.com/products/klocwork
- https://docs.microsoft.com/en-us/previous-versions/windows/embedded/ms933794%28v=msdn.10%29
- https://pvs-studio.com/en/pvs-studio/
- https://www.mathworks.com/products/polyspace.html
- https://fbinfer.com/
- https://clang-analyzer.llvm.org/

**C#/.NET**
- https://security-code-scan.github.io/
- https://docs.microsoft.com/en-us/previous-versions/dotnet/netframework-3.0/bb429476%28v=vs.80%29

**Python**
- https://github.com/PyCQA/bandit
- https://pyre-check.org/

**Ruby on Rails**
- https://brakemanscanner.org/
- https://rubygems.org/gems/dawnscanner

**NOTE**: https://huskyci.opensource.globo.com/ is a tool that combines Bandit and Safety (Python), Brakeman (Ruby), Npm Audit and Yarn Audit (JavaScript), Gose (Golang), and SpotBugs and Find Sec Bugs (Java). We could create a similar thing to combine multiple tools.

## Books

**C**
- CERT Secure C Coding Standard, https://www.amazon.com/gp/product/B004Y4UTB8/ref=dbs_a_def_rwt_bibl_vppi_i5
- Secure Coding in C and C++, https://www.amazon.com/gp/product/B00C0OBZI0/ref=dbs_a_def_rwt_bibl_vppi_i1

## Thesis

- An exploration of static analysis used on C cryptography libraries, https://www.cs.ru.nl/bachelors-theses/2020/Auke_Zeilstra___4751701___An_exploration_of_static_analysis_used_on_C_cryptography_libraries.pdf

## FAQ
- What should I ask the client during the kick-off meeting? [See: kickoff.md](kickoff.md)
