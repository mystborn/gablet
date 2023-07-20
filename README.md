# Gablet

This is the parent repository for all of the Gablet services. Gablet is a work-in-progress manga/manwha/manhua/online comic publishing platform. The biggest feature in comparison to its peers will be to provide an easy way to add translations to projects in a way that will allow the author and translators to receive recognition and pay for their efforts.

## Installation

These instructions are based off of running Ubuntu through WSL, but it should be similar on native linux machines.

TODO: Add docker containers

1. Install the following packages
    1. pkg-config
    2. build-essentials
    3. postgresql
    4. libpq-dev
    5. libsqlite3-dev
2. Install rust nightly
3. install NVM
4. Using NVM, install Node 18
5. In Postgres, create a `gablet` and `gablet_auth` table
6. Follow the setup steps in `gablet_api`, `gablet_auth_api`, and `gablet_view`