```
cmd/connectivityd
 ├─ connectivity-config
 ├─ connectivity-events
 ├─ connectivity-app
 ├─ connectivity-backend-nm
 ├─ connectivity-backend-bluez
 ├─ connectivity-dbus
 ├─ connectivity-http
 └─ connectivity-grpc

connectivity-app
 ├─ connectivity-backend
 ├─ connectivity-domain
 └─ connectivity-events

connectivity-backend
 └─ connectivity-domain

connectivity-backend-nm
 ├─ connectivity-backend
 └─ connectivity-domain

connectivity-backend-bluez
 ├─ connectivity-backend
 └─ connectivity-domain

connectivity-dbus
 ├─ connectivity-app
 ├─ connectivity-config
 └─ connectivity-domain

connectivity-http
 └─ connectivity-app

connectivity-grpc
 └─ connectivity-app

connectivity-events
 └─ connectivity-domain
```