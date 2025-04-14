## Threshhold Scheme

### ECDSA
- Lindell17: Fast Secure Two-Party ECDSA Signing [paper](https://eprint.iacr.org/2017/552)
- GG18: Fast Multiparty Threshold ECDSA with Fast Trustless Setup [paper](https://eprint.iacr.org/2019/114) [video](https://www.youtube.com/watch?v=PdfDZIwuZm0)
- GG20: One Round Threshold ECDSA with Identifiable Abort [paper](https://eprint.iacr.org/2020/540)
- CGGMP20: UC Non-Interactive, Proactive, Threshold ECDSA with Identifiable Aborts [paper](https://eprint.iacr.org/2021/060) [presentation](https://csrc.nist.gov/presentations/2020/mpts2020-3a3)
- GS22: Design and analysis of a distributed ECDSA signing service [paper](https://eprint.iacr.org/2022/506)


### BLS
- BLS [On the Adaptive Security of the Threshold BLS Signature Scheme](https://eprint.iacr.org/2022/534)

### Schnorr
- Frost [Two-Round Threshold Schnorr Signatures with FROST](https://datatracker.ietf.org/doc/draft-irtf-cfrg-frost/)
- [On the (in)security of ROS](https://eprint.iacr.org/2020/945)
- [How to Prove Schnorr Assuming Schnorr: Security of Multi- and Threshold Signatures](https://eprint.iacr.org/2021/1375)
- [Better than advertised security for non-interactive threshold signatures](https://crypto.iacr.org/2022/papers/538806_1_En_18_Chapter_OnlinePDF.pdf)

### RSA
- [Practical threshold signatures](https://www.shoup.net/papers/thsig.pdf)


## Implementation

- [AMIS, alice](https://github.com/getamis/alice/)
- [tss-ecdsa, gg20, boltlabs](https://github.com/boltlabs-inc/tss-ecdsa/)
- List of DKG and TSS implementations [Awesome-tss](https://github.com/ZenGo-X/awesome-tss)
- Binance [tss-lib](https://github.com/bnb-chain/tss-lib)
- ING-bank [Multiparty threshold ECDSA scheme](https://github.com/ing-bank/threshold-signatures)
- ZenGo [multi-party-ecdsa](https://github.com/ZenGo-X/multi-party-ecdsa)
- ZenGo [multi-party-eddsa](https://github.com/ZenGo-X/multi-party-eddsa)
- Coinbase WaaS [waas-sdk-react-native](https://github.com/coinbase/waas-sdk-react-native)
- BitGo TSS [BitGoJS](https://github.com/BitGo/BitGoJS/tree/master/modules/sdk-lib-mpc/src/tss)
- Safeheron [multi-party-ecdsa-cpp](https://github.com/Safeheron/multi-party-ecdsa-cpp)
- Circl [RSA Threshold Signatures](https://github.com/cloudflare/circl/tree/main/tss)


## Known attacks
- [ECDSA: Handle with Care](https://blog.trailofbits.com/2020/06/11/ecdsa-handle-with-care/) by trailofbits
- [Security disclosure for ECDSA and EdDSA threshold signature schemes](https://medium.com/@iofinnet/security-disclosure-for-ecdsa-and-eddsa-threshold-signature-schemes-4e969af7155b)
- [DOS on DKG](https://blog.trailofbits.com/2024/02/20/breaking-the-shared-key-in-threshold-signature-schemes/)
- [Zengo Research](https://zengo.com/research/)
- [Attacking Threshold Wallets](https://eprint.iacr.org/2020/1052)
- [tss-bugs](https://github.com/Zengo-X/tss-bugs)
- [Alpha-Rays: Key Extraction Attacks on Threshold ECDSA Implementations](https://eprint.iacr.org/2021/1621)
- [Practical Key-Extraction Attacks in Leading MPC Wallets](https://eprint.iacr.org/2023/1234)
- [Fiat-Shamir in the Wild](https://eprint.iacr.org/2024/1565)
- [Threshold Signatures Using Ed25519 and Ed448](https://datatracker.ietf.org/doc/html/draft-hallambaker-threshold-sigs-00)

### BitForge by Fireblocks
- [GG18 and GG20 Paillier Key Vulnerability [CVE-2023-33241]: Technical Report](https://www.fireblocks.com/blog/gg18-and-gg20-paillier-key-vulnerability-technical-report) by fireblocks
- [A Note on the Security of GG18](https://info.fireblocks.com/hubfs/A_Note_on_the_Security_of_GG.pdf)
- [BitForge: Fireblocks researchers uncover vulnerabilities in over 15 major wallet providers](https://www.fireblocks.com/blog/bitforge-fireblocks-researchers-uncover-vulnerabilities-in-over-15-major-wallet-providers/?ref=blog.getpara.com)
- [tss-lib leaks secret keys in response to incorrectly constructed Paillier moduli](https://github.com/advisories/GHSA-h24c-6p6p-m3vx)
- [BitForge and TSShock](https://blog.threshold.network/bitforge-and-tsshock/)

### TSShock by verichains
- [TSSHOCK](https://verichains.io/tsshock/?ref=blog.threshold.network) by verichains
- [TSSHOCK: Breaking MPC Wallets and Digital Custodians for $BILLION$ Profit](https://www.youtube.com/watch?v=5mlQb8PEF3A), blackhat 2023

## Talks
- Multiple Bugs in Multi-Party Computation: Breaking Cryptocurrency's Strongest Wallets [Video](https://www.youtube.com/watch?v=0Okqvm4lBQI) [Slides](https://www.aumasson.jp/data/talks/BH20_mpctss.pdf)
- Attacks to deployed threshold signatures [Video](https://www.nist.gov/video/mpts-2020-talk-3a1-attacks-deployed-threshold-signatures) [Slides](https://www.aumasson.jp/data/talks/tss_mpts20.pdf)
- Building MPC Wallets at Coinbase [Video](https://www.youtube.com/watch?v=udkjdLyMUXI)  
- NIST Multi-Party Threshold Cryptography [NIST MPTC Workshop](https://csrc.nist.gov/Projects/threshold-cryptography/events)
- [Threshold Signatures with Private Accountability | Cybersecurity Seminars](https://www.youtube.com/watch?v=CXAJ9KoyBkw)

## Standard 
- [Elliptic Curves for Security, RFC7748](https://datatracker.ietf.org/doc/html/rfc7748)
- [EdDSA, RFC8032](https://datatracker.ietf.org/doc/html/rfc8032#section-8)
- [NIST First Call for Multi-Party Threshold Schemes](https://csrc.nist.gov/pubs/ir/8214/c/ipd)
- [NIST IR 8214B, Notes on Threshold EdDSA/Schnorr Signatures](https://csrc.nist.gov/pubs/ir/8214/b/ipd)

