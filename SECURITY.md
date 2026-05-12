# Security Policy

## Reporting Vulnerabilities

**Do not open public issues for security vulnerabilities.**

Report security issues privately via [GitHub Security Advisories](https://github.com/your-org/ShadowPay/security/advisories/new).

Include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if available)

We will acknowledge receipt within 48 hours and provide updates as we investigate.

## Security Best Practices

### For Users

- Never share your secret keys or `.env` files
- Use hardware wallets or multisig for admin keys
- Verify contract addresses before interacting
- Test on testnet before mainnet deployment

### For Developers

- Run `cargo audit` regularly to check dependencies
- Use parameterized queries and input validation
- Follow the principle of least privilege
- Conduct security reviews before mainnet deployment

## Deployment Checklist

Before deploying to mainnet:

- [ ] All tests passing
- [ ] Security audit completed
- [ ] Testnet deployment verified
- [ ] Admin keys secured (multisig recommended)
- [ ] Token contract address confirmed
- [ ] ZK-proof verification logic audited
- [ ] Dependency audit clean (`cargo audit`)

## Supported Versions

| Version | Status | Support Until |
|---------|--------|---------------|
| 0.1.x   | Active | TBD           |

## Security Contacts

- Security Team: security@your-org.com
- GitHub: [@your-org](https://github.com/your-org)
