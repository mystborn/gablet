# Gablet Users API

This repo contains the authentication/user microservice API for Gablet. Includes CRUD operations for users and methods for logging users in/out.

Most endpoints have a web and api version. The former is for the Gablet website and adds authentication information to the HTTP Only headers. The latter is for other platforms (primarily intended for app development).

This repo doesn't actually run the server. Please refer to `gablet_auth_server` for that.

## Setup

1. Follow global installation instructions to set up the Gablet development environment.

2. In `gablet_auth_server`, create a file `config/credentials.toml`

3. Fill in the following fields:

```toml
[postgres]
username = ""
password = ""
db = "gablet_auth"
host = ""
port = 0000

[auth]
access_secret = ""
refresh_secret = ""

[mail]
username = ""
password = ""
host = "smtp.gmail.com"
port = 587
```

4. Run the `gablet_auth_server` project.