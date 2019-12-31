# gpio-http-server

Provides a simple HTTP interface to GPIO pins

## API

* `GET /gpio/{pin}` - gets the current value of the pin
* `POST /gpio/{pin}/on` - sets the pin to 1
* `POST /gpio/{pin}/off` - sets the pin to 0

