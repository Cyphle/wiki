# AWS Signing API requests

* Requests must be signed with credentials (not all)
* SDK, CLI or HTTP requests are signed for you
* Should sign an AWS HTTP request using Signature v4 (SigV4) : means signed with credentials
* Examples
    * HTTP header option in Authorization header
    * Query string option