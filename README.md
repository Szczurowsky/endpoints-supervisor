# endpoints-supervisor
Rust desktop app which check server status and let you know if any of monitored server downs
## Windows 
### Installation
- Download the .exe
- Start app (default in background)
- Close app by taskmanager process `endpoint-manager`
- Go to %appdata% and find folder called szurowsky
- Config app by changing value of all 3 configs file
###Debugging
- Run app with command `endpoint-supervisior.exe > logs.txt`
- Read logs from file
## Linux
### Installation
- Download app for Linux
- Allow app for being executed - `chmod +x filename`
- Run app for a while and close it
- Edit configs in `/home/user/.config/endpoint-manager/`
- Now you can run app by daemonize it or just in console. Good idea would be start app as service
### Service for app
```
[Unit]
Description=Endpoint-supervisor service
[Service]
ExecStart=path_to_file
StandardError=file:path_to_file
[Install]
WantedBy=multi-user.target
```
## Thanks
Special thanks for KernelErr0r for substantive help and good will